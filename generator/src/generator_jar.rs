use crate::dirs::Dirs;
use assertr::prelude::*;
use futures_util::StreamExt;
use reqwest::IntoUrl;
use rootcause::prelude::*;
use semver::Version;
use serde::Deserialize;
use sha1::Digest;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, BufWriter};

/// Version of the OpenAPI Generator CLI to use. Bump this to upgrade the generator.
static OPENAPI_GENERATOR_VERSION: LazyLock<Version> = LazyLock::new(|| {
    Version::parse("7.22.0").expect("OPENAPI_GENERATOR_VERSION must be a valid semver string")
});

/// Maven Central metadata document for `openapi-generator-cli`. Used by the pre-flight check that
/// warns when a newer release than [`OPENAPI_GENERATOR_VERSION`] is available.
const OPENAPI_GENERATOR_MAVEN_METADATA_URL: &str =
    "https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/maven-metadata.xml";

pub async fn ensure_jar(dirs: &Dirs) -> Result<PathBuf, Report> {
    let version = &*OPENAPI_GENERATOR_VERSION;
    let url = format!(
        "https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/{version}/openapi-generator-cli-{version}.jar"
    );
    let sha1_url = format!("{url}.sha1");
    let jar = dirs
        .downloads()
        .join(format!("openapi-generator-cli-{version}.jar"));

    download_jar_if_missing(version, url, sha1_url, jar.as_path()).await?;
    Ok(jar)
}

async fn download_jar_if_missing(
    version: &Version,
    url: impl IntoUrl,
    sha1_url: impl IntoUrl,
    dst: &Path,
) -> Result<(), Report> {
    if !(dst.exists() && dst.is_file()) {
        tracing::info!("Downloading version {version} of the OpenAPI Generator...",);
        let sha1_hash = download_jar(url, dst).await?;
        let expected_sha1_hash = reqwest::get(sha1_url).await?.text().await?;
        assert_that!(sha1_hash).is_equal_to(expected_sha1_hash);
        tracing::info!("Downloading version {version} of the OpenAPI Generator... DONE.",);
    }
    Ok(())
}

async fn download_jar(url: impl IntoUrl, dst: &Path) -> Result<String, Report> {
    let generator_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .append(false)
        .open(&dst)
        .await
        .context_with(|| format!("Failed to open {dst:?} for writing."))?;
    let mut writer = BufWriter::new(generator_file);
    let mut stream = reqwest::get(url)
        .await
        .context("Failed to download OpenAPI Generator")?
        .bytes_stream();

    let mut hasher = sha1::Sha1::new();
    while let Some(bytes) = stream.next().await {
        let bytes = bytes?;
        hasher.update(&bytes);
        writer.write_all(&bytes).await?;
    }
    writer.flush().await?;
    Ok(hex::encode(hasher.finalize()))
}

/// Subset of the Maven Central `maven-metadata.xml` schema. Only the `<versioning>` block is
/// modelled, and within it only the `<latest>` and `<release>` elements (Maven convention:
/// `release` is the latest non-snapshot, `latest` is the absolute newest including snapshots).
/// Everything else in the document is ignored by serde.
#[derive(Debug, Deserialize)]
struct MavenMetadata {
    versioning: MavenVersioning,
}

#[derive(Debug, Deserialize)]
struct MavenVersioning {
    #[serde(default)]
    latest: Option<Version>,
    #[serde(default)]
    release: Option<Version>,
}

impl MavenVersioning {
    fn preferred(&self) -> Option<&Version> {
        self.release.as_ref().or(self.latest.as_ref())
    }
}

/// Fetches the Maven Central metadata for `openapi-generator-cli` and warns if a newer release
/// than the pinned `OPENAPI_GENERATOR_VERSION` is available. Failures are logged at `warn` level
/// and never propagated, since this is purely informational and must not break `just gen` when
/// Maven Central is unreachable.
pub async fn warn_if_newer_release_available() {
    let current = &*OPENAPI_GENERATOR_VERSION;
    match fetch_latest_release().await {
        Ok(latest) => {
            if &latest > current {
                tracing::warn!(
                    "A newer OpenAPI Generator release is available: {latest} (currently pinned: {current}). \
                     See {OPENAPI_GENERATOR_MAVEN_METADATA_URL} and bump OPENAPI_GENERATOR_VERSION in generator/src/generator_jar.rs."
                );
            } else {
                tracing::info!(
                    "OpenAPI Generator {current} is up to date (latest release: {latest})."
                );
            }
        }
        Err(err) => {
            tracing::warn!("Could not check for a newer OpenAPI Generator release: {err:#}");
        }
    }
}

async fn fetch_latest_release() -> Result<Version, Report> {
    let xml = reqwest::get(OPENAPI_GENERATOR_MAVEN_METADATA_URL)
        .await
        .context_with(|| format!("Failed to GET {OPENAPI_GENERATOR_MAVEN_METADATA_URL}"))?
        .error_for_status()?
        .text()
        .await
        .context("Failed to read maven-metadata.xml body")?;
    let metadata: MavenMetadata =
        quick_xml::de::from_str(&xml).context("Failed to deserialize maven-metadata.xml")?;
    metadata
        .versioning
        .preferred()
        .cloned()
        .ok_or_else(|| report!("maven-metadata.xml has neither <release> nor <latest>"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_METADATA: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<metadata>
  <groupId>org.openapitools</groupId>
  <artifactId>openapi-generator-cli</artifactId>
  <versioning>
    <latest>7.22.0</latest>
    <release>7.22.0</release>
    <versions>
      <version>7.12.0</version>
      <version>7.22.0</version>
    </versions>
    <lastUpdated>20260428095039</lastUpdated>
  </versioning>
</metadata>"#;

    #[test]
    fn deserializes_maven_metadata_and_prefers_release() {
        let metadata: MavenMetadata = quick_xml::de::from_str(SAMPLE_METADATA).unwrap();
        assert_that!(metadata.versioning.preferred()).is_equal_to(Some(&Version::new(7, 22, 0)));
    }

    #[test]
    fn preferred_falls_back_to_latest_when_release_absent() {
        let xml = r#"<metadata><versioning><latest>7.22.0</latest></versioning></metadata>"#;
        let metadata: MavenMetadata = quick_xml::de::from_str(xml).unwrap();
        assert_that!(metadata.versioning.preferred()).is_equal_to(Some(&Version::new(7, 22, 0)));
    }

    #[test]
    fn preferred_returns_none_when_both_absent() {
        let xml = r#"<metadata><versioning></versioning></metadata>"#;
        let metadata: MavenMetadata = quick_xml::de::from_str(xml).unwrap();
        assert_that!(metadata.versioning.preferred()).is_none();
    }
}

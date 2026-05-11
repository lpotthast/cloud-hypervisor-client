use crate::dirs::Dirs;
use crate::normalize;
use assertr::prelude::*;
use rootcause::option_ext::OptionExt;
use rootcause::prelude::*;
use std::path::{Path, PathBuf};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use yaml_rust2::YamlLoader;

/// Expected `info.version` of the upstream cloud-hypervisor OpenAPI spec. The generator asserts the
/// downloaded spec matches this and refuses to proceed otherwise. Bump this when consuming a new
/// upstream spec version. Note: the upstream spec has been observed to remain pinned at the same
/// value across substantive changes, so this assertion alone is not sufficient to detect drift.
const CLOUD_HYPERVISOR_OPENAPI_EXPECTED_VERSION: &str = "0.3.0";

/// URL of the upstream cloud-hypervisor OpenAPI spec.
const CLOUD_HYPERVISOR_OPENAPI_URL: &str = "https://raw.githubusercontent.com/cloud-hypervisor/cloud-hypervisor/master/vmm/src/api/openapi/cloud-hypervisor.yaml";

pub async fn download(dirs: &Dirs) -> Result<PathBuf, Report> {
    let spec_path = dirs.downloads().join(format!(
        "cloud-hypervisor_{CLOUD_HYPERVISOR_OPENAPI_EXPECTED_VERSION}.yaml"
    ));
    let normalized_path = dirs.downloads().join(format!(
        "cloud-hypervisor_{CLOUD_HYPERVISOR_OPENAPI_EXPECTED_VERSION}.normalized.yaml"
    ));

    tracing::info!(
        "Downloading version {CLOUD_HYPERVISOR_OPENAPI_EXPECTED_VERSION} of the OpenAPI spec for the cloud-hypervisor REST API..."
    );
    let spec = reqwest::get(CLOUD_HYPERVISOR_OPENAPI_URL)
        .await
        .context_with(|| format!("Failed to download: {CLOUD_HYPERVISOR_OPENAPI_URL}"))?
        .text()
        .await
        .context("Failed to extract test from response")?;
    let parsed = YamlLoader::load_from_str(&spec).context("Failed to parse YAML")?;
    let parsed_version = parsed[0]["info"]["version"]
        .as_str()
        .context("Failed to parse version")?;
    assert_that!(parsed_version).is_equal_to(CLOUD_HYPERVISOR_OPENAPI_EXPECTED_VERSION);

    write_file(&spec_path, &spec).await?;

    tracing::info!("Lifting integer formats into schema types so `typeMappings:` resolves them...");
    let normalized =
        normalize::lift_integer_formats(&spec).context("Failed to normalize integer formats")?;
    write_file(&normalized_path, &normalized).await?;

    crate::postprocess::write_normalization_diff(dirs, &spec_path, &normalized_path).await?;

    Ok(normalized_path)
}

async fn write_file(path: &Path, contents: &str) -> Result<(), Report> {
    OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .append(false)
        .open(path)
        .await
        .context_with(|| format!("Failed to open: {path:?}"))?
        .write_all(contents.as_bytes())
        .await
        .context_with(|| format!("Failed to write: {path:?}"))?;
    Ok(())
}

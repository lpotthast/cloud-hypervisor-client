use crate::dirs::Dirs;
use assertr::prelude::*;
use rootcause::option_ext::OptionExt;
use rootcause::prelude::*;
use std::path::PathBuf;
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

    OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .append(false)
        .open(&spec_path)
        .await
        .context_with(|| format!("Failed to open: {spec_path:?}"))?
        .write_all(spec.as_bytes())
        .await?;

    Ok(spec_path)
}

mod tracing_init;

use anyhow::{anyhow, Context, Result};
use assertr::prelude::*;
use futures_util::StreamExt;
use reqwest::IntoUrl;
use sha1::Digest;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::process::Command;
use yaml_rust2::YamlLoader;

struct Dirs {
    lib_dir: PathBuf,
    workdir: PathBuf,
    downloads: PathBuf,
    templates_dir: PathBuf,
    templates_original_dir: PathBuf,
    output_dir: PathBuf,
}

impl Dirs {
    pub async fn init() -> Result<Self> {
        let workdir = PathBuf::from("workdir");
        let workdir_abs = workdir.canonicalize()?;

        let lib_dir = workdir_abs.parent().unwrap().parent().unwrap().to_owned();
        let downloads = workdir.join("downloads");
        let templates = workdir.join("templates");
        let templates_original = workdir.join("templates_original");
        let output = workdir.join("output");

        fs::create_dir_all(&downloads).await?;
        fs::create_dir_all(&templates).await?;
        fs::create_dir_all(&templates_original).await?;
        fs::create_dir_all(&output).await?;

        // Sanity-check. Repo must be checked out as "cloud-hypervisor-client" to pass this though.
        assert_that(lib_dir.as_path())
            .exists()
            .is_a_directory()
            .has_file_name("cloud-hypervisor-client");

        Ok(Self {
            lib_dir,
            workdir,
            downloads,
            templates_dir: templates,
            templates_original_dir: templates_original,
            output_dir: output,
        })
    }

    pub fn workdir(&self) -> &Path {
        self.workdir.as_path()
    }

    pub fn downloads(&self) -> &Path {
        self.downloads.as_path()
    }

    pub fn templates(&self) -> &Path {
        self.templates_dir.as_path()
    }

    pub fn templates_original(&self) -> &Path {
        self.templates_original_dir.as_path()
    }

    pub fn output(&self) -> &Path {
        self.output_dir.as_path()
    }

    pub fn lib(&self) -> &Path {
        self.lib_dir.as_path()
    }
}

pub async fn clear_directory(dir: &Path) -> Result<()> {
    assert_that(dir).is_a_directory();
    let mut read_dir = fs::read_dir(dir).await?;
    while let Some(entry) = read_dir.next_entry().await? {
        let path = entry.path();
        if path.is_dir() {
            fs::remove_dir_all(path).await?
        } else {
            fs::remove_file(path).await?
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_init::init_subscriber();

    let dirs = Dirs::init().await?;

    let openapi_generator_yaml = dirs.workdir().join("openapi-generator.yaml");
    assert_that(openapi_generator_yaml.as_path())
        .exists()
        .is_a_file();

    let generator_version = "7.9.0";
    let generator_url = format!("https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/{generator_version}/openapi-generator-cli-{generator_version}.jar");
    let generator_sha1_url = format!("https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/{generator_version}/openapi-generator-cli-{generator_version}.jar.sha1");
    let generator_jar = dirs
        .downloads()
        .join(format!("openapi-generator-cli-{generator_version}.jar"));

    let cloud_hypervisor_openapi_expected_version = "0.3.0";
    let cloud_hypervisor_openapi_url = "https://raw.githubusercontent.com/cloud-hypervisor/cloud-hypervisor/master/vmm/src/api/openapi/cloud-hypervisor.yaml";
    let cloud_hypervisor_openapi_spec = dirs.downloads().join(format!(
        "cloud-hypervisor_{cloud_hypervisor_openapi_expected_version}.yaml"
    ));

    tracing::info!("Downloading version {cloud_hypervisor_openapi_expected_version} of the OpenAPI spec for the cloud-hypervisor REST API...");
    let spec = reqwest::get(cloud_hypervisor_openapi_url)
        .await
        .with_context(|| format!("Failed to download: {cloud_hypervisor_openapi_url}"))?
        .text()
        .await
        .context("Failed to extract test from response")?;
    let parsed = YamlLoader::load_from_str(&spec).context("Failed to parse YAML")?;
    let parsed_version = parsed[0]["info"]["version"]
        .as_str()
        .context("Failed to parse version")?;
    assert_that(parsed_version).is_equal_to(cloud_hypervisor_openapi_expected_version);

    OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .append(false)
        .open(&cloud_hypervisor_openapi_spec)
        .await
        .with_context(|| format!("Failed to open: {cloud_hypervisor_openapi_spec:?}"))?
        .write_all(spec.as_bytes())
        .await?;

    download_generator_jar_if_missing(
        generator_version,
        generator_url,
        generator_sha1_url,
        generator_jar.as_path(),
    )
    .await?;

    let java_cmd = find_java().await?;
    tracing::info!("Using Java command: '{java_cmd}'");
    tracing::info!("Using generator jar: '{}'", generator_jar.display());

    tracing::info!("Running OpenAPI Generator template extraction...");
    clear_directory(dirs.templates_original())
        .await
        .with_context(|| {
            format!(
                "Failed to clear {} directory",
                dirs.templates_original().display()
            )
        })?;
    let output = Command::new(&java_cmd)
        .arg("-jar")
        .arg(&generator_jar)
        .arg("author")
        .arg("template")
        .arg("-g")
        .arg("rust")
        .arg("-o")
        .arg(dirs.templates_original())
        .output()
        .await?;
    if !output.status.success() {
        return Err(anyhow!(
            "OpenAPI Generator template extraction failed: {output:?}"
        ));
    }

    tracing::info!("Running OpenAPI Generator code generation...");
    clear_directory(dirs.output())
        .await
        .with_context(|| format!("Failed to clear {} directory", dirs.output().display()))?;
    // see: https://openapi-generator.tech/docs/usage#generate
    let output = Command::new(&java_cmd)
        .arg("-jar")
        .arg("-DapiDocs=false")
        .arg("-DmodelDocs=false")
        .arg(&generator_jar)
        .arg("generate")
        .arg("--input-spec")
        .arg(&cloud_hypervisor_openapi_spec)
        .arg("--generator-name")
        .arg("rust")
        .arg("--config")
        .arg(openapi_generator_yaml)
        .arg("--template-dir")
        .arg(dirs.templates())
        .arg("--output")
        .arg(dirs.output())
        .output()
        .await
        .context("Failed to run OpenAPI generator")?;
    if !output.status.success() {
        return Err(anyhow!(
            "OpenAPI Generator code generation failed: {output:?}"
        ));
    }

    tracing::info!("Cleaning up old generated code. Keeping lib.rs...");
    for dir in &[
        dirs.lib().join("src").join("apis"),
        dirs.lib().join("src").join("models"),
    ] {
        if dir.exists() {
            fs::remove_dir_all(&dir)
                .await
                .with_context(|| format!("Failed to remove {dir:?}"))?;
        }
    }

    tracing::info!("Copying generated sources...");
    fs::rename(
        dirs.output().join("src").join("apis"),
        dirs.lib().join("src").join("apis"),
    )
    .await?;
    fs::rename(
        dirs.output().join("src").join("models"),
        dirs.lib().join("src").join("models"),
    )
    .await?;

    tracing::info!("Formatting generated code...");
    let output = Command::new("cargo")
        .arg("fmt")
        .current_dir(dirs.lib())
        .output()
        .await
        .context("Failed to run `cargo fmt`")?;
    if !output.status.success() {
        return Err(anyhow!("`cargo fmt` failed: {output:?}"));
    }

    tracing::info!("Calculating template diff...");
    let _diff = run_git_diff(&dirs).await?;

    Ok(())
}

async fn run_git_diff(dirs: &Dirs) -> Result<bool> {
    let output = Command::new("git")
        .arg("-c")
        .arg("core.autocrlf=false")
        .arg("-c")
        .arg("core.safecrlf=false")
        .arg("diff")
        .arg("-w")
        .arg("--no-index")
        .arg("--diff-filter=M")
        .arg("--output")
        .arg(dirs.workdir().join("templates.diff"))
        .arg(dirs.templates_original())
        .arg(dirs.templates())
        .output()
        .await
        .context("Failed to run `git diff` on templates")?;

    match output.status.code() {
        Some(0) => {
            tracing::info!("No differences found between templates.");
            Ok(false)
        }
        Some(1) => {
            tracing::info!("Differences found and saved to templates.diff");
            Ok(true)
        }
        Some(code) => Err(anyhow::anyhow!("`git diff` failed with exit code {}", code)),
        None => Err(anyhow::anyhow!("`git diff` terminated by signal")),
    }
}

async fn find_java() -> Result<String> {
    if Command::new("java").arg("-version").output().await.is_ok() {
        Ok("java".to_string())
    } else {
        match std::env::var("JAVA_HOME") {
            Ok(java_home) => {
                let java_path = Path::new(&java_home).join("bin").join("java");
                if java_path.exists() && !java_path.is_dir() {
                    Ok(java_path.to_str().unwrap().to_string())
                } else {
                    Err(anyhow!("Java not found in PATH or JAVA_HOME."))
                }
            }
            Err(_) => Err(anyhow!("Java not found in PATH or JAVA_HOME.")),
        }
    }
}

async fn download_generator_jar_if_missing(
    version: &str,
    url: impl IntoUrl,
    sha1_url: impl IntoUrl,
    dst: &Path,
) -> Result<()> {
    if !(dst.exists() && dst.is_file()) {
        tracing::info!("Downloading version {version} of the OpenAPI Generator...",);
        let sha1_hash = download_generator_jar(url, dst).await?;
        let expected_sha1_hash = reqwest::get(sha1_url).await?.text().await?;
        assert_that(sha1_hash).is_equal_to(expected_sha1_hash);
        tracing::info!("Downloading version {version} of the OpenAPI Generator... DONE.",);
    }
    Ok(())
}

async fn download_generator_jar(url: impl IntoUrl, dst: &Path) -> Result<String> {
    let generator_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .append(false)
        .open(&dst)
        .await
        .with_context(|| format!("Failed to open {dst:?} for writing."))?;
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

use crate::dirs::Dirs;
use rootcause::prelude::*;
use std::path::Path;
use tokio::fs;
use tokio::process::Command;

/// Output filename for the templates diff written into `workdir/`.
const TEMPLATES_DIFF_FILE: &str = "templates.diff";

pub async fn install_sources(dirs: &Dirs) -> Result<(), Report> {
    tracing::info!("Cleaning up old generated code. Keeping lib.rs...");
    for dir in &[
        dirs.lib().join("src").join("apis"),
        dirs.lib().join("src").join("models"),
    ] {
        if dir.exists() {
            fs::remove_dir_all(&dir)
                .await
                .context_with(|| format!("Failed to remove {dir:?}"))?;
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
    Ok(())
}

pub async fn format_lib(dirs: &Dirs) -> Result<(), Report> {
    tracing::info!("Formatting generated code...");
    let output = Command::new("cargo")
        .arg("fmt")
        .current_dir(dirs.lib())
        .output()
        .await
        .context("Failed to run `cargo fmt`")?;
    if !output.status.success() {
        return Err(report!("`cargo fmt` failed: {output:?}"));
    }
    Ok(())
}

pub async fn write_templates_diff(dirs: &Dirs) -> Result<bool, Report> {
    tracing::info!("Calculating template diff...");
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
        .arg(TEMPLATES_DIFF_FILE)
        .arg(
            dirs.templates_original()
                .strip_prefix(dirs.workdir())
                .expect("templates_original is a child of workdir"),
        )
        .arg(
            dirs.templates()
                .strip_prefix(dirs.workdir())
                .expect("templates is a child of workdir"),
        )
        .current_dir(dirs.workdir())
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
        Some(code) => Err(report!("`git diff` failed with exit code {}", code)),
        None => Err(report!("`git diff` terminated by signal")),
    }
}

pub async fn write_normalization_diff(
    dirs: &Dirs,
    raw: &Path,
    normalized: &Path,
) -> Result<bool, Report> {
    tracing::info!("Calculating normalization diff...");
    // Sibling of the two YAML files, with the `.yaml` suffix swapped for `.diff`. So:
    // `cloud-hypervisor_<ver>.normalized.yaml` -> `cloud-hypervisor_<ver>.normalized.diff`.
    let diff_path = normalized.with_extension("diff");
    let output = Command::new("git")
        .arg("-c")
        .arg("core.autocrlf=false")
        .arg("-c")
        .arg("core.safecrlf=false")
        .arg("diff")
        .arg("-w")
        .arg("--no-index")
        .arg("--output")
        .arg(&diff_path)
        .arg(
            raw.strip_prefix(dirs.workdir())
                .expect("raw spec is a child of workdir"),
        )
        .arg(
            normalized
                .strip_prefix(dirs.workdir())
                .expect("normalized spec is a child of workdir"),
        )
        .current_dir(dirs.workdir())
        .output()
        .await
        .context("Failed to run `git diff` on specs")?;

    let diff_display = diff_path
        .strip_prefix(dirs.workdir())
        .unwrap_or(&diff_path)
        .display();
    match output.status.code() {
        Some(0) => {
            tracing::info!("No differences found between raw and normalized spec.");
            Ok(false)
        }
        Some(1) => {
            tracing::info!("Differences found and saved to {diff_display}");
            Ok(true)
        }
        Some(code) => Err(report!("`git diff` failed with exit code {}", code)),
        None => Err(report!("`git diff` terminated by signal")),
    }
}

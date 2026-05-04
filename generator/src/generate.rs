use crate::dirs::Dirs;
use rootcause::prelude::*;
use std::path::Path;
use tokio::process::Command;

/// Filename of the OpenAPI Generator config inside `workdir/`.
const OPENAPI_GENERATOR_CONFIG_FILE: &str = "openapi-generator.yaml";

pub async fn find_java() -> Result<String, Report> {
    if Command::new("java").arg("-version").output().await.is_ok() {
        Ok("java".to_string())
    } else {
        match std::env::var("JAVA_HOME") {
            Ok(java_home) => {
                let java_path = Path::new(&java_home).join("bin").join("java");
                if java_path.exists() && !java_path.is_dir() {
                    Ok(java_path.to_str().unwrap().to_string())
                } else {
                    Err(report!("Java not found in PATH or JAVA_HOME."))
                }
            }
            Err(_) => Err(report!("Java not found in PATH or JAVA_HOME.")),
        }
    }
}

pub async fn extract_templates(dirs: &Dirs, java: &str, jar: &Path) -> Result<(), Report> {
    tracing::info!("Running OpenAPI Generator template extraction...");
    dirs.clear_templates_original().await?;
    let output = Command::new(java)
        .arg("-jar")
        .arg(jar)
        .arg("author")
        .arg("template")
        .arg("-g")
        .arg("rust")
        .arg("-o")
        .arg(dirs.templates_original())
        .output()
        .await?;
    if !output.status.success() {
        return Err(report!(
            "OpenAPI Generator template extraction failed: {output:?}"
        ));
    }
    Ok(())
}

pub async fn run(dirs: &Dirs, java: &str, jar: &Path, spec: &Path) -> Result<(), Report> {
    tracing::info!("Running OpenAPI Generator code generation...");
    dirs.clear_output().await?;
    let config = dirs.workdir().join(OPENAPI_GENERATOR_CONFIG_FILE);
    // see: https://openapi-generator.tech/docs/usage#generate
    let output = Command::new(java)
        .arg("-jar")
        .arg("-DapiDocs=false")
        .arg("-DmodelDocs=false")
        .arg(jar)
        .arg("generate")
        .arg("--input-spec")
        .arg(spec)
        .arg("--generator-name")
        .arg("rust")
        .arg("--config")
        .arg(config)
        .arg("--template-dir")
        .arg(dirs.templates())
        .arg("--output")
        .arg(dirs.output())
        .output()
        .await
        .context("Failed to run OpenAPI generator")?;
    if !output.status.success() {
        return Err(report!(
            "OpenAPI Generator code generation failed: {output:?}"
        ));
    }
    Ok(())
}

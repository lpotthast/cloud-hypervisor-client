mod dirs;
mod generate;
mod generator_jar;
mod normalize;
mod postprocess;
mod spec;
mod tracing_init;

use crate::dirs::Dirs;
use assertr::prelude::*;
use rootcause::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Report> {
    tracing_init::init_subscriber();

    rootcause::hooks::Hooks::new()
        .report_creation_hook(rootcause_backtrace::BacktraceCollector::new_from_env())
        .report_creation_hook(rootcause_tracing::SpanCollector::new())
        .install()
        .expect("failed to install rootcause hooks");

    let dirs = Dirs::init().await?;

    assert_that!(dirs.workdir().join("openapi-generator.yaml").as_path())
        .exists()
        .is_a_file();

    generator_jar::warn_if_newer_release_available().await;
    let jar = generator_jar::ensure_jar(&dirs).await?;
    let spec = spec::download(&dirs).await?;

    let java = generate::find_java().await?;
    tracing::info!("Using Java command: '{java}'");
    tracing::info!("Using generator jar: '{}'", jar.display());

    generate::extract_templates(&dirs, &java, &jar).await?;
    generate::run(&dirs, &java, &jar, &spec).await?;

    postprocess::install_sources(&dirs).await?;
    postprocess::format_lib(&dirs).await?;
    let _diff = postprocess::write_templates_diff(&dirs).await?;

    Ok(())
}

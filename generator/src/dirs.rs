use assertr::prelude::*;
use rootcause::prelude::*;
use std::path::{Path, PathBuf};
use tokio::fs;

pub struct Dirs {
    lib_dir: PathBuf,
    workdir: PathBuf,
    downloads: PathBuf,
    templates_dir: PathBuf,
    templates_original_dir: PathBuf,
    output_dir: PathBuf,
}

impl Dirs {
    pub async fn init() -> Result<Self, Report> {
        let generator_crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let workdir = generator_crate_dir.join("workdir");
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

    pub async fn clear_templates_original(&self) -> Result<(), Report> {
        clear_directory(self.templates_original())
            .await
            .context_with(|| {
                format!(
                    "Failed to clear {} directory",
                    self.templates_original().display()
                )
            })?;
        Ok(())
    }

    pub async fn clear_output(&self) -> Result<(), Report> {
        clear_directory(self.output())
            .await
            .context_with(|| format!("Failed to clear {} directory", self.output().display()))?;
        Ok(())
    }
}

async fn clear_directory(dir: &Path) -> Result<(), Report> {
    assert_that!(dir).is_a_directory();
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

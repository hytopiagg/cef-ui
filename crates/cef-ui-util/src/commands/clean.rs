use anyhow::Result;
use std::{
    fs::remove_dir_all,
    path::PathBuf,
    process::{Command, Stdio}
};
use tracing::info;

/// Clean the project.
pub struct CleanCommand {
    /// The artifacts directory.
    pub artifacts_dir: PathBuf
}

impl CleanCommand {
    pub fn run(&self) -> Result<()> {
        info!("Cleaning project ..");

        Command::new("cargo")
            .args(&["clean"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()?;

        info!("Removing artifacts dir ..");

        // Remove the artifacts directory.
        if self.artifacts_dir.exists() {
            remove_dir_all(&self.artifacts_dir)?;
        }

        info!("Done!");

        Ok(())
    }
}

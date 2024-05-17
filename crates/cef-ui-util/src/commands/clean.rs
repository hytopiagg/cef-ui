use anyhow::Result;
use std::{
    fs::remove_dir_all,
    process::{Command, Stdio}
};
use tracing::info;

use crate::get_cef_artifacts_dir;

/// Clean the project.
pub struct CleanCommand;

impl CleanCommand {
    pub fn run(&self) -> Result<()> {
        let artifacts_dir = get_cef_artifacts_dir()?;

        info!("Cleaning project ..");

        Command::new("cargo")
            .args(&["clean"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()?;

        info!("Removing artifacts dir ..");

        // Remove the artifacts directory.
        if artifacts_dir.exists() {
            remove_dir_all(&artifacts_dir)?;
        }

        info!("Done!");

        Ok(())
    }
}

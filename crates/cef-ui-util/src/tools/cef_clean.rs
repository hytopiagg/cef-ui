use anyhow::Result;
use std::{
    fs::remove_dir_all,
    path::Path,
    process::{Command, Stdio}
};
use tracing::{info, level_filters::LevelFilter, subscriber::set_global_default, Level};
use tracing_log::LogTracer;
use tracing_subscriber::FmtSubscriber;

/// Clean the project.
pub fn cef_clean(artifacts_dir: &Path) -> Result<()> {
    // This routes log macros through tracing.
    LogTracer::init()?;

    // Setup the tracing subscriber globally.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(LevelFilter::from_level(Level::INFO))
        .finish();

    set_global_default(subscriber)?;

    info!("Cleaning project ..");

    Command::new("cargo")
        .args(&["clean"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    info!("Removing artifacts dir ..");

    // Remove the artifacts directory.
    if artifacts_dir.exists() {
        remove_dir_all(artifacts_dir)?;
    }

    Ok(())
}

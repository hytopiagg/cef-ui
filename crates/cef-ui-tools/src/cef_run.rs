use anyhow::Result;
use clap::Parser;
use std::{
    env::current_dir,
    process::{Command, Stdio}
};
use tracing::{info, level_filters::LevelFilter, subscriber::set_global_default, Level};
use tracing_log::LogTracer;
use tracing_subscriber::FmtSubscriber;

/// Command line arguments.
#[derive(Parser, Default)]
struct BuildArgs {
    /// Whether this is a release build.
    #[arg(long, default_value_t = false)]
    pub release: bool
}

/// Run the project.
pub fn cef_run() -> Result<()> {
    // This routes log macros through tracing.
    LogTracer::init()?;

    // Setup the tracing subscriber globally.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(LevelFilter::from_level(Level::INFO))
        .finish();

    set_global_default(subscriber)?;

    info!("Running project ..");

    let args = BuildArgs::parse();

    if cfg!(target_os = "macos") {
        run_macos(args.release)?;
    } else {
        run_linux(args.release)?;
    }

    Ok(())
}

/// Run the project on macOS.
fn run_linux(release: bool) -> Result<()> {
    Ok(())
}

/// Run the project on macOS.
fn run_macos(release: bool) -> Result<()> {
    let target_dir = match release {
        true => current_dir()?.join("target/release"),
        false => current_dir()?.join("target/debug")
    }
    .join("cef-ui-simple.app/Contents/MacOS/cef-ui-simple");

    Command::new(&target_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(())
}

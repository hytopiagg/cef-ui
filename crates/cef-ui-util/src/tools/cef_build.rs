use anyhow::Result;
use clap::Parser;
use std::process::{Command, Stdio};
use tracing::{info, level_filters::LevelFilter, subscriber::set_global_default, Level};
use tracing_log::LogTracer;
use tracing_subscriber::FmtSubscriber;

/// Command line arguments.
#[derive(Parser, Default)]
struct BuildArgs {
    /// Whether this is a release build.
    #[arg(long, default_value_t = String::from("dev"))]
    pub profile: String
}

/// Build the project.
pub fn cef_build() -> Result<()> {
    // This routes log macros through tracing.
    LogTracer::init()?;

    // Setup the tracing subscriber globally.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(LevelFilter::from_level(Level::INFO))
        .finish();

    set_global_default(subscriber)?;

    info!("Building main binary ..");

    let args = BuildArgs::parse();

    // Build the main executable.
    build_exe("cef-ui-simple", &args.profile)?;

    // On macOS, build the helper executable
    // and package the app bundle as required.
    if cfg!(target_os = "macos") {
        use crate::builds::build_app_bundle;

        info!("Building helper binary ..");

        build_exe("cef-ui-simple-helper", &args.profile)?;

        info!("Building app bundle ..");

        build_app_bundle(&args.profile)?;
    }

    Ok(())
}

/// Build a specific executable.
pub fn build_exe(name: &str, profile: &str) -> Result<()> {
    let args = vec!["build", "--bin", name, "--profile", profile];

    Command::new("cargo")
        .args(&args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(())
}

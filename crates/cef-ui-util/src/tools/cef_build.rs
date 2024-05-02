use anyhow::Result;
use clap::Parser;
use tracing::{info, level_filters::LevelFilter, subscriber::set_global_default, Level};
use tracing_log::LogTracer;
use tracing_subscriber::FmtSubscriber;

use crate::{build_exe, AppBundleSettings};

/// Command line arguments.
#[derive(Parser, Default)]
struct BuildArgs {
    /// Whether this is a release build.
    #[arg(long, default_value_t = String::from("dev"))]
    pub profile: String
}

pub struct BuildSettings {
    /// The name of the main package.
    pub package_main: String,

    /// The name of the helper package.
    pub package_helper: String,

    /// The app bundle settings.
    pub app_bundle_settings: AppBundleSettings
}

impl BuildSettings {
    pub fn run(&self) -> Result<()> {
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
        build_exe(&self.package_main, &args.profile)?;

        // On macOS, build the helper executable
        // and package the app bundle as required.
        if cfg!(target_os = "macos") {
            info!("Building helper binary ..");

            build_exe(&self.package_helper, &args.profile)?;

            info!("Building app bundle ..");

            self.app_bundle_settings
                .run(&args.profile)?;
        }

        Ok(())
    }
}

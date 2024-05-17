use anyhow::Result;
use cef_ui_util::{get_cef_artifacts_dir, get_cef_workspace_dir, AppBundleSettings, BuildCommand};
use clap::Parser;
use tracing::{level_filters::LevelFilter, subscriber::set_global_default, Level};
use tracing_log::LogTracer;
use tracing_subscriber::FmtSubscriber;

/// Command line arguments.
#[derive(Parser, Default)]
struct BuildArgs {
    /// Whether this is a release build.
    #[arg(long, default_value_t = String::from("dev"))]
    pub profile: String
}

fn main() -> Result<()> {
    // This routes log macros through tracing.
    LogTracer::init()?;

    // Setup the tracing subscriber globally.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(LevelFilter::from_level(Level::INFO))
        .finish();

    set_global_default(subscriber)?;

    let args = BuildArgs::parse();
    let workspace_dir = get_cef_workspace_dir()?;

    // Build the main executable.
    BuildCommand {
        binary:  String::from("cef-ui-simple"),
        profile: args.profile.to_string()
    }
    .run()?;

    // If on macOS, we need to do some extra work.
    if cfg!(target_os = "macos") {
        // Build the helper executable.
        BuildCommand {
            binary:  String::from("cef-ui-simple-helper"),
            profile: args.profile.to_string()
        }
        .run()?;

        // Build the app bundle.
        AppBundleSettings {
            profile:         args.profile.to_string(),
            artifacts_dir:   get_cef_artifacts_dir()?,
            app_name:        String::from("cef-ui-simple"),
            main_exe_name:   String::from("cef-ui-simple"),
            helper_exe_name: String::from("cef-ui-simple-helper"),
            resources_dir:   workspace_dir.join("resources/macos"),
            org_name:        String::from("hytopia")
        }
        .run()?;
    }

    Ok(())
}

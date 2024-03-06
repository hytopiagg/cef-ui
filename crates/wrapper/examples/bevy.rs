use anyhow::Result;
use bevy::{
    log::{
        tracing_subscriber::{filter::LevelFilter, FmtSubscriber},
        Level
    },
    utils::tracing::subscriber::set_global_default
};
use std::{env, path::PathBuf, process::exit};
use tracing_log::LogTracer;
use wrapper::{App, AppCallbacks, Context, LogSeverity, MainArgs, Settings};

pub struct MyCefApp;

impl AppCallbacks for MyCefApp {}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("Error: {}", e);
        exit(1);
    }
}

fn try_main() -> Result<()> {
    // This routes log macros through tracing.
    LogTracer::init()?;

    // Setup the tracing subscriber globally.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(LevelFilter::from_level(Level::DEBUG))
        .finish();

    set_global_default(subscriber)?;

    // TODO: Set this properly based on the platform.
    let root_cache_dir = PathBuf::from("/tmp");

    println!("Root cache path: {:?}", root_cache_dir);

    let main_args = MainArgs::new(env::args())?;
    let settings = Settings::new()
        .log_severity(LogSeverity::Warning)
        .root_cache_path(root_cache_dir)?;
    let app = App::new(MyCefApp {});

    println!("{:?}", main_args);

    let context = Context::new(main_args, settings, Some(app));

    // If this is a CEF subprocess, let it run and then
    // emit the proper exit code so CEF can clean up.
    if let Some(code) = context.is_cef_subprocess() {
        exit(code);
    }

    // Initialize CEF.
    context.initialize()?;

    // Shutdown CEF.
    context.shutdown();

    // App::new()
    //     .add_plugins(DefaultPlugins)
    //     .add_systems(Update, close_on_esc)
    //     .run();

    Ok(())
}

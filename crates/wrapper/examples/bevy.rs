use anyhow::{anyhow, Result};
use std::{env, path::PathBuf, process::exit};
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

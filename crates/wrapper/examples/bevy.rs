use anyhow::Result;
use std::{env, process::exit};
use wrapper::{App, AppCallbacks, Context, MainArgs, Settings};

pub struct MyCefApp;

impl AppCallbacks for MyCefApp {}

fn main() -> Result<()> {
    let main_args = MainArgs::new(env::args())?;
    let settings = Settings::default();
    let app = App::new(MyCefApp {});

    println!("{:?}", main_args);

    let context = Context::new(main_args, settings, Some(app));

    // If this is a CEF subprocess, let it run and then
    // emit the proper exit code so CEF can clean up.
    if let Some(code) = context.is_cef_subprocess() {
        exit(code);
    }

    // App::new()
    //     .add_plugins(DefaultPlugins)
    //     .add_systems(Update, close_on_esc)
    //     .run();

    Ok(())
}

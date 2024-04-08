use std::{
    env,
    fs::{create_dir_all, remove_dir_all},
    path::PathBuf,
    process::exit
};

use anyhow::Result;
use tracing::{level_filters::LevelFilter, subscriber::set_global_default, Level};
use tracing_log::LogTracer;
use tracing_subscriber::FmtSubscriber;

use cef_ui::{
    App, AppCallbacks, BrowserHost, BrowserProcessHandler, BrowserSettings, Client,
    ClientCallbacks, CommandLine, Context, ContextMenuHandler, KeyboardHandler, LifeSpanHandler,
    LogSeverity, MainArgs, RenderHandler, Settings, WindowInfo
};

pub struct MyAppCallbacks;

impl AppCallbacks for MyAppCallbacks {
    fn on_before_command_line_processing(&mut self, _: Option<&str>, _: Option<CommandLine>) {}

    fn get_browser_process_handler(&mut self) -> Option<BrowserProcessHandler> {
        None
    }
}

pub struct MyClientCallbacks;

impl ClientCallbacks for MyClientCallbacks {
    fn get_context_menu_handler(&mut self) -> Option<ContextMenuHandler> {
        None
    }

    fn get_keyboard_handler(&mut self) -> Option<KeyboardHandler> {
        None
    }

    fn get_life_span_handler(&mut self) -> Option<LifeSpanHandler> {
        None
    }

    fn get_render_handler(&mut self) -> Option<RenderHandler> {
        None
    }
}

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

    // TODO: This should be platform specific.
    let root_cache_dir = PathBuf::from("/tmp/simple");

    ensure_root_cache_dir(&root_cache_dir)?;

    let main_args = MainArgs::new(env::args())?;

    let settings = Settings::new()
        .log_severity(LogSeverity::Warning)
        .root_cache_path(&root_cache_dir)?;

    let app = App::new(MyAppCallbacks {});

    let context = Context::new(main_args, settings, Some(app));

    // If this is a CEF subprocess, let it run and then
    // emit the proper exit code so CEF can clean up.
    if let Some(code) = context.is_cef_subprocess() {
        exit(code);
    }

    // Initialize CEF.
    context.initialize()?;

    let window_info = WindowInfo::new().window_name(&String::from("Simple"));
    let browser_settings = BrowserSettings::new();
    let client = Client::new(MyClientCallbacks);

    // Create a new browser.
    BrowserHost::create_browser_sync(
        &window_info,
        client,
        "https://www.google.com/",
        &browser_settings,
        None,
        None
    );

    // Run the message loop.
    context.run_message_loop();

    // Shutdown CEF.
    context.shutdown();

    Ok(())
}

/// Remove the root cache directory if it exists and then create it.
fn ensure_root_cache_dir(path: &PathBuf) -> Result<()> {
    if path.exists() {
        remove_dir_all(path)?;
    }

    create_dir_all(path)?;

    Ok(())
}

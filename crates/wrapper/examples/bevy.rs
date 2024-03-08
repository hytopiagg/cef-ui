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
use winit::{
    raw_window_handle::{HasWindowHandle, RawWindowHandle},
    window::Window
};
use wrapper::{
    App, AppCallbacks, BrowserHost, BrowserSettings, Client, ClientCallbacks, Context, LogSeverity,
    MainArgs, Settings, WindowHandle, WindowInfo
};

pub struct MyAppCallbacks;

impl AppCallbacks for MyAppCallbacks {}

pub struct MyClientCallbacks;

impl ClientCallbacks for MyClientCallbacks {}

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
        .root_cache_path(&root_cache_dir)?;
    let app = App::new(MyAppCallbacks {});

    println!("{:?}", main_args);

    let context = Context::new(main_args, settings, Some(app));

    // If this is a CEF subprocess, let it run and then
    // emit the proper exit code so CEF can clean up.
    if let Some(code) = context.is_cef_subprocess() {
        exit(code);
    }

    // Initialize CEF.
    context.initialize()?;

    // let event_loop = EventLoop::new()?;
    //
    // Create a new window.
    // let window = WindowBuilder::new()
    //     .with_title("Bevy")
    //     .build(&event_loop)?;
    //
    // let window_info = get_window_info(&window)?;

    let window_info = WindowInfo::new().window_name(&String::from("Bevy"));
    let browser_settings = BrowserSettings::new();
    let client = Client::new(MyClientCallbacks);

    // Create a new browser.
    BrowserHost::create_browser_sync(
        &window_info,
        client,
        "https://www.google.com/",
        &browser_settings,
        None
    );

    // Run the message loop.
    context.run_message_loop();

    // Shutdown CEF.
    context.shutdown();

    // App::new()
    //     .add_plugins(DefaultPlugins)
    //     .add_systems(Update, close_on_esc)
    //     .run();

    Ok(())
}

/// Get the window info on Linux.
#[cfg(target_os = "linux")]
fn get_window_info(window: &Window) -> Result<WindowInfo> {
    let window_info = WindowInfo::new().window_name(&String::from("Bevy"));
    let window_info = match window.window_handle()?.as_raw() {
        RawWindowHandle::Xlib(handle) => {
            window_info.parent_window(WindowHandle::new(handle.window))
        },
        _ => panic!("Unsupported window handle type!")
    };

    Ok(window_info)
}

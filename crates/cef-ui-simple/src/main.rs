use std::{
    env,
    fs::{create_dir_all, remove_dir_all, File},
    path::PathBuf,
    process::exit
};

use anyhow::Result;
use tracing::{level_filters::LevelFilter, subscriber::set_global_default, Level};
use tracing_log::LogTracer;
use tracing_subscriber::FmtSubscriber;

use cef_ui::{
    App, AppCallbacks, Browser, BrowserHost, BrowserProcessHandler, BrowserSettings, Client,
    ClientCallbacks, CommandLine, Context, ContextMenuHandler, ContextMenuHandlerCallbacks,
    ContextMenuParams, EventFlags, Frame, KeyboardHandler, LifeSpanHandler, LogSeverity, MainArgs,
    MenuCommandId, MenuModel, Point, QuickMenuEditStateFlags, RenderHandler,
    RunContextMenuCallback, RunQuickMenuCallback, Settings, Size, WindowInfo
};
use log::{debug, error};

pub struct MyAppCallbacks;

impl AppCallbacks for MyAppCallbacks {
    fn on_before_command_line_processing(
        &mut self,
        process_type: Option<&str>,
        command_line: Option<CommandLine>
    ) {
        if let Some(command_line) = command_line {
            if process_type.is_none() {
                debug!("Setting command line switches.");

                // This is to disable scary warnings on macOS.
                #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
                if let Err(e) = command_line.append_switch("--use-mock-keychain") {
                    error!("{}", e);
                }
            }
        }
    }

    fn get_browser_process_handler(&mut self) -> Option<BrowserProcessHandler> {
        None
    }
}

pub struct MyContextMenuHandler;

#[allow(unused_variables)]
impl ContextMenuHandlerCallbacks for MyContextMenuHandler {
    fn on_before_context_menu(
        &mut self,
        browser: Browser,
        frame: Frame,
        params: ContextMenuParams,
        model: MenuModel
    ) {
        if let Err(e) = model.clear() {
            error!("{}", e);
        }
    }

    fn run_context_menu(
        &mut self,
        browser: Browser,
        frame: Frame,
        params: ContextMenuParams,
        model: MenuModel,
        callback: RunContextMenuCallback
    ) -> bool {
        false
    }

    fn on_context_menu_command(
        &mut self,
        browser: Browser,
        frame: Frame,
        params: ContextMenuParams,
        command_id: MenuCommandId,
        event_flags: EventFlags
    ) -> bool {
        false
    }

    fn on_context_menu_dismissed(&mut self, browser: Browser, frame: Frame) {}

    fn run_quick_menu(
        &mut self,
        browser: Browser,
        frame: Frame,
        location: &Point,
        size: &Size,
        edit_state_flags: QuickMenuEditStateFlags,
        callback: RunQuickMenuCallback
    ) -> bool {
        false
    }

    fn on_quick_menu_command(
        &mut self,
        browser: Browser,
        frame: Frame,
        command_id: MenuCommandId,
        event_flags: EventFlags
    ) -> bool {
        false
    }

    fn on_quick_menu_dismissed(&mut self, browser: Browser, frame: Frame) {}
}

pub struct MyClientCallbacks;

impl ClientCallbacks for MyClientCallbacks {
    fn get_context_menu_handler(&mut self) -> Option<ContextMenuHandler> {
        Some(ContextMenuHandler::new(MyContextMenuHandler {}))
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

    // Open a file to write logs to
    let log_file = File::create("/Users/kevin/repos/cef-ui/CEF.log")?;

    // Setup the tracing subscriber globally.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(LevelFilter::from_level(Level::DEBUG))
        .with_writer(log_file)
        .finish();

    set_global_default(subscriber)?;

    // TODO: This should be platform specific.
    let root_cache_dir = PathBuf::from("/tmp/simple");

    ensure_root_cache_dir(&root_cache_dir)?;

    let main_args = MainArgs::new(env::args())?;

    let settings = Settings::new()
        .log_severity(LogSeverity::Warning)
        .root_cache_path(&root_cache_dir)?
        .no_sandbox(true);

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

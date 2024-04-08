use crate::{
    bindings::{
        cef_do_message_loop_work, cef_execute_process, cef_initialize, cef_quit_message_loop,
        cef_run_message_loop, cef_shutdown
    },
    App, MainArgs, Settings
};
use anyhow::{anyhow, Result};
use std::ptr::null_mut;

pub struct Context {
    pub main_args: MainArgs,
    pub settings:  Settings,
    pub app:       Option<App>
}

impl Context {
    pub fn new(main_args: MainArgs, settings: Settings, app: Option<App>) -> Self {
        Self {
            main_args,
            settings,
            app
        }
    }

    /// This function should be called from the application entry point function to
    /// execute a secondary process. It can be used to run secondary processes from
    /// the browser client executable (default behavior) or from a separate
    /// executable specified by the cef_settings_t.browser_subprocess_path value. If
    /// called for the browser process (identified by no "type" command-line value)
    /// it will return immediately with a value of -1. If called for a recognized
    /// secondary process it will block until the process should exit and then
    /// return the process exit code. The |application| parameter may be NULL. The
    /// |windows_sandbox_info| parameter is only used on Windows and may be NULL
    /// (see cef_sandbox_win.h for details).
    pub fn is_cef_subprocess(&self) -> Option<i32> {
        let code = unsafe {
            let app = self
                .app
                .clone()
                .map(|app| app.into_raw())
                .unwrap_or(null_mut());

            cef_execute_process(self.main_args.as_raw(), app, null_mut())
        };

        match code {
            -1 => None,
            _ => Some(code)
        }
    }

    /// This function should be called on the main application thread to initialize
    /// the CEF browser process. The |application| parameter may be NULL. Returns
    /// true (1) if initialization succeeds. Returns false (0) if initialization
    /// fails or if early exit is desired (for example, due to process singleton
    /// relaunch behavior). If this function returns false (0) then the application
    /// should exit immediately without calling any other CEF functions. The
    /// |windows_sandbox_info| parameter is only used on Windows and may be NULL
    /// (see cef_sandbox_win.h for details).
    pub fn initialize(&self) -> Result<()> {
        match unsafe {
            cef_initialize(
                self.main_args.as_raw(),
                self.settings.as_raw(),
                self.app
                    .clone()
                    .map(|app| app.into_raw())
                    .unwrap_or(null_mut()),
                null_mut()
            ) != 0
        } {
            true => Ok(()),
            false => Err(anyhow!("Failed to initialize CEF."))
        }
    }

    /// Run the CEF message loop. Use this function instead of an application-
    /// provided message loop to get the best balance between performance and CPU
    /// usage. This function should only be called on the main application thread
    /// and only if cef_initialize() is called with a
    /// cef_settings_t.multi_threaded_message_loop value of false (0). This function
    /// will block until a quit message is received by the system.
    pub fn run_message_loop(&self) {
        unsafe { cef_run_message_loop() };
    }

    /// Quit the CEF message loop that was started by calling
    /// cef_run_message_loop(). This function should only be called on the main
    /// application thread and only if cef_run_message_loop() was used.
    pub fn quit_message_loop(&self) {
        unsafe { cef_quit_message_loop() };
    }

    /// This function should be called on the main application thread to shut down
    /// the CEF browser process before the application exits. Do not call any other
    /// CEF functions after calling this function.
    pub fn shutdown(&self) {
        unsafe { cef_shutdown() };
    }

    /// Perform a single iteration of CEF message loop processing. This function is
    /// provided for cases where the CEF message loop must be integrated into an
    /// existing application message loop. Use of this function is not recommended
    /// for most users; use either the cef_run_message_loop() function or
    /// cef_settings_t.multi_threaded_message_loop if possible. When using this
    /// function care must be taken to balance performance against excessive CPU
    /// usage. It is recommended to enable the cef_settings_t.external_message_pump
    /// option when using this function so that
    /// cef_browser_process_handler_t::on_schedule_message_pump_work() callbacks can
    /// facilitate the scheduling process. This function should only be called on
    /// the main application thread and only if cef_initialize() is called with a
    /// cef_settings_t.multi_threaded_message_loop value of false (0). This function
    /// will not block.
    pub fn do_message_loop_work(&self) {
        unsafe { cef_do_message_loop_work() };
    }
}

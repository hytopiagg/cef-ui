use crate::{CefMainArgs, CefSettings};
use cef_ui_bindings_linux_x86_64::cef_execute_process;
use std::ptr::null_mut;

/// Wraps cef_app_t.
pub struct CefApp {
    pub main_args: CefMainArgs,
    pub settings:  CefSettings
}

impl CefApp {
    pub fn new(main_args: CefMainArgs, settings: CefSettings) -> Self {
        Self {
            main_args,
            settings
        }
    }

    /// You MUST call this at the start of your application. If it returns true,
    /// then initialize CEF and your application. If it returns false, then exit
    /// your application, returning the error code.
    pub fn is_cef_subprocess(&self) -> Option<i32> {
        // TODO: Populate this with a reference to the app!
        let code = unsafe { cef_execute_process(&self.main_args.as_raw(), null_mut(), null_mut()) };

        match code {
            -1 => None,
            _ => Some(code)
        }
    }

    // pub fn execute_process<T: App>(args: &MainArgs, app: Option<T>) -> i32 {
    //     let args = args.to_raw();
    //     let app = app
    //         .map(|app| app.into_raw(true))
    //         .unwrap_or(std::ptr::null_mut());
    //
    //     unsafe { cef_execute_process(&args, app, std::ptr::null_mut()) }
    // }

    // pub fn check_cef_process(&self) -> i32 {
    //     unsafe {
    //         cef_execute_process()
    //         //cef_check_cef_process(self.main_args.as_raw())
    //     }
    //
    //     0
    // }

    // pub fn initialize(&self) -> i32 {
    //     // unsafe {
    //     //     cef_initialize(
    //     //         self.main_args.as_raw(),
    //     //         self.settings.as_raw(),
    //     //         self,
    //     //         std::ptr::null_mut()
    //     //     )
    //     // }
    //
    //     0
    // }

    pub fn run(&self) {
        // call cef_initialize
        // create cef_browser_window_info_t
        // call cef_browser_host_create_browser
        // call cef_run_message_loop
        // call cef_shutdown
    }
}

// // Continuing from the previous code snippet...
// cef_app_t* app = create_cef_app();
//
// int result = cef_initialize(&main_args, &settings, app, NULL);
// if (!result) {
// // Handle initialization failure
// return -1;
// }
//
// // Run the CEF message loop
// cef_run_message_loop();
//
// // Shutdown CEF when done
// cef_shutdown();
//
// return 0;

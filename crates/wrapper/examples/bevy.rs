// use bevy::{app::App, prelude::Update, window::close_on_esc, DefaultPlugins};
use anyhow::Result;
use cef_ui_bindings_linux_x86_64::cef_app_t;
use std::{env, mem::zeroed, process::exit};
use wrapper::{CefApp, CefMainArgs, CefRefCountedPtr, CefSettings};

fn main() -> Result<()> {
    let cef_main_args = CefMainArgs::new(env::args())?;

    println!("{:?}", cef_main_args);

    let cef_settings = CefSettings::default();

    let the_cef_app_t: cef_app_t = unsafe { zeroed() };
    let the_cef_app = CefApp::new(cef_main_args, cef_settings);
    let cef_app_ptr = CefRefCountedPtr::new(the_cef_app_t, the_cef_app);

    let _copy = cef_app_ptr.clone();

    // If this is a CEF subprocess, let it run and then
    // emit the proper exit code so CEF can clean up.
    if let Some(code) = cef_app_ptr.is_cef_subprocess() {
        exit(code);
    }

    // App::new()
    //     .add_plugins(DefaultPlugins)
    //     .add_systems(Update, close_on_esc)
    //     .run();

    Ok(())
}

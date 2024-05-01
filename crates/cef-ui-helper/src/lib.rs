use crate::{
    main_args::{cef_main_args_t, MainArgs},
    sandbox::ScopedSandbox
};
use anyhow::{anyhow, Result};
use libloading::{Library, Symbol};
use log::{error, info};
use std::{
    env::current_exe,
    ffi::{c_int, c_void},
    fs::canonicalize,
    path::PathBuf,
    process::exit,
    ptr::null_mut
};
use tracing::{level_filters::LevelFilter, subscriber::set_global_default, Level};
use tracing_log::LogTracer;
use tracing_subscriber::FmtSubscriber;

mod main_args;
mod sandbox;

/// The relative path to the CEF framework library on macOS.
const CEF_PATH: &str = "../../../Chromium Embedded Framework.framework/Chromium Embedded Framework";

/// Returns the CEF error code or 1 if an error occurred.
pub fn run() {
    let ret = try_run().unwrap_or_else(|e| {
        error!("An error occurred: {}", e);

        1
    });

    info!("The return code is: {}", ret);

    exit(ret);
}

/// Try and run the helper, returning the CEF error code if successful.
fn try_run() -> Result<i32> {
    // This routes log macros through tracing.
    LogTracer::init()?;

    // Open a file to emit logs to.
    // let filename = get_log_path(&PathBuf::from("/Users/kevin/repos/cef-ui"))?;
    // let log_file = File::create(filename)?;

    // Setup the tracing subscriber globally.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(LevelFilter::from_level(Level::DEBUG))
        // .with_writer(log_file)
        .finish();

    set_global_default(subscriber)?;

    // Setup the sandbox if enabled.
    #[cfg(feature = "sandbox")]
    let _sandbox = ScopedSandbox::new()?;

    // Manually load CEF and execute the subprocess.
    let ret = unsafe {
        // Load our main args.
        let main_args = MainArgs::new()?;

        info!("Main args: {:?}", main_args);

        // Manually load the cef_execute_process function.
        let cef_path = get_cef_path(CEF_PATH)?;
        let lib = Library::new(cef_path)?;
        let cef_execute_process: Symbol<
            unsafe extern "C" fn(args: *const cef_main_args_t, *mut c_void, *mut c_void) -> c_int
        > = lib.get(b"cef_execute_process")?;

        info!("Executing CEF subprocess ..");

        let ret = cef_execute_process(main_args.as_raw(), null_mut(), null_mut()) as i32;

        info!("CEF exited with code: {}", ret);

        lib.close()?;

        info!("Closed CEF library.");

        ret
    };

    Ok(ret)
}

/// Convert the exe name to something sane.
#[allow(dead_code)]
fn get_log_path(base: &PathBuf) -> Result<PathBuf> {
    let name = current_exe()?;
    let name = name
        .file_stem()
        .ok_or_else(|| anyhow!("Could not get exe name."))?;
    let name = name
        .to_str()
        .ok_or_else(|| anyhow!("Could not convert exe name to string."))?;
    let name = name
        .replace(" ", "")
        .replace("(", "")
        .replace(")", "");
    let name = format!("HELPER-{}.log", name);
    let path = base.join(name);

    Ok(path)
}

/// Get the cef library path.
fn get_cef_path(relative_path: &str) -> Result<PathBuf> {
    let cef_path = current_exe()?
        .parent()
        .map(PathBuf::from)
        .ok_or_else(|| anyhow!("Could not get parent directory"))?;
    let cef_path = cef_path.join(relative_path);
    let cef_path = canonicalize(cef_path)?;

    Ok(cef_path)
}

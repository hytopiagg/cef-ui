use crate::main_args::{cef_main_args_t, MainArgs};
use anyhow::{anyhow, Result};
use libloading::{Library, Symbol};
use log::{error, info};
use std::{
    env,
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

/// The relative path to the CEF framework library on macOS.
const CEF_PATH: &str = "../../../Chromium Embedded Framework.framework/Chromium Embedded Framework";

fn main() {
    let ret = try_main().unwrap_or_else(|e| {
        error!("An error occurred: {}", e);

        1
    });

    info!("The return code is: {}", ret);

    exit(ret);
}

fn try_main() -> Result<i32> {
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

    let cef_path = get_cef_path(CEF_PATH)?;

    info!("cef_path: {:?}", cef_path);

    let main_args = MainArgs::new(env::args())?;

    info!("main_args: {:?}", main_args);

    let ret = unsafe {
        let lib = Library::new(cef_path)?;

        let cef_execute_process: Symbol<
            unsafe extern "C" fn(args: *const cef_main_args_t, *mut c_void, *mut c_void) -> c_int
        > = lib.get(b"cef_execute_process")?;

        let ret = cef_execute_process(main_args.as_raw(), null_mut(), null_mut()) as i32;

        lib.close()?;

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

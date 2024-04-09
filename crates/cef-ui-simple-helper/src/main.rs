use crate::main_args::{cef_main_args_t, MainArgs};
use anyhow::{anyhow, Result};
use libloading::{Library, Symbol};
use log::{debug, error};
use rand::{prelude::StdRng, Rng, SeedableRng};
use std::{
    env,
    ffi::{c_int, c_void},
    fs::{canonicalize, File},
    path::PathBuf,
    process::exit,
    ptr::null_mut,
    time::{SystemTime, UNIX_EPOCH}
};
use tracing::{level_filters::LevelFilter, subscriber::set_global_default, Level};
use tracing_log::LogTracer;
use tracing_subscriber::FmtSubscriber;

mod main_args;

/// The relative path to the CEF framework library on macOS.
const CEF_PATH: &str = "../../../Chromium Embedded Framework.framework/Chromium Embedded Framework";

fn main() {
    let ret = try_main().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);

        1
    });

    exit(ret);
}

fn try_main() -> Result<i32> {
    // This routes log macros through tracing.
    LogTracer::init()?;

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    // Use the duration's whole seconds as the seed
    let seed = since_the_epoch.as_secs();

    // Create a seeded RNG
    let mut rng = StdRng::seed_from_u64(seed);

    // Generate a random number
    let random_number: i32 = rng.gen();

    let filename = format!("/Users/kevin/repos/cef-ui/HELPER-{}.log", random_number);

    // Open a file to write logs to
    let log_file = File::create(filename)?;

    // Setup the tracing subscriber globally.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(LevelFilter::from_level(Level::TRACE))
        .with_writer(log_file)
        .finish();

    set_global_default(subscriber)?;

    let cef_path = get_cef_path(CEF_PATH)?;

    debug!("cef_path: {:?}", cef_path);

    let main_args = MainArgs::new(env::args())?;

    debug!("main_args: {:?}", main_args);

    let ret = unsafe {
        let lib = Library::new(cef_path)?;

        let cef_execute_process: Symbol<
            unsafe extern "C" fn(args: *const cef_main_args_t, *mut c_void, *mut c_void) -> c_int
        > = lib
            .get(b"cef_execute_process")
            .unwrap_or_else(|e| {
                debug!("Could not get cef_execute_process: {:?}", e);

                exit(1);
            });

        debug!("loaded cef_execute_process");

        let ret = cef_execute_process(main_args.as_raw(), null_mut(), null_mut()) as i32;

        debug!("cef_execute_process returned: {}", ret);

        if let Err(e) = lib.close() {
            error!("Could not close library: {:?}", e);
        }

        ret
    };

    debug!("cef_execute_process returned: {}", ret);

    Ok(ret)
}

/// Get the directory of the current executable.
fn get_exe_dir() -> Result<PathBuf> {
    env::current_exe()?
        .parent()
        .map(PathBuf::from)
        .ok_or_else(|| anyhow!("Could not get parent directory"))
}

/// Get the cef library path.
fn get_cef_path(relative_path: &str) -> Result<PathBuf> {
    let cef_path = get_exe_dir()?.join(relative_path);
    let cef_path = canonicalize(cef_path)?;

    Ok(cef_path)
}

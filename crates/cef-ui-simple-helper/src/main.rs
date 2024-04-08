use crate::main_args::{cef_main_args_t, MainArgs};
use anyhow::{anyhow, Result};
use libloading::{Library, Symbol};
use std::{
    env,
    ffi::{c_int, c_void},
    fs::canonicalize,
    path::PathBuf,
    process::exit,
    ptr::null_mut
};

mod main_args;

fn main() {
    let ret = try_main().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);

        1
    });

    exit(ret);
}

fn try_main() -> Result<i32> {
    let cef_path = get_cef_path()?;

    println!("cef_path: {:?}", cef_path);

    let main_args = MainArgs::new(env::args())?;

    let ret = unsafe {
        let lib = Library::new(cef_path)?;

        let cef_execute_process: Symbol<
            unsafe extern "C" fn(args: *const cef_main_args_t, *mut c_void, *mut c_void) -> c_int
        > = lib.get(b"cef_execute_process")?;

        cef_execute_process(main_args.as_raw(), null_mut(), null_mut()) as i32
    };

    println!("cef_execute_process returned: {}", ret);

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
fn get_cef_path() -> Result<PathBuf> {
    let cef_path = get_exe_dir()?.join("../../artifacts/cef/libcef.so");
    let cef_path = canonicalize(cef_path)?;

    Ok(cef_path)
}

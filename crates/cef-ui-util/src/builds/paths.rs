use crate::get_exe_dir;
use anyhow::Result;
use std::{env::var, fs::canonicalize, path::PathBuf};

/// Get the workspace directory. Only call within build.rs!
pub fn get_build_rs_workspace_dir() -> Result<PathBuf> {
    let dir = PathBuf::from(var("CARGO_MANIFEST_DIR")?).join("../..");
    let dir = canonicalize(dir)?;

    Ok(dir)
}

/// Get the target directory. Only call within build.rs!
pub fn get_build_rs_target_dir() -> Result<PathBuf> {
    let profile = var("PROFILE")?;
    let dir = get_build_rs_workspace_dir()?
        .join("target")
        .join(profile);

    Ok(dir)
}

/// Get the workspace directory. Only call from tools.
pub fn get_tool_workspace_dir() -> Result<PathBuf> {
    let dir = get_exe_dir()?.join("../..");
    let dir = canonicalize(dir)?;

    Ok(dir)
}

/// Get the target directory. Only call from tools.
pub fn get_tool_target_dir(profile: &str) -> Result<PathBuf> {
    let dir = get_tool_workspace_dir()?.join("target");
    let dir = match profile == "dev" {
        true => dir.join("debug"),
        false => dir.join(profile)
    };

    Ok(dir)
}

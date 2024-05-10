use crate::get_exe_dir;
use anyhow::Result;
use cargo_metadata::MetadataCommand;
use std::{env::var, fs::canonicalize, path::PathBuf};

/// The name of the artifacts directory environment variable.
pub const CEF_ARTIFACTS_DIR: &str = "CEF_ARTIFACTS_DIR";

/// The default artifacts directory, relative to the
/// workspace root. You can customize this by setting
/// the CEF_ARTIFACTS_DIR environment variable within
/// the .cargo/config.toml file.
pub const DEFAULT_CEF_ARTIFACTS_DIR: &str = "artifacts";

/// The cef directory within the artifacts dir.
pub const CEF_DIRECTORY: &str = "cef";

/// Get the workspace directory. Only call within build.rs!
pub fn get_build_rs_workspace_dir() -> Result<PathBuf> {
    let metadata = MetadataCommand::new().exec()?;

    Ok(metadata.workspace_root.into())
}

/// Get the target directory. Only call within build.rs!
pub fn get_build_rs_target_dir() -> Result<PathBuf> {
    let profile = var("PROFILE")?;
    let dir = get_build_rs_workspace_dir()?
        .join("target")
        .join(profile);

    Ok(dir)
}

/// Get the artifacts dir. Only call within build.rs!
pub fn get_build_rs_artifacts_dir() -> Result<PathBuf> {
    let dir = get_build_rs_workspace_dir()?;
    let dir = dir.join(var(CEF_ARTIFACTS_DIR).unwrap_or(DEFAULT_CEF_ARTIFACTS_DIR.into()));

    Ok(dir)
}

/// Get the cef directory. Only call within build.rs!
pub fn get_build_rs_cef_dir() -> Result<PathBuf> {
    let dir = get_build_rs_artifacts_dir()?.join(CEF_DIRECTORY);

    Ok(dir)
}

/// Get the workspace directory. Only call from tools!
pub fn get_tool_workspace_dir() -> Result<PathBuf> {
    let dir = get_exe_dir()?.join("../..");
    let dir = canonicalize(dir)?;

    Ok(dir)
}

/// Get the target directory. Only call from tools!
pub fn get_tool_target_dir(profile: &str) -> Result<PathBuf> {
    let dir = get_tool_workspace_dir()?.join("target");
    let dir = match profile == "dev" {
        true => dir.join("debug"),
        false => dir.join(profile)
    };

    Ok(dir)
}

/// Get the artifacts dir. Only call from tools!
pub fn get_tool_artifacts_dir() -> Result<PathBuf> {
    let dir = get_tool_workspace_dir()?;
    let dir = dir.join(var(CEF_ARTIFACTS_DIR).unwrap_or(DEFAULT_CEF_ARTIFACTS_DIR.into()));

    Ok(dir)
}

/// Get the cef directory. Only call from tools!
pub fn get_tool_cef_dir() -> Result<PathBuf> {
    let dir = get_tool_artifacts_dir()?.join(CEF_DIRECTORY);

    Ok(dir)
}

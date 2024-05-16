use anyhow::Result;
use std::{env::var, path::PathBuf};

/// We must use environment variables for specifying the
/// workspace and the artifacts directory because of the
/// way that CEF links at compile time and at runtime.
pub const CEF_WORKSPACE_DIR: &str = "CEF_WORKSPACE_DIR";
pub const CEF_ARTIFACTS_DIR: &str = "CEF_ARTIFACTS_DIR";

/// The cef directory within the artifacts dir.
pub const CEF_DIRECTORY: &str = "cef";

/// Get the workspace directory. Only call within build.rs!
pub fn get_cef_workspace_dir() -> Result<PathBuf> {
    Ok(var(CEF_WORKSPACE_DIR)?.into())
}

/// Get the target directory. Only call within build.rs!
pub fn get_cef_target_dir(profile: &str) -> Result<PathBuf> {
    // The debug profile is actually called "dev".
    let profile = match profile == "dev" {
        true => "debug",
        false => profile
    };

    Ok(get_cef_workspace_dir()?
        .join("target")
        .join(profile))
}

/// Get the artifacts dir. Only call within build.rs!
pub fn get_cef_artifacts_dir() -> Result<PathBuf> {
    Ok(var(CEF_ARTIFACTS_DIR)?.into())
}

/// Get the cef directory. Only call within build.rs!
pub fn get_cef_cef_dir() -> Result<PathBuf> {
    Ok(get_cef_artifacts_dir()?.join(CEF_DIRECTORY))
}

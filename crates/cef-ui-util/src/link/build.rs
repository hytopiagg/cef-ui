use crate::{download_file, extract_tar_gz, get_exe_dir};
use anyhow::Result;
use std::{
    env::{
        consts::{ARCH, OS},
        var
    },
    fs::{canonicalize, create_dir_all},
    path::{Path, PathBuf}
};

/// The current CEF version.
pub const CEF_VERSION: &str = "v0.1.0";

/// Returns the platform-specific CEF artifacts url.
pub fn get_cef_url() -> String {
    format!(
        "https://github.com/hytopiagg/cef-ui/releases/download/cef-artifacts-{}/cef-{}-{}.tgz",
        CEF_VERSION, OS, ARCH
    )
}

/// Downloads the tarball, untars it, and decompresses it. If the
/// target directory exists, then this function does nothing.
pub fn download_and_extract_cef(dir: &Path) -> Result<()> {
    let url = get_cef_url();

    if dir.exists() {
        return Ok(());
    }

    // Create the new directory.
    create_dir_all(dir)?;

    let path = dir.join("cef.tgz");

    download_file(&url, &path)?;
    extract_tar_gz(&path, dir)?;

    Ok(())
}

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

use crate::{download_file, extract_tar_gz};
use anyhow::Result;
use std::{
    env::consts::{ARCH, OS},
    fs::create_dir_all,
    path::Path
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

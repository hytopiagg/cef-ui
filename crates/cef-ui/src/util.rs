use anyhow::Result;
use flate2::read::GzDecoder;
use reqwest::blocking::get;
use std::{
    fs::{create_dir, remove_dir_all, File},
    io::{copy, Cursor, ErrorKind},
    path::Path
};
use tar::Archive;

/// Download a file to disk.
pub fn download_file(url: &str, path: &Path) -> Result<()> {
    let response = get(url)?;
    let mut file = File::create(path)?;
    let mut content = Cursor::new(response.bytes()?);

    copy(&mut content, &mut file)?;

    Ok(())
}

/// Untar and decompress a file.
pub fn extract_tar_gz(path: &Path, dir: &Path) -> Result<()> {
    let file = File::open(path)?;
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);

    archive.unpack(dir)?;

    Ok(())
}

/// Download a tarball, untar it, and decompress it. This
/// makes sure that any existing files are overwritten.
pub fn download_and_extract_tar_gz(url: &str, dir: &Path, name: &str) -> Result<()> {
    // Remove any existing folder.
    match remove_dir_all(dir) {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e)
    }?;

    // Create the new directory.
    create_dir(dir)?;

    let path = dir.join(name);

    download_file(url, &path)?;
    extract_tar_gz(&path, dir)?;

    Ok(())
}

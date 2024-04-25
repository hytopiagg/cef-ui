use anyhow::{anyhow, Result};
use bzip2::read::BzDecoder;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use percent_encoding::percent_decode_str;
use reqwest::blocking::get;
use std::{
    env::{current_dir, current_exe},
    fs::{self, create_dir, create_dir_all, read_dir, remove_dir_all, remove_file, File},
    io::{self, Cursor},
    path::{Path, PathBuf}
};
use tar::{Archive, Builder};
use url::Url;

/// Get the path of the current executable.
pub fn get_exe_dir() -> Result<PathBuf> {
    let exe_dir = current_exe()?;
    let exe_dir = exe_dir
        .parent()
        .ok_or_else(|| anyhow!("Failed to get parent directory of executable."))?;

    Ok(exe_dir.to_path_buf())
}

/// Gets the project directory.
pub fn get_project_dir() -> Result<PathBuf> {
    let exe_dir = get_exe_dir()?.join("../../");
    let exe_dir = exe_dir.canonicalize()?;

    Ok(exe_dir)
}

/// Gets the target directory.
pub fn get_target_dir(release: bool) -> Result<PathBuf> {
    let target_dir = current_dir()?;
    let target_dir = target_dir
        .join("target")
        .join(match release {
            true => "release",
            false => "debug"
        });

    Ok(target_dir)
}

/// Given a url, get the filename it points to.
pub fn get_url_filename(url: &str) -> Result<String> {
    let parsed = Url::parse(url)?;
    let segments = parsed
        .path_segments()
        .ok_or_else(|| anyhow!("Failed to get path segments."))?;
    let filename = segments
        .last()
        .ok_or_else(|| anyhow!("Failed to get last path segment."))?;
    let filename = percent_decode_str(filename)
        .decode_utf8()
        .map_err(|e| anyhow::anyhow!("Failed to decode filename: {}", e))?;

    Ok(filename.into())
}

/// Download a file to disk.
pub fn download_file(url: &str, path: &Path) -> Result<()> {
    let response = get(url)?;
    let mut file = File::create(path)?;
    let mut content = Cursor::new(response.bytes()?);

    io::copy(&mut content, &mut file)?;

    Ok(())
}

/// Extract a tar gzipped file.
pub fn extract_tar_gz(file: &Path, dir: &Path) -> Result<()> {
    let file = File::open(file)?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);

    archive.unpack(dir)?;

    Ok(())
}

/// Extract a bzip2 file.
pub fn extract_bz2(file: &Path, dir: &Path) -> Result<()> {
    let file = File::open(file)?;
    let decoder = BzDecoder::new(file);
    let mut archive = Archive::new(decoder);

    archive.unpack(dir)?;

    Ok(())
}

/// Create a tar gzipped file.
pub fn create_tar_gz(file: &Path, dir: &Path) -> Result<()> {
    let file = File::create(file)?;
    let encoder = GzEncoder::new(file, Compression::default());
    let mut archive = Builder::new(encoder);

    let stem = dir
        .file_stem()
        .ok_or_else(|| anyhow!("Failed to get file stem."))?;

    archive.append_dir_all(stem, dir)?;
    archive.finish()?;

    Ok(())
}

/// Copies all files and directories in one directory to a target
/// directory, resursively. Copies both files and directories. It
/// replaces files if they already exist.
pub fn copy_files(src: &Path, dst: &Path) -> Result<()> {
    // Ensure destination exists.
    if !dst.exists() {
        create_dir_all(dst)?;
    }

    copy_recursive(src, dst)?;

    Ok(())
}

/// Recursively copies files and directories from src to dst.
fn copy_recursive(src: &Path, dst: &Path) -> Result<()> {
    for entry in read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if file_type.is_file() {
            let dst = dst.join(entry.file_name());

            if dst.exists() {
                remove_file(&dst)?;
            }

            fs::copy(entry.path(), dst)?;
        } else if file_type.is_dir() {
            let dst = dst.join(entry.file_name());

            if dst.exists() {
                remove_dir_all(&dst)?;
            }

            create_dir(&dst)?;
            copy_recursive(&entry.path(), &dst)?;
        }
    }

    Ok(())
}

use anyhow::{anyhow, Context, Result};
use bzip2::read::BzDecoder;
use std::{
    fs::{create_dir_all, File},
    io::copy,
    path::PathBuf
};
use tar::Archive;
use urlencoding::decode;

const CEF_URL: &str = "https://cef-builds.spotifycdn.com/cef_binary_121.3.13%2Bg5c4a81b%2Bchromium-121.0.6167.184_linux64_minimal.tar.bz2";

fn main() -> Result<()> {
    let artifacts_dir = std::env::var("CEF_UI_ARTIFACTS_DIR").unwrap_or(String::from("artifacts"));
    let url = String::from(CEF_URL);

    let filename = decode(CEF_URL)
        .map_err(|e| anyhow!(e))
        .context("Failed to decode URL.")?
        .into_owned()
        .split('/')
        .last()
        .map(PathBuf::from)
        .ok_or_else(|| anyhow!("Failed to extract file name from URL."))?;

    let filestem = filename
        .file_stem()
        .map(PathBuf::from)
        .ok_or_else(|| anyhow!("Failed to extract file stem from URL."))?
        .file_stem()
        .map(PathBuf::from)
        .ok_or_else(|| anyhow!("Failed to extract file stem from URL."))?;

    let download_path = PathBuf::from(artifacts_dir.clone()).join(filename.clone());
    let extracted_path = PathBuf::from(artifacts_dir.clone()).join(filestem);

    println!("cargo:warning=      filename: {:?}", filename);
    println!("cargo:warning= download_path: {:?}", download_path);
    println!("cargo:warning=extracted_path: {:?}", extracted_path);

    // Make sure the artifacts directory exists.
    create_dir_all(&artifacts_dir)
        .map_err(|e| anyhow!(e))
        .context("Failed to create artifacts directory.")?;

    // Download the file if it doesn't exist.
    if !download_path.exists() {
        println!("cargo:warning=Downloading file...");

        download_file(&url, &download_path)
            .map_err(|e| anyhow!(e))
            .context("Failed to download file.")?;
    }

    // Extract the file if it hasn't been extracted.
    if !extracted_path.exists() {
        println!("cargo:warning=Extracting file...");

        extract_file(&download_path, &PathBuf::from(artifacts_dir))
            .map_err(|e| anyhow!(e))
            .context("Failed to extract file.")?;
    }

    Ok(())
}

fn download_file(url: &String, path: &PathBuf) -> Result<()> {
    let response = ureq::get(url.as_str()).call()?;

    let mut file = File::create(path)?;
    let mut reader = response.into_reader();

    copy(&mut reader, &mut file)?;

    Ok(())
}

fn extract_file(download_path: &PathBuf, extracted_path: &PathBuf) -> Result<()> {
    let file = File::open(download_path)?;
    let decompressed = BzDecoder::new(file);
    let mut archive = Archive::new(decompressed);

    archive.unpack(extracted_path)?;

    Ok(())
}

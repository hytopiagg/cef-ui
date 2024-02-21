use anyhow::{anyhow, Context, Result};
use bindgen::Builder;
use bzip2::read::BzDecoder;
use std::{
    fs::{create_dir_all, File},
    io::{copy, Write},
    path::PathBuf
};
use tar::Archive;
use urlencoding::decode;
use walkdir::WalkDir;

//const CEF_URL: &str = "https://cef-builds.spotifycdn.com/cef_binary_121.3.13%2Bg5c4a81b%2Bchromium-121.0.6167.184_linux64_minimal.tar.bz2";
const CEF_URL: &str = "https://cef-builds.spotifycdn.com/cef_binary_121.3.13%2Bg5c4a81b%2Bchromium-121.0.6167.184_macosarm64_minimal.tar.bz2";

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
    let include_path = extracted_path
        .join("include")
        .join("capi");
    let everything_header_path = extracted_path.join("everything.h");
    let bindings_path = extracted_path.join("bindings.rs");

    println!("cargo:warning=filename:               {:?}", filename);
    println!("cargo:warning=download_path:          {:?}", download_path);
    println!("cargo:warning=extracted_path:         {:?}", extracted_path);
    println!("cargo:warning=include_path:           {:?}", include_path);
    println!(
        "cargo:warning=everything_header_path: {:?}",
        everything_header_path
    );
    println!("cargo:warning=bindings_path:          {:?}", bindings_path);

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

    // TODO: Strip debug symbols on macOS and Linux!

    // Build the everything header if it doesn't exist.
    //if !everything_header_path.exists() {
    {
        println!("cargo:warning=Building everything header...");

        build_everything_header(&include_path, &everything_header_path)
            .map_err(|e| anyhow!(e))
            .context("Failed to build everything header.")?;
    }

    //if !bindings_path.exists() {
    {
        println!("cargo:warning=Generating bindings...");

        generate_bindings(&extracted_path, &everything_header_path, &bindings_path)
            .map_err(|e| anyhow!("Failed to generate bindings: {}", e))?;
        //.context("Failed to generate bindings.")?;
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

fn build_everything_header(include_path: &PathBuf, everything_header_path: &PathBuf) -> Result<()> {
    // Build the list of header files.
    let headers: Vec<PathBuf> = WalkDir::new(include_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.into_path())
        .filter(|path| {
            path.extension()
                .map_or(false, |ext| ext == "h")
                && !path.components().any(|component| {
                    component
                        .as_os_str()
                        .to_str()
                        .map_or(false, |s| {
                            s == "test"
                            // || s == "internal"
                            // || s == "capi"
                            // || s == "base"
                            // || s == "views"
                            // || s == "wrapper"
                        })
                })
        })
        .collect();

    let mut file = File::create(&everything_header_path)?;

    // Generate a header that includes all the other headers.
    for path in headers {
        let relative_path = path.strip_prefix(include_path)?;
        let include_str = format!("{}", relative_path.display()).replace("\\", "/");

        // TODO: Fix this!
        writeln!(file, "#include \"include/capi/{}\"", include_str)?;
    }

    Ok(())
}

fn generate_bindings(
    extracted_path: &PathBuf,
    everything_header_path: &PathBuf,
    bindings_path: &PathBuf
) -> Result<()> {
    let header_path = everything_header_path
        .to_str()
        .ok_or_else(|| anyhow!("Failed to convert everything header path to string."))?;

    let a = extracted_path.canonicalize()?;
    let b = extracted_path
        .join("include")
        .join("capi")
        .canonicalize()?;

    let a = a
        .to_str()
        .ok_or_else(|| anyhow!("Failed to convert include path to string."))?;
    let b = b
        .to_str()
        .ok_or_else(|| anyhow!("Failed to convert include path to string."))?;

    println!("cargo:warning=INCLUDE_PATH: {}", a);
    println!("cargo:warning=INCLUDE_PATH: {}", b);

    let bindings = Builder::default()
        .header(header_path)
        .layout_tests(false)
        .clang_args(vec![
            "-I",
            &a,
            "-I",
            &b,
            // "-isystem",
            // "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/c++/v1",
            "-isystem",
            "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include",
            // "--std=c++17",
            // "-x",
            // "c++",
        ])
        .generate()?;

    bindings.write_to_file(bindings_path)?;

    Ok(())
}

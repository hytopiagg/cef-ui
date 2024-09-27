use crate::{
    copy_files, create_tar_gz, download_file, extract_bz2, get_cef_artifacts_dir,
    get_cef_workspace_dir, get_url_filename
};
use anyhow::Result;
use bindgen::{builder, EnumVariation};
use std::{
    env::consts::{ARCH, OS},
    fs::{self, canonicalize, create_dir_all, remove_dir_all, rename},
    path::Path,
    process::{Command, Stdio}
};
use tracing::info;
use walkdir::WalkDir;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
const CEF_URL: &str = "https://cef-builds.spotifycdn.com/cef_binary_121.3.15%2Bg4d3b0b4%2Bchromium-121.0.6167.184_linux64_minimal.tar.bz2";

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const CEF_URL: &str = "https://cef-builds.spotifycdn.com/cef_binary_121.3.15%2Bg4d3b0b4%2Bchromium-121.0.6167.184_macosarm64_minimal.tar.bz2";

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
const CEF_URL: &str = "https://cef-builds.spotifycdn.com/cef_binary_121.3.15%2Bg4d3b0b4%2Bchromium-121.0.6167.184_windows64_minimal.tar.bz2";

/// Try and generate CEF artifacts.
pub struct ArtifactsCommand;

impl ArtifactsCommand {
    pub fn run(&self) -> Result<()> {
        let workspace_dir = get_cef_workspace_dir()?;
        let artifacts_dir = get_cef_artifacts_dir()?;

        // Create the artifacts/ directory.
        info!("Creating artifacts dir ..");

        if artifacts_dir.exists() {
            remove_dir_all(&artifacts_dir)?;
        }

        create_dir_all(&artifacts_dir)?;

        // Download CEF.
        info!("Downloading CEF ..");

        let filename = get_url_filename(CEF_URL)?;

        download_file(CEF_URL, &artifacts_dir.join(&filename))?;

        // Extract CEF.
        info!("Extracting CEF ..");

        extract_bz2(&artifacts_dir.join(&filename), &artifacts_dir)?;

        // Generate bindings.
        info!("Generating bindings ..");

        let extracted_dir = filename
            .strip_suffix(".tar.bz2")
            .unwrap();
        let extracted_dir = Path::new(&artifacts_dir).join(extracted_dir);
        let extracted_dir = canonicalize(&extracted_dir)?;
        let bindings_file = extracted_dir.join("bindings.rs");

        Self::generate_bindings(&extracted_dir, &bindings_file)?;

        // Strip debug symbols.
        info!("Stripping debug symbols ..");

        Self::strip_debug_symbols(&extracted_dir)?;

        // Create artifacts.
        info!("Creating artifacts ..");

        Self::create_artifacts(&artifacts_dir, &extracted_dir)?;

        // Copy bindings.
        info!("Copying bindings ..");

        let dst = workspace_dir.join(format!("crates/cef-ui-sys/src/{}_{}/bindings.rs", OS, ARCH));

        fs::copy(bindings_file, dst)?;

        info!("Done!");

        Ok(())
    }

    /// Creates the bindgen header file.
    fn create_header(include_dir: &Path) -> Result<String> {
        let mut header = String::new();
        let mut add = |path: &str| header.push_str(format!("#include \"{}\"\n", path).as_str());

        // All headers in the include/capi directory.
        let walk = WalkDir::new(include_dir.join("capi"))
            .into_iter()
            .filter_entry(|e| {
                e.path()
                    .components()
                    .all(|c| c.as_os_str() != "test")
            })
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file());

        for entry in walk {
            let path = entry
                .path()
                .strip_prefix(include_dir)
                .unwrap();

            let path = Path::new("include").join(path);

            add(path.to_str().unwrap());
        }

        // Manually included headers.
        add("include/cef_version.h");
        add("include/internal/cef_logging_internal.h");
        add("include/internal/cef_trace_event_internal.h");

        // macOS-specific headers.
        if cfg!(target_os = "macos") {
            add("include/internal/cef_types_mac.h");
            add("include/cef_sandbox_mac.h");
        }

        // Windows-specific headers.
        if cfg!(target_os = "windows") {
            add("include/internal/cef_types_win.h");
            add("include/cef_sandbox_win.h");
        }

        Ok(header)
    }

    /// Generate the cef bindings.
    fn generate_bindings(extracted_dir: &Path, bindings_file: &Path) -> Result<()> {
        let include_dir = extracted_dir.join("include");
        let header = Self::create_header(&include_dir)?;
        let inc = |path: &Path| {
            let mut path = path.to_string_lossy().to_string();

            // This is so gross, on Windows a prefix is added for paths longer than
            // 256 characters and this breaks Rust bindgen, hence this hack. :^/
            if cfg!(target_os = "windows") {
                if let Some(modified) = path.strip_prefix(r#"\\?\"#) {
                    path = modified.to_string();
                }
            }

            return format!("-I{}", path);
        };

        // Generate the bindings.
        let bindings = builder()
            .header_contents("everything.h", &header)
            .layout_tests(false)
            .generate_comments(false)
            .default_enum_style(EnumVariation::Rust {
                non_exhaustive: false
            })
            .constified_enum("cef_event_flags_t")
            .constified_enum("cef_touch_handle_state_flags_t")
            .constified_enum("cef_drag_operations_mask_t")
            .constified_enum("cef_cert_status_t")
            .constified_enum("cef_urlrequest_flags_t")
            .constified_enum("cef_context_menu_type_flags_t")
            .constified_enum("cef_context_menu_media_state_flags_t")
            .constified_enum("cef_context_menu_edit_state_flags_t")
            .constified_enum("cef_quick_menu_edit_state_flags_t")
            .raw_line("#![allow(non_camel_case_types)]")
            .raw_line("#![allow(non_upper_case_globals)]")
            .raw_line("#![allow(non_snake_case)]")
            .raw_line("#![allow(dead_code)]")
            .clang_args(&[
                inc(&include_dir),
                inc(&include_dir.join("capi")),
                inc(&extracted_dir)
            ])
            .generate()?;

        bindings.write_to_file(bindings_file)?;

        Ok(())
    }

    /// Strip debug symbols from binaries.
    fn strip_debug_symbols(extracted_dir: &Path) -> Result<()> {
        if cfg!(target_os = "linux") {
            let extracted_dir = extracted_dir
                .to_string_lossy()
                .to_string();
            let strip = |value| -> Result<()> {
                let value = extracted_dir.clone() + value;

                Command::new("strip")
                    .arg(value)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output()?;

                Ok(())
            };

            strip("/Release/chrome-sandbox")?;
            strip("/Release/libcef.so")?;
            strip("/Release/libEGL.so")?;
            strip("/Release/libGLESv2.so")?;
            strip("/Release/libvk_swiftshader.so")?;
            strip("/Release/libvulkan.so.1")?;
        }

        Ok(())
    }

    /// Create the final artifact.
    fn create_artifacts(artifacts_dir: &Path, extracted_dir: &Path) -> Result<()> {
        let cef_dir = artifacts_dir.join("cef");

        // Copy files for Linux and Windows.
        if cfg!(target_os = "linux") {
            copy_files(&extracted_dir.join("Release"), &cef_dir)?;
            copy_files(&extracted_dir.join("Resources"), &cef_dir)?;
        }

        // Copy files for macOS.
        if cfg!(target_os = "macos") {
            copy_files(&extracted_dir.join("Release"), &cef_dir)?;
            rename(
                &cef_dir.join("cef_sandbox.a"),
                &cef_dir.join("libcef_sandbox.a")
            )?;
        }

        // Copy files for Windows.
        if cfg!(target_os = "windows") {
            copy_files(&extracted_dir.join("Release"), &cef_dir)?;
            copy_files(&extracted_dir.join("Resources"), &cef_dir)?;
        }

        // Create the tar gzipped file.
        let filename = format!("cef-{}-{}.tgz", OS, ARCH);

        create_tar_gz(&artifacts_dir.join(filename), &cef_dir)?;

        Ok(())
    }
}

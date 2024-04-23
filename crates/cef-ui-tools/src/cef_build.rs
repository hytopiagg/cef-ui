use crate::{copy_files, get_target_dir};
use anyhow::Result;
use clap::Parser;
use log::info;
use std::{
    env::current_dir,
    fs::{copy, create_dir_all, remove_dir_all},
    process::{Command, Stdio}
};
use tracing::{level_filters::LevelFilter, subscriber::set_global_default, Level};
use tracing_log::LogTracer;
use tracing_subscriber::FmtSubscriber;

/// Command line arguments.
#[derive(Parser, Default)]
struct BuildArgs {
    /// Whether this is a release build.
    #[arg(long, default_value_t = false)]
    pub release: bool
}

/// Build the project.
pub fn cef_build() -> Result<()> {
    // This routes log macros through tracing.
    LogTracer::init()?;

    // Setup the tracing subscriber globally.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(LevelFilter::from_level(Level::INFO))
        .finish();

    set_global_default(subscriber)?;

    info!("Building project ..");

    let args = BuildArgs::parse();

    // Build the main executable.
    build_exe(args.release, "cef-ui-simple")?;

    // On macOS, build the helper executable
    // and package the app bundle as required.
    if cfg!(target_os = "macos") {
        info!("Building helper ..");

        build_exe(args.release, "cef-ui-simple-helper")?;

        info!("Building app bundle ..");

        build_app_bundle(args.release)?;
    }

    // On Windows, we have to copy all the
    // CEF files to the target directory.
    if cfg!(target_os = "windows") {
        info!("Copying CEF files ..");

        copy_cef_to_target(args.release)?;
    }

    Ok(())
}

/// Build a specific executable.
fn build_exe(release: bool, name: &str) -> Result<()> {
    let mut args = vec!["build", "--bin", name];

    if release {
        args.push("--release");
    }

    Command::new("cargo")
        .args(&args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(())
}

/// Package the app bundle on macOS.
fn build_app_bundle(release: bool) -> Result<()> {
    let cwd = current_dir()?;
    let target_dir = get_target_dir(release)?;
    let app_dir = target_dir.join("cef-ui-simple.app");
    let resources_dir = cwd.join("resources/macos");

    // Remove any existing app.
    if app_dir.exists() {
        remove_dir_all(&app_dir)?;
    }

    // Create main bundle folders.
    create_dir_all(app_dir.clone())?;
    create_dir_all(app_dir.join("Contents/Frameworks"))?;
    create_dir_all(app_dir.join("Contents/MacOS"))?;
    create_dir_all(app_dir.join("Contents/Resources"))?;

    // Copy main bundle files.
    copy(
        resources_dir.join("Info.plist"),
        app_dir.join("Contents/Info.plist")
    )?;

    copy(
        resources_dir.join("Icon.icns"),
        app_dir.join("Contents/Resources/Icon.icns")
    )?;

    copy_files(
        &resources_dir.join("English.lproj"),
        &app_dir.join("Contents/Resources/English.lproj")
    )?;

    copy(
        target_dir.join("cef-ui-simple"),
        app_dir.join("Contents/MacOS/cef-ui-simple")
    )?;

    // Copy the CEF framework.
    copy_files(
        &cwd.join("artifacts/cef/Chromium Embedded Framework.framework"),
        &app_dir.join("Contents/Frameworks/Chromium Embedded Framework.framework")
    )?;

    let create_helper = |name: Option<&str>| -> Result<()> {
        let helper_name = match name {
            Some(name) => format!("cef-ui-simple Helper ({})", name),
            None => "cef-ui-simple Helper".to_string()
        };

        let helper_dir = app_dir.join(format!("Contents/Frameworks/{}.app", helper_name));

        // Create helper bundle folders.
        create_dir_all(helper_dir.clone())?;
        create_dir_all(helper_dir.join("Contents/MacOS"))?;

        // Copy helper bundle files.
        let plist_name = match name {
            Some(name) => format!("{}HelperInfo.plist", name),
            None => "HelperInfo.plist".to_string()
        };

        copy(
            resources_dir.join(plist_name),
            helper_dir.join("Contents/Info.plist")
        )?;

        copy(
            target_dir.join("cef-ui-simple-helper"),
            helper_dir
                .join("Contents/MacOS")
                .join(helper_name)
        )?;

        Ok(())
    };

    // Create the helper bundles.
    create_helper(None)?;
    create_helper(Some("Alerts"))?;
    create_helper(Some("GPU"))?;
    create_helper(Some("Plugin"))?;
    create_helper(Some("Renderer"))?;

    Ok(())
}

/// Copy the CEF files to the target directory on Windows.
fn copy_cef_to_target(release: bool) -> Result<()> {
    let cwd = current_dir()?;
    let target_dir = get_target_dir(release)?;

    // Copy the CEF framework.
    copy_files(&cwd.join("artifacts/cef"), &target_dir)?;

    Ok(())
}

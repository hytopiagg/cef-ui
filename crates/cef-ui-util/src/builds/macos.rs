use crate::{copy_files, get_tool_target_dir, get_tool_workspace_dir};
use anyhow::Result;
use std::fs::{copy, create_dir_all, remove_dir_all};

/// Package the app bundle on macOS.
pub fn build_app_bundle(profile: &str) -> Result<()> {
    let workspace_dir = get_tool_workspace_dir()?;
    let target_dir = get_tool_target_dir(profile)?;
    let app_dir = target_dir.join("cef-ui-simple.app");
    let resources_dir = workspace_dir.join("resources/macos");

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
        &workspace_dir.join("artifacts/cef/Chromium Embedded Framework.framework"),
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

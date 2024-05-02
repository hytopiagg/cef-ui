use anyhow::Result;
use cef_ui_util::{get_tool_workspace_dir, AppBundleSettings, BuildSettings};

fn main() -> Result<()> {
    let workspace_dir = get_tool_workspace_dir()?;

    BuildSettings {
        package_main:        String::from("cef-ui-simple"),
        package_helper:      String::from("cef-ui-simple-helper"),
        app_bundle_settings: AppBundleSettings {
            artifacts_dir:   workspace_dir.join("artifacts"),
            app_name:        String::from("cef-ui-simple"),
            main_exe_name:   String::from("cef-ui-simple"),
            helper_exe_name: String::from("cef-ui-simple-helper"),
            resources_dir:   workspace_dir.join("resources/macos"),
            org_name:        String::from("hytopia")
        }
    }
    .run()?;

    Ok(())
}

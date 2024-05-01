use anyhow::Result;
use cef_ui_util::{cef_artifacts, get_tool_workspace_dir};

fn main() -> Result<()> {
    let workspace_dir = get_tool_workspace_dir()?;
    let artifacts_dir = workspace_dir.join("artifacts");

    cef_artifacts(&artifacts_dir)?;

    Ok(())
}

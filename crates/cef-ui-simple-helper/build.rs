use anyhow::Result;
use cef_ui_util::{get_build_rs_workspace_dir, link_cef_helper};

fn main() -> Result<()> {
    let workspace_dir = get_build_rs_workspace_dir()?;
    let artifacts_dir = workspace_dir.join("artifacts");

    link_cef_helper(&artifacts_dir)?;

    Ok(())
}

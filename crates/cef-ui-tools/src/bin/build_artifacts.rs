use anyhow::Result;
use cef_ui_tools::build_artifacts;

fn main() -> Result<()> {
    build_artifacts()?;

    Ok(())
}

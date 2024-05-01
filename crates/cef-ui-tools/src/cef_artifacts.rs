use anyhow::Result;
use cef_ui_util::cef_artifacts;

fn main() -> Result<()> {
    cef_artifacts()?;

    Ok(())
}

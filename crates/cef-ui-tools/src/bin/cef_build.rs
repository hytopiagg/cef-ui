use anyhow::Result;
use cef_ui_tools::cef_build;

fn main() -> Result<()> {
    cef_build()?;

    Ok(())
}
use anyhow::Result;
use cef_ui_util::cef_build;

fn main() -> Result<()> {
    cef_build()?;

    Ok(())
}

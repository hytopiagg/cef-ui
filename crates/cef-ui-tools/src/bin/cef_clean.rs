use anyhow::Result;
use cef_ui_tools::cef_clean;

fn main() -> Result<()> {
    cef_clean()?;

    Ok(())
}

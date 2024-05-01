use anyhow::Result;
use cef_ui_util::cef_clean;

fn main() -> Result<()> {
    cef_clean()?;

    Ok(())
}

use anyhow::Result;
use cef_ui_tools::clean_all;

fn main() -> Result<()> {
    clean_all()?;

    Ok(())
}

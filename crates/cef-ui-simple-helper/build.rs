use anyhow::Result;
use cef_ui_util::link_cef_helper;

fn main() -> Result<()> {
    link_cef_helper()?;

    Ok(())
}

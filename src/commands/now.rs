use anyhow::Result;

use crate::display;
use crate::spotify::applescript;

pub fn run() -> Result<()> {
    let info = applescript::get_now_playing()?;
    display::print_now_playing(&info);
    Ok(())
}

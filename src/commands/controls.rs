use anyhow::Result;
use console::Style;

use crate::spotify::applescript;

pub fn pause() -> Result<()> {
    applescript::pause()?;
    let dim = Style::new().dim();
    println!("  {} Paused", dim.apply_to("⏸"));
    Ok(())
}

pub fn resume() -> Result<()> {
    applescript::resume()?;
    let dim = Style::new().dim();
    println!("  {} Resumed", dim.apply_to("▶"));
    Ok(())
}

pub fn toggle() -> Result<()> {
    applescript::toggle()?;
    Ok(())
}

pub fn next() -> Result<()> {
    applescript::next_track()?;
    // Small delay so Spotify updates the current track
    std::thread::sleep(std::time::Duration::from_millis(200));
    if let Ok(info) = applescript::get_now_playing() {
        let green = Style::new().green().bold();
        let dim = Style::new().dim();
        println!(
            "  {} {} {}",
            green.apply_to("⏭"),
            info.track_name,
            dim.apply_to(format!("— {}", info.artist))
        );
    }
    Ok(())
}

pub fn prev() -> Result<()> {
    applescript::prev_track()?;
    std::thread::sleep(std::time::Duration::from_millis(200));
    if let Ok(info) = applescript::get_now_playing() {
        let green = Style::new().green().bold();
        let dim = Style::new().dim();
        println!(
            "  {} {} {}",
            green.apply_to("⏮"),
            info.track_name,
            dim.apply_to(format!("— {}", info.artist))
        );
    }
    Ok(())
}

pub fn volume(level: Option<u32>) -> Result<()> {
    let dim = Style::new().dim();
    match level {
        Some(vol) => {
            let vol = vol.min(100) as i32;
            applescript::set_volume(vol)?;
            println!("  {} Volume: {}%", dim.apply_to("🔊"), vol);
        }
        None => {
            let vol = applescript::get_volume()?;
            println!("  {} Volume: {}%", dim.apply_to("🔊"), vol);
        }
    }
    Ok(())
}

pub fn shuffle(state: Option<String>) -> Result<()> {
    let dim = Style::new().dim();
    match state {
        Some(s) => {
            let on = matches!(s.to_lowercase().as_str(), "on" | "true" | "1");
            applescript::set_shuffle(on)?;
            let label = if on { "on" } else { "off" };
            println!("  {} Shuffle: {}", dim.apply_to("🔀"), label);
        }
        None => {
            let on = applescript::get_shuffle()?;
            let label = if on { "on" } else { "off" };
            println!("  {} Shuffle: {}", dim.apply_to("🔀"), label);
        }
    }
    Ok(())
}

pub fn repeat(state: Option<String>) -> Result<()> {
    let dim = Style::new().dim();
    match state {
        Some(s) => {
            let on = matches!(s.to_lowercase().as_str(), "on" | "true" | "1");
            applescript::set_repeat(on)?;
            let label = if on { "on" } else { "off" };
            println!("  {} Repeat: {}", dim.apply_to("🔁"), label);
        }
        None => {
            let on = applescript::get_repeat()?;
            let label = if on { "on" } else { "off" };
            println!("  {} Repeat: {}", dim.apply_to("🔁"), label);
        }
    }
    Ok(())
}

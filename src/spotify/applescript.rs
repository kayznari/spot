use anyhow::{Context, Result, bail};
use std::process::Command;

fn run_osascript(script: &str) -> Result<String> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .context("Failed to run osascript")?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("osascript failed: {}", stderr.trim());
    }
}

fn tell_spotify(command: &str) -> Result<String> {
    run_osascript(&format!("tell application \"Spotify\" to {command}"))
}

pub fn ensure_running() -> Result<()> {
    let running = run_osascript(
        "tell application \"System Events\" to (name of processes) contains \"Spotify\"",
    )?;
    if running != "true" {
        run_osascript("tell application \"Spotify\" to activate")?;
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
    Ok(())
}

pub fn play_uri(uri: &str) -> Result<()> {
    ensure_running()?;
    tell_spotify(&format!("play track \"{uri}\""))?;
    Ok(())
}

pub fn pause() -> Result<()> {
    tell_spotify("pause")?;
    Ok(())
}

pub fn resume() -> Result<()> {
    tell_spotify("play")?;
    Ok(())
}

pub fn toggle() -> Result<()> {
    tell_spotify("playpause")?;
    Ok(())
}

pub fn next_track() -> Result<()> {
    tell_spotify("next track")?;
    Ok(())
}

pub fn prev_track() -> Result<()> {
    tell_spotify("previous track")?;
    Ok(())
}

pub fn get_volume() -> Result<i32> {
    let vol = tell_spotify("sound volume")?;
    vol.parse::<i32>()
        .context("Failed to parse volume from Spotify")
}

pub fn set_volume(vol: i32) -> Result<()> {
    tell_spotify(&format!("set sound volume to {vol}"))?;
    Ok(())
}

pub fn get_shuffle() -> Result<bool> {
    let val = tell_spotify("shuffling")?;
    Ok(val == "true")
}

pub fn set_shuffle(on: bool) -> Result<()> {
    tell_spotify(&format!("set shuffling to {on}"))?;
    Ok(())
}

pub fn get_repeat() -> Result<bool> {
    let val = tell_spotify("repeating")?;
    Ok(val == "true")
}

pub fn set_repeat(on: bool) -> Result<()> {
    tell_spotify(&format!("set repeating to {on}"))?;
    Ok(())
}

#[allow(dead_code)]
pub struct NowPlayingInfo {
    pub track_name: String,
    pub artist: String,
    pub album: String,
    pub track_id: String,
    pub duration_ms: i64,
    pub position_ms: i64,
    pub is_playing: bool,
}

pub fn get_now_playing() -> Result<NowPlayingInfo> {
    let script = r#"
tell application "Spotify"
    if player state is stopped then
        return "STOPPED"
    end if
    set trackName to name of current track
    set artistName to artist of current track
    set albumName to album of current track
    set trackId to id of current track
    set trackDuration to duration of current track
    set trackPosition to player position
    set playState to player state as string
    return trackName & "|||" & artistName & "|||" & albumName & "|||" & trackId & "|||" & trackDuration & "|||" & trackPosition & "|||" & playState
end tell
"#;
    let output = run_osascript(script)?;

    if output == "STOPPED" {
        bail!("No track is currently playing");
    }

    let parts: Vec<&str> = output.split("|||").collect();
    if parts.len() < 7 {
        bail!("Unexpected output from Spotify: {output}");
    }

    let duration_ms = parts[4]
        .parse::<i64>()
        .unwrap_or(0);
    let position_secs = parts[5]
        .parse::<f64>()
        .unwrap_or(0.0);

    Ok(NowPlayingInfo {
        track_name: parts[0].to_string(),
        artist: parts[1].to_string(),
        album: parts[2].to_string(),
        track_id: parts[3].to_string(),
        duration_ms,
        position_ms: (position_secs * 1000.0) as i64,
        is_playing: parts[6] == "playing",
    })
}

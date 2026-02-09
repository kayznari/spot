use anyhow::Result;
use console::Style;

use crate::config::Config;
use crate::spotify::{api, applescript, auth};

#[derive(Debug, Clone, Copy)]
pub enum PlayMode {
    Track,
    Album,
    Artist,
    Playlist,
}

pub async fn run(query: &str, mode: PlayMode) -> Result<()> {
    // Check aliases first
    let config = Config::load()?;
    let resolved = config
        .aliases
        .as_ref()
        .and_then(|a| a.get(query))
        .cloned();
    let query = resolved.as_deref().unwrap_or(query);

    let token = auth::get_token(&config).await?;

    match mode {
        PlayMode::Track => {
            let results = api::search(&token, query, api::SearchType::Track, 5).await?;
            let track = results.first().ok_or_else(|| anyhow::anyhow!("No track found for \"{}\"", query))?;
            play_and_print(track)?;
            applescript::set_repeat(false)?;
            Ok(())
        }
        PlayMode::Album => {
            let results = api::search(&token, query, api::SearchType::Album, 5).await?;
            let album = results.first().ok_or_else(|| anyhow::anyhow!("No album found for \"{}\"", query))?;
            play_and_print(album)?;
            applescript::set_repeat(true)?;
            let dim = Style::new().dim();
            println!("  {} Repeat on", dim.apply_to("🔁"));
            Ok(())
        }
        PlayMode::Artist => {
            let results = api::search(&token, query, api::SearchType::Artist, 5).await?;
            let artist = results.first().ok_or_else(|| anyhow::anyhow!("No artist found for \"{}\"", query))?;
            play_and_print(artist)?;
            applescript::set_repeat(false)?;
            Ok(())
        }
        PlayMode::Playlist => {
            let results = api::search(&token, query, api::SearchType::Playlist, 5).await?;
            let playlist = results.first().ok_or_else(|| anyhow::anyhow!("No playlist found for \"{}\"", query))?;
            play_and_print(playlist)?;
            applescript::set_repeat(false)?;
            Ok(())
        }
    }
}

fn play_and_print(result: &api::SearchResult) -> Result<()> {
    applescript::play_uri(&result.uri)?;

    let green = Style::new().green().bold();
    let dim = Style::new().dim();
    println!(
        "  {} {} {}",
        green.apply_to("▶"),
        green.apply_to(&result.name),
        dim.apply_to(format!("— {}", result.detail)),
    );

    Ok(())
}

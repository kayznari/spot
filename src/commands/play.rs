use anyhow::{Result, bail};
use console::Style;

use crate::config::Config;
use crate::spotify::{api, applescript, auth};

pub async fn run(query: &str) -> Result<()> {
    // Check aliases first
    let config = Config::load()?;
    let resolved = config
        .aliases
        .as_ref()
        .and_then(|a| a.get(query))
        .cloned();
    let query = resolved.as_deref().unwrap_or(query);

    let token = auth::get_token(&config).await?;

    // Search tracks, albums, and artists concurrently
    let (tracks, albums, artists) = tokio::join!(
        api::search(&token, query, api::SearchType::Track, 5),
        api::search(&token, query, api::SearchType::Album, 5),
        api::search(&token, query, api::SearchType::Artist, 5),
    );

    let tracks = tracks?;
    let albums = albums?;
    let artists = artists?;

    let query_lower = query.to_lowercase();

    // Exact artist match → play artist (top songs)
    if let Some(artist) = artists.first() {
        if artist.name.to_lowercase() == query_lower {
            return play_and_print(artist);
        }
    }

    // Exact album match → play album + repeat
    if let Some(album) = albums.first() {
        if album.name.to_lowercase() == query_lower {
            play_and_print(album)?;
            applescript::set_repeat(true)?;
            let dim = Style::new().dim();
            println!("  {} Repeat on", dim.apply_to("🔁"));
            return Ok(());
        }
    }

    // Otherwise → play top track
    if let Some(track) = tracks.first() {
        return play_and_print(track);
    }

    // Fallback to whatever we got
    if let Some(artist) = artists.first() {
        return play_and_print(artist);
    }
    if let Some(album) = albums.first() {
        play_and_print(album)?;
        applescript::set_repeat(true)?;
        return Ok(());
    }

    bail!("No results found for \"{}\"", query);
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

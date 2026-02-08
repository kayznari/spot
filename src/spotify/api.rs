use anyhow::{Context, Result, bail};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub name: String,
    pub uri: String,
    pub detail: String,
}

impl std::fmt::Display for SearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} — {}", self.name, self.detail)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SearchType {
    Track,
    Album,
    Artist,
    Playlist,
}

impl SearchType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Track => "track",
            Self::Album => "album",
            Self::Artist => "artist",
            Self::Playlist => "playlist",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Track => "Tracks",
            Self::Album => "Albums",
            Self::Artist => "Artists",
            Self::Playlist => "Playlists",
        }
    }
}

#[derive(Deserialize)]
struct SearchResponse {
    tracks: Option<Paging<TrackItem>>,
    albums: Option<Paging<AlbumItem>>,
    artists: Option<Paging<ArtistItem>>,
    playlists: Option<Paging<PlaylistItem>>,
}

#[derive(Deserialize)]
struct Paging<T> {
    items: Vec<T>,
}

#[derive(Deserialize)]
struct TrackItem {
    name: String,
    uri: String,
    artists: Vec<ArtistRef>,
    album: AlbumRef,
}

#[derive(Deserialize)]
struct AlbumItem {
    name: String,
    uri: String,
    artists: Vec<ArtistRef>,
    release_date: Option<String>,
}

#[derive(Deserialize)]
struct ArtistItem {
    name: String,
    uri: String,
    genres: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct PlaylistItem {
    name: String,
    uri: String,
    owner: PlaylistOwner,
    tracks: PlaylistTracks,
}

#[derive(Deserialize)]
struct PlaylistOwner {
    display_name: Option<String>,
}

#[derive(Deserialize)]
struct PlaylistTracks {
    total: u32,
}

#[derive(Deserialize)]
struct ArtistRef {
    name: String,
}

#[derive(Deserialize)]
struct AlbumRef {
    name: String,
}

pub async fn search(
    token: &str,
    query: &str,
    search_type: SearchType,
    limit: u32,
) -> Result<Vec<SearchResult>> {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://api.spotify.com/v1/search")
        .bearer_auth(token)
        .query(&[
            ("q", query),
            ("type", search_type.as_str()),
            ("limit", &limit.to_string()),
        ])
        .send()
        .await
        .context("Failed to search Spotify")?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("Search failed ({status}): {body}");
    }

    let data: SearchResponse = resp.json().await.context("Failed to parse search response")?;

    let results = match search_type {
        SearchType::Track => data
            .tracks
            .map(|t| {
                t.items
                    .into_iter()
                    .map(|item| {
                        let artists = item
                            .artists
                            .iter()
                            .map(|a| a.name.as_str())
                            .collect::<Vec<_>>()
                            .join(", ");
                        SearchResult {
                            name: item.name,
                            uri: item.uri,
                            detail: format!("{artists} • {}", item.album.name),
                        }
                    })
                    .collect()
            })
            .unwrap_or_default(),

        SearchType::Album => data
            .albums
            .map(|a| {
                a.items
                    .into_iter()
                    .map(|item| {
                        let artists = item
                            .artists
                            .iter()
                            .map(|a| a.name.as_str())
                            .collect::<Vec<_>>()
                            .join(", ");
                        let year = item
                            .release_date
                            .as_deref()
                            .and_then(|d| d.split('-').next())
                            .unwrap_or("????");
                        SearchResult {
                            name: item.name,
                            uri: item.uri,
                            detail: format!("{artists} ({year})"),
                        }
                    })
                    .collect()
            })
            .unwrap_or_default(),

        SearchType::Artist => data
            .artists
            .map(|a| {
                a.items
                    .into_iter()
                    .map(|item| {
                        let genres = item
                            .genres
                            .as_deref()
                            .unwrap_or(&[])
                            .iter()
                            .take(3)
                            .map(|s| s.as_str())
                            .collect::<Vec<_>>()
                            .join(", ");
                        let detail = if genres.is_empty() {
                            "Artist".to_string()
                        } else {
                            genres
                        };
                        SearchResult {
                            name: item.name,
                            uri: item.uri,
                            detail,
                        }
                    })
                    .collect()
            })
            .unwrap_or_default(),

        SearchType::Playlist => data
            .playlists
            .map(|p| {
                p.items
                    .into_iter()
                    .map(|item| {
                        let owner = item
                            .owner
                            .display_name
                            .as_deref()
                            .unwrap_or("Unknown");
                        SearchResult {
                            name: item.name,
                            uri: item.uri,
                            detail: format!("by {owner} • {} tracks", item.tracks.total),
                        }
                    })
                    .collect()
            })
            .unwrap_or_default(),
    };

    Ok(results)
}

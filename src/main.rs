mod commands;
mod config;
mod display;
mod spotify;

use anyhow::Result;
use clap::{Parser, Subcommand};

use spotify::api::SearchType;

#[derive(Parser)]
#[command(name = "spot", about = "Spotify CLI for macOS", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Search and play a track, album, artist, or playlist
    Play {
        /// Search query (omit to resume playback)
        query: Vec<String>,

        /// Search albums
        #[arg(short = 'a', long)]
        album: bool,

        /// Search artists
        #[arg(short = 'r', long)]
        artist: bool,

        /// Search playlists
        #[arg(short = 'p', long)]
        playlist: bool,
    },

    /// Search and display results
    Search {
        /// Search query
        query: Vec<String>,

        /// Search albums
        #[arg(short = 'a', long)]
        album: bool,

        /// Search artists
        #[arg(short = 'r', long)]
        artist: bool,

        /// Search playlists
        #[arg(short = 'p', long)]
        playlist: bool,
    },

    /// Show currently playing track
    Now,

    /// Pause playback
    Pause,

    /// Resume playback
    Resume,

    /// Toggle play/pause
    Toggle,

    /// Skip to next track
    Next,

    /// Skip to previous track
    Prev,

    /// Get or set volume (0-100)
    Vol {
        /// Volume level (0-100)
        level: Option<u32>,
    },

    /// Get or set shuffle mode
    Shuffle {
        /// on/off
        state: Option<String>,
    },

    /// Get or set repeat mode
    Repeat {
        /// on/off
        state: Option<String>,
    },

    /// Set up Spotify API credentials
    Auth {
        /// Check credential status
        #[arg(long)]
        status: bool,
    },
}

fn resolve_search_type(album: bool, artist: bool, playlist: bool) -> SearchType {
    if album {
        SearchType::Album
    } else if artist {
        SearchType::Artist
    } else if playlist {
        SearchType::Playlist
    } else {
        SearchType::Track
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Play {
            query,
            album,
            artist,
            playlist,
        } => {
            let search_type = resolve_search_type(album, artist, playlist);
            let query_str = if query.is_empty() {
                None
            } else {
                Some(query.join(" "))
            };
            commands::play::run(query_str, search_type).await?;
        }

        Command::Search {
            query,
            album,
            artist,
            playlist,
        } => {
            let search_type = resolve_search_type(album, artist, playlist);
            let query_str = query.join(" ");
            if query_str.is_empty() {
                anyhow::bail!("Search query is required. Usage: spot search <query>");
            }
            commands::search::run(&query_str, search_type).await?;
        }

        Command::Now => commands::now::run()?,
        Command::Pause => commands::controls::pause()?,
        Command::Resume => commands::controls::resume()?,
        Command::Toggle => commands::controls::toggle()?,
        Command::Next => commands::controls::next()?,
        Command::Prev => commands::controls::prev()?,
        Command::Vol { level } => commands::controls::volume(level)?,
        Command::Shuffle { state } => commands::controls::shuffle(state)?,
        Command::Repeat { state } => commands::controls::repeat(state)?,
        Command::Auth { status } => commands::auth::run(status).await?,
    }

    Ok(())
}

# spot

A Spotify CLI for macOS. Search and play music from your terminal.

## Quick Start

```bash
# Clone and install
git clone https://github.com/kayznari/spot.git
cd spot
cargo install --path .
```

Requires **Rust 1.83+** and **macOS** with the Spotify desktop app installed.

## Setup

1. Create a Spotify app at [developer.spotify.com/dashboard](https://developer.spotify.com/dashboard)
2. Copy your Client ID and Client Secret
3. Run:

```bash
spot auth
```

## Usage

```bash
# Play (defaults to track, use flags to pick type)
spot "bohemian rhapsody"              # plays top track
spot -a "abbey road"                  # plays album
spot -r "kendrick lamar"              # plays artist
spot -p "chill vibes"                 # plays playlist

# Search
spot search "bohemian rhapsody"
spot search -a "abbey road"       # albums
spot search -r "kendrick lamar"   # artists
spot search -p "chill vibes"      # playlists

# Controls
spot now          # what's playing
spot pause
spot resume
spot toggle
spot next
spot prev
spot vol 75       # volume 0-100
spot shuffle on
spot repeat on

# Aliases (shortcuts for frequent searches)
spot alias add chill "lo-fi beats"
spot alias ls
spot chill        # plays "lo-fi beats"
```

## How It Works

- **Search** uses the Spotify Web API (no Premium required)
- **Playback** uses AppleScript to control the Spotify desktop app
- Controls like pause/next/vol are pure AppleScript (no API credentials needed)

## License

MIT

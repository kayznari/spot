# Spot — Spotify CLI for macOS

## Overview

A Rust CLI that controls Spotify from the terminal. Search + playback via Spotify Web API + AppleScript.

## Architecture

```
src/
├── main.rs                 CLI (clap) + dispatch. Uses external_subcommand so `spot <query>` works without a subcommand.
├── config.rs               ~/.config/spot/config.toml — credentials + aliases (HashMap<String, String>)
├── display.rs              Formatting: track display, progress bar, duration
├── commands/
│   ├── play.rs             Smart play: searches tracks/albums/artists concurrently, exact-matches to pick type, then plays
│   ├── search.rs           Search + display (supports -a/-r/-p flags)
│   ├── controls.rs         pause/resume/toggle/next/prev/vol/shuffle/repeat — thin wrappers around AppleScript
│   ├── now.rs              Now playing + progress bar
│   ├── auth.rs             Interactive credential setup + validation
│   └── alias.rs            Add/remove/list aliases in config
└── spotify/
    ├── api.rs              Spotify Web API search — returns Vec<SearchResult> with name/uri/detail
    ├── auth.rs             Client Credentials token fetch + file cache (~/.config/spot/token.json, 1hr TTL)
    └── applescript.rs      osascript wrappers (play URI, pause, resume, now playing, etc.)
```

## How It Works

- **Search**: Spotify Web API with Client Credentials flow (client_id + client_secret, no user login)
- **Playback**: AppleScript `play track "spotify:track:ID"` — doesn't steal focus, works without Premium
- **Controls** (pause/next/etc.): Pure AppleScript, no API credentials needed
- **Smart play** (`spot <query>`): Searches all types concurrently, exact name match picks type:
  1. Artist match → plays artist (top songs)
  2. Album match → plays album + repeat on
  3. Otherwise → plays top track
- **Aliases**: Stored in config.toml under `[aliases]`, resolved before search in play.rs

## Commands

```
spot <query>               Smart play (artist/album/track auto-detected)
spot search <query>        Search and display results (-a album, -r artist, -p playlist)
spot now                   Currently playing + progress bar
spot pause / resume / toggle
spot next / prev
spot vol [0-100]
spot shuffle [on|off]
spot repeat [on|off]
spot auth                  Set up credentials
spot auth --status         Check credentials
spot alias add <name> <q>  Add alias (e.g. spot alias add white-album "The Beatles")
spot alias rm <name>       Remove alias
spot alias ls              List aliases
```

## Build & Install

```
cargo install --path .
```

Requires Rust 1.83+. Binary installs to ~/.cargo/bin/spot.

## Config

Located at `~/.config/spot/config.toml`:

```toml
client_id = "..."
client_secret = "..."

[aliases]
white-album = "The Beatles"
dsotm = "The Dark Side of the Moon"
```

Token cache at `~/.config/spot/token.json` (auto-refreshed).

## Key Design Decisions

- **AppleScript `play track` over `open spotify:URI`** — doesn't steal focus
- **external_subcommand** — `spot radiohead` works without typing `spot play radiohead`
- **Concurrent search** — tokio::join! searches tracks/albums/artists in parallel
- **Token caching to file** — avoids HTTP round-trip on every command
- **Batched AppleScript for `now`** — single osascript call returns all track info
- **Aliases** — stored in config, resolved before search to handle nicknames

## Known Limitations

- Smart matching is exact (case-insensitive). `spot beatles` won't match "The Beatles" — use an alias.
- Client Credentials flow = no user-specific features (no liked songs, no private playlists).
- macOS only (AppleScript dependency).

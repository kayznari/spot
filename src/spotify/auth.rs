use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::Config;

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct CachedToken {
    access_token: String,
    expires_at: u64,
}

fn token_cache_path() -> Result<std::path::PathBuf> {
    Ok(Config::config_dir()?.join("token.json"))
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn load_cached_token() -> Option<String> {
    let path = token_cache_path().ok()?;
    let contents = fs::read_to_string(path).ok()?;
    let cached: CachedToken = serde_json::from_str(&contents).ok()?;
    if now_secs() < cached.expires_at.saturating_sub(60) {
        Some(cached.access_token)
    } else {
        None
    }
}

fn save_cached_token(token: &str, expires_in: u64) -> Result<()> {
    let path = token_cache_path()?;
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let cached = CachedToken {
        access_token: token.to_string(),
        expires_at: now_secs() + expires_in,
    };
    let json = serde_json::to_string(&cached)?;
    fs::write(path, json)?;
    Ok(())
}

pub async fn get_token(config: &Config) -> Result<String> {
    if let Some(token) = load_cached_token() {
        return Ok(token);
    }

    let client_id = config
        .client_id
        .as_deref()
        .context("No client_id configured. Run `spot auth` to set up credentials.")?;
    let client_secret = config
        .client_secret
        .as_deref()
        .context("No client_secret configured. Run `spot auth` to set up credentials.")?;

    let client = reqwest::Client::new();
    let resp = client
        .post("https://accounts.spotify.com/api/token")
        .form(&[("grant_type", "client_credentials")])
        .basic_auth(client_id, Some(client_secret))
        .send()
        .await
        .context("Failed to request access token from Spotify")?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("Token request failed ({status}): {body}. Check your credentials with `spot auth`.");
    }

    let token_resp: TokenResponse = resp
        .json()
        .await
        .context("Failed to parse token response")?;

    save_cached_token(&token_resp.access_token, token_resp.expires_in)?;

    Ok(token_resp.access_token)
}

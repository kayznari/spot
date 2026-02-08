use anyhow::Result;

use crate::config::Config;
use crate::display;
use crate::spotify::{api, auth};

pub async fn run(query: &str, search_type: api::SearchType) -> Result<()> {
    let config = Config::load()?;
    let token = auth::get_token(&config).await?;
    let results = api::search(&token, query, search_type, 20).await?;
    display::print_search_results(&results, search_type);
    Ok(())
}

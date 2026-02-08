use anyhow::{Result, bail};
use console::Style;
use dialoguer::FuzzySelect;

use crate::config::Config;
use crate::spotify::{api, applescript, auth};

pub async fn run(query: Option<String>, search_type: api::SearchType) -> Result<()> {
    let query = match query {
        Some(q) if !q.is_empty() => q,
        _ => {
            applescript::resume()?;
            let dim = Style::new().dim();
            println!("  {} Resumed playback", dim.apply_to("▶"));
            return Ok(());
        }
    };

    let config = Config::load()?;
    let token = auth::get_token(&config).await?;
    let results = api::search(&token, &query, search_type, 20).await?;

    if results.is_empty() {
        bail!("No {} found for \"{}\"", search_type.label().to_lowercase(), query);
    }

    let items: Vec<String> = results.iter().map(|r| r.to_string()).collect();

    let selection = FuzzySelect::new()
        .with_prompt(format!("  Select {}", search_type.as_str()))
        .items(&items)
        .default(0)
        .interact_opt()?;

    let selection = match selection {
        Some(i) => i,
        None => return Ok(()),
    };

    let chosen = &results[selection];
    applescript::play_uri(&chosen.uri)?;

    let green = Style::new().green().bold();
    let dim = Style::new().dim();
    println!(
        "\n  {} {} {}",
        green.apply_to("▶"),
        green.apply_to(&chosen.name),
        dim.apply_to(format!("— {}", chosen.detail)),
    );

    Ok(())
}

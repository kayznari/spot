use anyhow::Result;
use console::Style;
use std::collections::HashMap;

use crate::config::Config;

pub fn add(name: &str, query: &str) -> Result<()> {
    let mut config = Config::load()?;
    let aliases = config.aliases.get_or_insert_with(HashMap::new);
    aliases.insert(name.to_string(), query.to_string());
    config.save()?;

    let green = Style::new().green().bold();
    let dim = Style::new().dim();
    println!(
        "  {} {} {} {}",
        green.apply_to("✓"),
        green.apply_to(name),
        dim.apply_to("→"),
        query,
    );
    Ok(())
}

pub fn remove(name: &str) -> Result<()> {
    let mut config = Config::load()?;
    if let Some(aliases) = &mut config.aliases {
        aliases.remove(name);
    }
    config.save()?;

    let dim = Style::new().dim();
    println!("  {} Removed alias \"{}\"", dim.apply_to("✓"), name);
    Ok(())
}

pub fn list() -> Result<()> {
    let config = Config::load()?;
    let green = Style::new().green().bold();
    let dim = Style::new().dim();

    match &config.aliases {
        Some(aliases) if !aliases.is_empty() => {
            println!("\n  {}\n", green.apply_to("Aliases"));
            let mut sorted: Vec<_> = aliases.iter().collect();
            sorted.sort_by_key(|(k, _)| k.clone());
            for (name, query) in sorted {
                println!(
                    "  {} {} {}",
                    green.apply_to(name),
                    dim.apply_to("→"),
                    query,
                );
            }
            println!();
        }
        _ => {
            println!("  No aliases configured.");
            println!(
                "  Add one with: {}",
                dim.apply_to("spot alias add white-album \"The Beatles\"")
            );
        }
    }
    Ok(())
}

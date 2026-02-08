use anyhow::Result;
use console::Style;
use dialoguer::Input;

use crate::config::Config;
use crate::spotify;

pub async fn run(status: bool) -> Result<()> {
    let config = Config::load()?;

    if status {
        return show_status(&config).await;
    }

    setup_credentials().await
}

async fn show_status(config: &Config) -> Result<()> {
    let green = Style::new().green().bold();
    let red = Style::new().red().bold();
    let dim = Style::new().dim();

    if !config.has_credentials() {
        println!("{} No credentials configured.", red.apply_to("✗"));
        println!("  Run {} to set up.", green.apply_to("spot auth"));
        return Ok(());
    }

    let client_id = config.client_id.as_deref().unwrap_or("");
    let masked = if client_id.len() > 8 {
        format!("{}…{}", &client_id[..4], &client_id[client_id.len() - 4..])
    } else {
        "****".to_string()
    };

    println!("  Client ID: {}", dim.apply_to(&masked));

    match spotify::auth::get_token(&config).await {
        Ok(_) => println!("{} Credentials are valid.", green.apply_to("✓")),
        Err(e) => println!(
            "{} Credentials are invalid: {}",
            red.apply_to("✗"),
            e
        ),
    }

    Ok(())
}

async fn setup_credentials() -> Result<()> {
    let green = Style::new().green().bold();
    let dim = Style::new().dim();

    println!(
        "\n  {} Spotify API Credentials Setup\n",
        green.apply_to("♫")
    );
    println!(
        "  {}",
        dim.apply_to("Create an app at https://developer.spotify.com/dashboard")
    );
    println!(
        "  {}\n",
        dim.apply_to("to get your Client ID and Client Secret.")
    );

    let client_id: String = Input::new()
        .with_prompt("  Client ID")
        .interact_text()?;

    let client_secret: String = Input::new()
        .with_prompt("  Client Secret")
        .interact_text()?;

    let mut config = Config::load()?;
    config.client_id = Some(client_id.trim().to_string());
    config.client_secret = Some(client_secret.trim().to_string());

    print!("\n  Validating credentials... ");

    match spotify::auth::get_token(&config).await {
        Ok(_) => {
            config.save()?;
            println!("{}", green.apply_to("valid!"));
            println!(
                "\n  {} Credentials saved. You're ready to go!",
                green.apply_to("✓")
            );
        }
        Err(e) => {
            println!("failed!");
            println!("\n  Error: {e}");
            println!("  Credentials were NOT saved. Please try again.");
        }
    }

    Ok(())
}

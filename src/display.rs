use console::Style;

use crate::spotify::api::{SearchResult, SearchType};
use crate::spotify::applescript::NowPlayingInfo;

pub fn format_duration(ms: i64) -> String {
    let total_secs = ms / 1000;
    let mins = total_secs / 60;
    let secs = total_secs % 60;
    format!("{mins}:{secs:02}")
}

pub fn progress_bar(position_ms: i64, duration_ms: i64, width: usize) -> String {
    if duration_ms == 0 {
        return " ".repeat(width);
    }
    let ratio = (position_ms as f64 / duration_ms as f64).clamp(0.0, 1.0);
    let filled = (ratio * width as f64) as usize;
    let empty = width.saturating_sub(filled);
    format!("{}{}",
        "━".repeat(filled),
        "─".repeat(empty),
    )
}

pub fn print_now_playing(info: &NowPlayingInfo) {
    let green = Style::new().green().bold();
    let dim = Style::new().dim();
    let status = if info.is_playing { "▶" } else { "⏸" };

    println!(
        "{} {} {}",
        green.apply_to(status),
        green.apply_to(&info.track_name),
        dim.apply_to(format!("— {} • {}", info.artist, info.album))
    );

    let bar = progress_bar(info.position_ms, info.duration_ms, 30);
    let pos = format_duration(info.position_ms);
    let dur = format_duration(info.duration_ms);
    println!(
        "  {} {} {}",
        dim.apply_to(&pos),
        bar,
        dim.apply_to(&dur),
    );
}

pub fn print_search_results(results: &[SearchResult], search_type: SearchType) {
    let green = Style::new().green().bold();
    let dim = Style::new().dim();

    if results.is_empty() {
        println!("No {} found.", search_type.label().to_lowercase());
        return;
    }

    println!("{}\n", green.apply_to(format!("  {}", search_type.label())));

    for (i, result) in results.iter().enumerate() {
        println!(
            "  {} {} {}",
            dim.apply_to(format!("{:>2}.", i + 1)),
            result.name,
            dim.apply_to(format!("— {}", result.detail)),
        );
    }
}

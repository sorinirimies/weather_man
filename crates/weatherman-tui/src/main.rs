//! Command-line entry point for the terminal weather app.

use anyhow::Result;
use clap::Parser;
use weatherman_core::{load_report, Forecast, WeatherConfig};
use weatherman_tui::WeatherTui;

#[derive(Parser)]
#[command(
    name = "weatherman-tui",
    author,
    version,
    about = "A single-page terminal weather app (current + hourly + 7-day forecast)"
)]
struct Cli {
    /// Location to check weather for (default: auto-detect from IP)
    #[arg(short, long)]
    location: Option<String>,

    /// Units to display: metric, imperial, standard
    #[arg(short, long, default_value = "metric")]
    units: String,

    /// Output the forecast as JSON and exit (no TUI)
    #[arg(short, long, default_value_t = false)]
    json: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = WeatherConfig {
        units: cli.units.clone(),
        location: cli.location.clone(),
        json_output: cli.json,
        ..Default::default()
    };

    let report = load_report(&config, cli.location.as_deref()).await?;

    if cli.json {
        let forecast = Forecast {
            current: report.current,
            hourly: report.hourly,
            daily: report.daily,
            timezone_offset: 0,
            units: config.units.clone(),
        };
        println!("{}", serde_json::to_string_pretty(&forecast)?);
        return Ok(());
    }

    let mut tui = WeatherTui::new(report, config)?;
    tui.run()?;
    Ok(())
}

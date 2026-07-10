//! Command-line entry point for the terminal weather app.

use anyhow::Result;
use clap::Parser;
use weather_man_core::{LocationService, WeatherConfig, WeatherForecaster, WeatherProvider};
use weather_man_tui::WeatherTui;

#[derive(Parser)]
#[command(
    name = "weather_man-tui",
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

    let location_service = LocationService::new();
    let location = match &cli.location {
        Some(name) => location_service.get_location_by_name(name).await?,
        None => location_service.get_location_from_ip().await?,
    };

    let forecaster = WeatherForecaster::new(config.clone());
    let forecast = forecaster.forecast(&location).await?;

    if cli.json {
        println!("{}", serde_json::to_string_pretty(&forecast)?);
        return Ok(());
    }

    let mut tui = WeatherTui::new(forecast.hourly, forecast.daily, location, config)?;
    tui.run()?;
    Ok(())
}

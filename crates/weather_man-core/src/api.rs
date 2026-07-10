//! High-level orchestration: resolve a location and load its full forecast.
//!
//! This is the non-UI "do the whole thing" entry point shared by the GUI and
//! TUI (and available to any library consumer).

use crate::forecaster::{WeatherForecaster, WeatherProvider};
use crate::location::LocationService;
use crate::types::{CurrentWeather, DailyForecast, HourlyForecast, Location, WeatherConfig};
use anyhow::Result;

/// A fully-resolved weather report for a single location.
#[derive(Debug, Clone)]
pub struct WeatherReport {
    pub location: Location,
    pub current: Option<CurrentWeather>,
    pub hourly: Vec<HourlyForecast>,
    pub daily: Vec<DailyForecast>,
}

/// Resolve a location by name, or auto-detect from IP when `query` is `None`.
pub async fn resolve_location(query: Option<&str>) -> Result<Location> {
    let service = LocationService::new();
    match query {
        Some(name) => service.get_location_by_name(name).await,
        None => service.get_location_from_ip().await,
    }
}

/// Resolve a location and load its complete forecast in one call.
///
/// ```no_run
/// # async fn run() -> anyhow::Result<()> {
/// use weather_man_core::{load_report, WeatherConfig};
/// let report = load_report(&WeatherConfig::default(), Some("Berlin")).await?;
/// println!("{} — {} hours", report.location.name, report.hourly.len());
/// # Ok(())
/// # }
/// ```
pub async fn load_report(config: &WeatherConfig, query: Option<&str>) -> Result<WeatherReport> {
    let location = resolve_location(query).await?;
    let forecaster = WeatherForecaster::new(config.clone());
    let forecast = forecaster.forecast(&location).await?;

    Ok(WeatherReport {
        location,
        current: forecast.current,
        hourly: forecast.hourly,
        daily: forecast.daily,
    })
}

//! # weather_man-core
//!
//! Framework-free weather logic shared by the Iced GUI and Ratatui TUI, and
//! usable as a **drop-in weather API provider library** on its own.
//!
//! | Module | What lives here |
//! |--------|-----------------|
//! | [`types`] | Domain models — conditions, locations, current/hourly/daily forecasts, config |
//! | [`forecaster`] | [`WeatherProvider`] trait + Open-Meteo implementation ([`WeatherForecaster`]) |
//! | [`location`] | IP + name geocoding ([`LocationService`]) |
//! | [`format`] | UI-agnostic formatting helpers (units, wind, tones, local time) |
//!
//! This crate has NO GUI or TUI dependencies.
//!
//! ## Quick start
//!
//! ```no_run
//! use weather_man_core::{LocationService, WeatherForecaster, WeatherProvider, WeatherConfig};
//!
//! # async fn run() -> anyhow::Result<()> {
//! let location = LocationService::new().get_location_by_name("Berlin").await?;
//! let provider = WeatherForecaster::new(WeatherConfig::default());
//! let forecast = provider.forecast(&location).await?;
//! println!("{} days", forecast.daily.len());
//! # Ok(())
//! # }
//! ```

pub mod forecaster;
pub mod format;
pub mod location;
pub mod types;

// Convenience re-exports — the public surface most consumers need.
pub use forecaster::{
    weather_description_from_wmo, wmo_code_to_condition, WeatherForecaster, WeatherProvider,
};
pub use format::{
    condition_tone, convert_to_local, day_label, format_date_short, format_local_time, is_daytime,
    pop_percent, temp_unit_label, weekday_name, wind_direction_arrow, wind_direction_label,
    wind_unit_label, ConditionTone,
};
pub use location::LocationService;
pub use types::{
    CurrentWeather, DailyForecast, DetailLevel, Forecast, HourlyForecast, Location,
    WeatherCondition, WeatherConfig, WeatherDescription,
};

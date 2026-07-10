//! Demonstrate that `WeatherProvider` is a drop-in trait: implement it over any
//! backend (here, canned in-memory data) and reuse every core helper on top.
//!
//! Run:  cargo run -p weatherman-core --example custom_provider
//!
//! This example makes **no network calls** — handy for tests, demos and offline
//! development.

use chrono::{Duration, Utc};
use weatherman_core::{
    CurrentWeather, DailyForecast, Forecast, HourlyForecast, Location, WeatherCondition,
    WeatherConfig, WeatherDescription,
};
use weatherman_core::{ForecastView, WeatherProvider};

/// A provider that returns fixed, hand-written data instead of calling an API.
struct MockProvider;

impl WeatherProvider for MockProvider {
    async fn current(&self, _location: &Location) -> anyhow::Result<CurrentWeather> {
        Ok(sample_current())
    }
    async fn hourly(&self, _location: &Location) -> anyhow::Result<Vec<HourlyForecast>> {
        Ok(sample_hourly())
    }
    async fn daily(&self, _location: &Location) -> anyhow::Result<Vec<DailyForecast>> {
        Ok(sample_daily())
    }
    async fn forecast(&self, location: &Location) -> anyhow::Result<Forecast> {
        Ok(Forecast {
            current: Some(self.current(location).await?),
            hourly: self.hourly(location).await?,
            daily: self.daily(location).await?,
            timezone_offset: 0,
            units: "metric".to_string(),
        })
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let provider = MockProvider;
    let location = Location {
        name: "Demo City".into(),
        country: "Nowhere".into(),
        ..Location::default()
    };
    let config = WeatherConfig::default();

    let forecast = provider.forecast(&location).await?;
    let view = ForecastView::build(
        forecast.current.as_ref(),
        &forecast.hourly,
        &forecast.daily,
        &location,
        &config,
    );

    println!("⛅ Custom provider demo — no network used\n");
    if let Some(c) = &view.current {
        println!(
            "{}  {}  {}  ({})",
            c.emoji, c.location_line, c.temperature, c.condition
        );
    }
    println!("\nUpcoming days:");
    for d in &view.days {
        println!(
            "  {:<9} {} {:<8} {}",
            d.label, d.emoji, d.condition, d.temp_high_low
        );
    }
    Ok(())
}

fn sample_current() -> CurrentWeather {
    let now = Utc::now();
    CurrentWeather {
        timestamp: now,
        temperature: 21.0,
        feels_like: 20.0,
        humidity: 50,
        pressure: 1015,
        wind_speed: 3.4,
        wind_direction: 200,
        conditions: vec![desc("Clear", "clear sky")],
        main_condition: WeatherCondition::Clear,
        visibility: 10000,
        clouds: 5,
        uv_index: 4.0,
        sunrise: now,
        sunset: now + Duration::hours(12),
        rain_last_hour: None,
        snow_last_hour: None,
        air_quality_index: None,
    }
}

fn sample_hourly() -> Vec<HourlyForecast> {
    let base = Utc::now();
    (0..6)
        .map(|i| HourlyForecast {
            timestamp: base + Duration::hours(i),
            temperature: 21.0 + i as f64,
            feels_like: 20.0 + i as f64,
            humidity: 50,
            pressure: 1015,
            wind_speed: 3.0,
            wind_direction: 200,
            conditions: vec![desc("Clear", "clear sky")],
            main_condition: WeatherCondition::Clear,
            pop: 0.05 * i as f64,
            visibility: 10000,
            clouds: 5,
            rain: None,
            snow: None,
        })
        .collect()
}

fn sample_daily() -> Vec<DailyForecast> {
    let base = Utc::now();
    let conditions = [
        WeatherCondition::Clear,
        WeatherCondition::Clouds,
        WeatherCondition::Rain,
    ];
    (0..3)
        .map(|i| DailyForecast {
            date: base + Duration::days(i),
            sunrise: base,
            sunset: base + Duration::hours(12),
            temp_morning: 14.0,
            temp_day: 24.0,
            temp_evening: 19.0,
            temp_night: 11.0,
            temp_min: 11.0 + i as f64,
            temp_max: 24.0 + i as f64,
            feels_like_day: 23.0,
            feels_like_night: 10.0,
            pressure: 1015,
            humidity: 50,
            wind_speed: 3.0,
            wind_direction: 200,
            conditions: vec![],
            main_condition: conditions[i as usize],
            clouds: 20,
            pop: 0.1 * i as f64,
            rain: None,
            snow: None,
            uv_index: 4.0,
        })
        .collect()
}

fn desc(main: &str, description: &str) -> WeatherDescription {
    WeatherDescription {
        id: 0,
        main: main.into(),
        description: description.into(),
        icon: "01d".into(),
    }
}

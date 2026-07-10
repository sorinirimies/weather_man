# weather_man-core

Framework-free weather logic shared by the [`weather_man`](https://crates.io/crates/weather_man)
Iced GUI and the [`weather_man-tui`](https://crates.io/crates/weather_man-tui) Ratatui TUI —
and usable on its own as a **drop-in weather API provider library**.

No GUI or TUI dependencies. Data comes from [Open-Meteo](https://open-meteo.com/) (no API key
required) with geocoding via [Nominatim/OpenStreetMap](https://nominatim.openstreetmap.org/).

## Features

- Domain models: `WeatherCondition`, `Location`, `CurrentWeather`, `HourlyForecast`, `DailyForecast`, `Forecast`
- `WeatherProvider` trait so you can swap the backend (paid API, cache, test double…)
- `WeatherForecaster` — Open-Meteo implementation of `WeatherProvider`
- `LocationService` — IP-based and name-based geocoding
- UI-agnostic formatting helpers (units, wind direction, condition tones, local time)

## Example

```rust,no_run
use weather_man_core::{LocationService, WeatherForecaster, WeatherProvider, WeatherConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let location = LocationService::new().get_location_by_name("Berlin").await?;
    let provider = WeatherForecaster::new(WeatherConfig::default());
    let forecast = provider.forecast(&location).await?;
    println!("{} hourly, {} daily", forecast.hourly.len(), forecast.daily.len());
    Ok(())
}
```

## License

MIT

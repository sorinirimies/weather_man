//! Application state, messages, and update logic for the Iced GUI.

use iced::Task;
use weather_man_core::{
    CurrentWeather, DailyForecast, HourlyForecast, Location, LocationService, WeatherConfig,
    WeatherForecaster, WeatherProvider,
};

/// A successfully loaded weather bundle for a location.
#[derive(Debug, Clone)]
pub struct Loaded {
    pub location: Location,
    pub current: CurrentWeather,
    pub hourly: Vec<HourlyForecast>,
    pub daily: Vec<DailyForecast>,
}

/// Messages driving the application.
#[derive(Debug, Clone)]
pub enum Message {
    /// The search text input changed.
    QueryChanged(String),
    /// Trigger a fetch for the current query (or auto-detect when empty).
    Search,
    /// Toggle between metric and imperial units (re-fetches).
    ToggleUnits,
    /// A weather fetch finished.
    Fetched(Result<Box<Loaded>, String>),
}

/// Loading status of the current view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Loading,
    Ready,
    Error(String),
}

/// The root application state.
pub struct App {
    pub config: WeatherConfig,
    pub query: String,
    pub status: Status,
    pub loaded: Option<Loaded>,
}

impl App {
    /// Build the initial state and kick off an auto-detected fetch.
    pub fn new() -> (Self, Task<Message>) {
        let config = WeatherConfig::default();
        let app = Self {
            config: config.clone(),
            query: String::new(),
            status: Status::Loading,
            loaded: None,
        };
        (app, Task::perform(fetch(config, None), Message::Fetched))
    }

    /// Window title.
    pub fn title(&self) -> String {
        match &self.loaded {
            Some(l) => format!("Weather Man — {}", l.location.name),
            None => "Weather Man".to_string(),
        }
    }

    /// Handle a message and optionally schedule a task.
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::QueryChanged(q) => {
                self.query = q;
                Task::none()
            }
            Message::Search => {
                self.status = Status::Loading;
                let query = if self.query.trim().is_empty() {
                    None
                } else {
                    Some(self.query.trim().to_string())
                };
                Task::perform(fetch(self.config.clone(), query), Message::Fetched)
            }
            Message::ToggleUnits => {
                self.config.units = if self.config.units == "imperial" {
                    "metric".to_string()
                } else {
                    "imperial".to_string()
                };
                self.status = Status::Loading;
                let query = self
                    .loaded
                    .as_ref()
                    .map(|l| l.location.name.clone())
                    .filter(|n| !n.is_empty());
                Task::perform(fetch(self.config.clone(), query), Message::Fetched)
            }
            Message::Fetched(Ok(loaded)) => {
                self.status = Status::Ready;
                self.loaded = Some(*loaded);
                Task::none()
            }
            Message::Fetched(Err(err)) => {
                self.status = Status::Error(err);
                Task::none()
            }
        }
    }
}

/// Fetch weather for an optional named location (auto-detect when `None`).
async fn fetch(config: WeatherConfig, query: Option<String>) -> Result<Box<Loaded>, String> {
    let location_service = LocationService::new();
    let location = match query {
        Some(name) => location_service
            .get_location_by_name(&name)
            .await
            .map_err(|e| e.to_string())?,
        None => location_service
            .get_location_from_ip()
            .await
            .map_err(|e| e.to_string())?,
    };

    let forecaster = WeatherForecaster::new(config);
    let forecast = forecaster
        .forecast(&location)
        .await
        .map_err(|e| e.to_string())?;

    let current = forecast
        .current
        .ok_or_else(|| "No current weather data available".to_string())?;

    Ok(Box::new(Loaded {
        location,
        current,
        hourly: forecast.hourly,
        daily: forecast.daily,
    }))
}

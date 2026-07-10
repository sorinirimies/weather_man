//! weather_man-tui — a single-page Ratatui terminal weather app.
//!
//! Data and formatting come from [`weather_man_core`]; this crate only handles
//! terminal rendering and input.

pub mod app;
pub mod view;

pub use app::WeatherTui;

//! weatherman-tui — a single-page Ratatui terminal weather app.
//!
//! Data and formatting come from [`weatherman_core`]; this crate only handles
//! terminal rendering and input.

pub mod app;
pub mod view;

pub use app::WeatherTui;

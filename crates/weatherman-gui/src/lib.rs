//! weatherman GUI — desktop weather app built with Iced.
//!
//! All data and formatting come from [`weatherman_core`]; this crate only
//! handles rendering and user interaction.

pub mod app;
pub mod theme;
pub mod view;

pub use app::{App, Message};

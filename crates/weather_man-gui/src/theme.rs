//! Iced colour helpers derived from `weather_man_core` condition tones.

use iced::Color;
use weather_man_core::tone_color_fn;

tone_color_fn!(
    /// Accent colour for a weather condition tone, matching the TUI palette.
    pub fn tone_color -> Color {
        sunny:   Color::from_rgb8(0xF5, 0xB3, 0x00),
        cloudy:  Color::from_rgb8(0x9A, 0xA5, 0xB1),
        wet:     Color::from_rgb8(0x4D, 0x9D, 0xE0),
        storm:   Color::from_rgb8(0xB0, 0x7C, 0xF0),
        snow:    Color::from_rgb8(0xE6, 0xF0, 0xFF),
        fog:     Color::from_rgb8(0x77, 0x80, 0x8A),
        neutral: Color::from_rgb8(0x9A, 0xA5, 0xB1),
    }
);

/// Muted secondary text colour.
pub const MUTED: Color = Color::from_rgb(0.62, 0.66, 0.72);
/// Accent/cyan colour used for headings.
pub const ACCENT: Color = Color::from_rgb(0.30, 0.80, 0.90);

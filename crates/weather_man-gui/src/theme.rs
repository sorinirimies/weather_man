//! Iced colour helpers derived from `weather_man_core` condition tones.

use iced::Color;
use weather_man_core::{condition_tone, ConditionTone, WeatherCondition};

/// Accent colour for a weather condition, matching the TUI's semantic tones.
pub fn condition_color(condition: &WeatherCondition) -> Color {
    match condition_tone(condition) {
        ConditionTone::Sunny => Color::from_rgb8(0xF5, 0xB3, 0x00),
        ConditionTone::Cloudy => Color::from_rgb8(0x9A, 0xA5, 0xB1),
        ConditionTone::Wet => Color::from_rgb8(0x4D, 0x9D, 0xE0),
        ConditionTone::Storm => Color::from_rgb8(0xB0, 0x7C, 0xF0),
        ConditionTone::Snow => Color::from_rgb8(0xE6, 0xF0, 0xFF),
        ConditionTone::Fog => Color::from_rgb8(0x77, 0x80, 0x8A),
        ConditionTone::Neutral => Color::from_rgb8(0x9A, 0xA5, 0xB1),
    }
}

/// Muted secondary text colour.
pub const MUTED: Color = Color::from_rgb(0.62, 0.66, 0.72);
/// Accent/cyan colour used for headings.
pub const ACCENT: Color = Color::from_rgb(0.30, 0.80, 0.90);

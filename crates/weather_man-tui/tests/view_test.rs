use chrono::{Duration, Utc};
use weather_man_core::{
    DailyForecast, HourlyForecast, Location, WeatherCondition, WeatherConfig, WeatherDescription,
};
use weather_man_tui::view;

fn sample_location() -> Location {
    Location {
        name: "Testville".to_string(),
        country: "Testland".to_string(),
        country_code: "TL".to_string(),
        latitude: 0.0,
        longitude: 0.0,
        timezone: "UTC".to_string(),
        region: None,
        state: None,
    }
}

fn sample_hourly(n: usize) -> Vec<HourlyForecast> {
    let base = Utc::now();
    (0..n)
        .map(|i| HourlyForecast {
            timestamp: base + Duration::hours(i as i64),
            temperature: 20.0,
            feels_like: 19.0,
            humidity: 55,
            pressure: 1013,
            wind_speed: 3.0,
            wind_direction: 90,
            conditions: vec![WeatherDescription {
                id: 0,
                main: "Clear".into(),
                description: "clear sky".into(),
                icon: "01d".into(),
            }],
            main_condition: WeatherCondition::Clear,
            pop: 0.1,
            visibility: 10000,
            clouds: 0,
            rain: None,
            snow: None,
        })
        .collect()
}

fn sample_daily(n: usize) -> Vec<DailyForecast> {
    let base = Utc::now();
    (0..n)
        .map(|i| DailyForecast {
            date: base + Duration::days(i as i64),
            sunrise: base,
            sunset: base,
            temp_morning: 12.0,
            temp_day: 22.0,
            temp_evening: 18.0,
            temp_night: 10.0,
            temp_min: 10.0,
            temp_max: 24.0,
            feels_like_day: 22.0,
            feels_like_night: 9.0,
            pressure: 1013,
            humidity: 50,
            wind_speed: 4.0,
            wind_direction: 180,
            conditions: vec![],
            main_condition: WeatherCondition::Rain,
            clouds: 40,
            pop: 0.6,
            rain: Some(1.2),
            snow: None,
            uv_index: 3.0,
        })
        .collect()
}

#[test]
fn tone_color_is_stable() {
    // Same condition should always map to the same colour.
    assert_eq!(
        view::tone_color(&WeatherCondition::Clear),
        view::tone_color(&WeatherCondition::Clear)
    );
    assert_ne!(
        view::tone_color(&WeatherCondition::Clear),
        view::tone_color(&WeatherCondition::Rain)
    );
}

#[test]
fn current_section_has_header_and_data() {
    let cfg = WeatherConfig::default();
    let lines = view::build_current_section(&sample_hourly(3), &sample_location(), &cfg);
    // header + two info lines
    assert_eq!(lines.len(), 3);
}

#[test]
fn current_section_handles_empty() {
    let cfg = WeatherConfig::default();
    let lines = view::build_current_section(&[], &sample_location(), &cfg);
    assert_eq!(lines.len(), 2); // header + "no data"
}

#[test]
fn hourly_section_caps_at_24() {
    let cfg = WeatherConfig::default();
    let lines = view::build_hourly_section(&sample_hourly(48), &sample_location(), &cfg);
    assert_eq!(lines.len(), 25); // header + 24 rows
}

#[test]
fn daily_section_caps_at_7() {
    let cfg = WeatherConfig::default();
    let lines = view::build_daily_section(&sample_daily(10), &sample_location(), &cfg);
    assert_eq!(lines.len(), 8); // header + 7 rows
}

#[test]
fn page_line_count_sums_sections() {
    let cfg = WeatherConfig::default();
    let loc = sample_location();
    let count = view::page_line_count(&sample_hourly(24), &sample_daily(7), &loc, &cfg);
    // current(3) + blank(1) + hourly(25) + blank(1) + daily(8) = 38
    assert_eq!(count, 38);
}

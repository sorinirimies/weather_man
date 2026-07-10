//! Single-page, scrollable weather view rendering (no animations).

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};
use weather_man_core::{
    condition_tone, convert_to_local, day_label, pop_percent, temp_unit_label, weekday_name,
    wind_direction_label, wind_unit_label, ConditionTone, DailyForecast, HourlyForecast, Location,
    WeatherCondition, WeatherConfig,
};

use chrono::Timelike;

/// Map a semantic condition tone to a ratatui colour.
pub fn tone_color(condition: &WeatherCondition) -> Color {
    match condition_tone(condition) {
        ConditionTone::Sunny => Color::Yellow,
        ConditionTone::Cloudy => Color::Gray,
        ConditionTone::Wet => Color::Blue,
        ConditionTone::Storm => Color::Magenta,
        ConditionTone::Snow => Color::White,
        ConditionTone::Fog => Color::DarkGray,
        ConditionTone::Neutral => Color::Gray,
    }
}

fn section_header<'a>(title: &str) -> Line<'a> {
    Line::from(Span::styled(
        title.to_string(),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))
}

/// Build the current-conditions section from the first hourly entry.
pub fn build_current_section<'a>(
    hourly: &[HourlyForecast],
    location: &Location,
    config: &WeatherConfig,
) -> Vec<Line<'a>> {
    let mut lines = vec![section_header("🌡  Current Conditions")];

    let Some(current) = hourly.first() else {
        lines.push(Line::from(Span::styled(
            "No current weather data available.",
            Style::default().fg(Color::Red),
        )));
        return lines;
    };

    let temp_unit = temp_unit_label(&config.units);
    let wind_unit = wind_unit_label(&config.units);
    let local = convert_to_local(&current.timestamp, &location.timezone);

    lines.push(Line::from(vec![
        Span::raw(format!("{} ", current.main_condition.get_emoji())),
        Span::styled(
            current.main_condition.to_string(),
            Style::default().fg(tone_color(&current.main_condition)),
        ),
        Span::raw("   "),
        Span::styled(
            format!("{:.1}{}", current.temperature, temp_unit),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!("  (feels {:.1}{})", current.feels_like, temp_unit)),
    ]));

    lines.push(Line::from(vec![
        Span::styled("Local time: ", Style::default().fg(Color::Gray)),
        Span::raw(format!("{:02}:{:02}", local.hour(), local.minute())),
        Span::raw("   "),
        Span::styled("Humidity: ", Style::default().fg(Color::Gray)),
        Span::raw(format!("{}%", current.humidity)),
        Span::raw("   "),
        Span::styled("Wind: ", Style::default().fg(Color::Gray)),
        Span::raw(format!(
            "{:.1} {} {}",
            current.wind_speed,
            wind_unit,
            wind_direction_label(current.wind_direction)
        )),
    ]));

    lines
}

/// Build the next-24-hours section.
pub fn build_hourly_section<'a>(
    hourly: &[HourlyForecast],
    location: &Location,
    config: &WeatherConfig,
) -> Vec<Line<'a>> {
    let mut lines = vec![section_header("🕓  Next 24 Hours")];

    if hourly.is_empty() {
        lines.push(Line::from(Span::styled(
            "No hourly forecast data available.",
            Style::default().fg(Color::Red),
        )));
        return lines;
    }

    let temp_unit = temp_unit_label(&config.units);

    for hour in hourly.iter().take(24) {
        let local = convert_to_local(&hour.timestamp, &location.timezone);
        let pop = pop_percent(hour.pop);

        lines.push(Line::from(vec![
            Span::styled(
                format!("{:02}:00  ", local.hour()),
                Style::default().fg(Color::Cyan),
            ),
            Span::raw(format!("{} ", hour.main_condition.get_emoji())),
            Span::styled(
                format!("{:>5.1}{}", hour.temperature, temp_unit),
                Style::default().fg(Color::White),
            ),
            Span::raw("   "),
            Span::styled(format!("💧 {:>3}%", pop), Style::default().fg(Color::Blue)),
            Span::raw("   "),
            Span::styled(
                hour.main_condition.to_string(),
                Style::default().fg(tone_color(&hour.main_condition)),
            ),
        ]));
    }

    lines
}

/// Build the 7-day forecast section.
pub fn build_daily_section<'a>(
    daily: &[DailyForecast],
    location: &Location,
    config: &WeatherConfig,
) -> Vec<Line<'a>> {
    let mut lines = vec![section_header("📅  7-Day Forecast")];

    if daily.is_empty() {
        lines.push(Line::from(Span::styled(
            "No daily forecast data available.",
            Style::default().fg(Color::Red),
        )));
        return lines;
    }

    let temp_unit = temp_unit_label(&config.units);

    for (i, day) in daily.iter().take(7).enumerate() {
        let local = convert_to_local(&day.date, &location.timezone);
        let label = day_label(i, weekday_name(&local));
        let date_str = local.format("%m/%d").to_string();
        let pop = pop_percent(day.pop);

        lines.push(Line::from(vec![
            Span::styled(
                format!("{:<10}", label),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!("{:<6}", date_str), Style::default().fg(Color::Gray)),
            Span::raw(format!("{} ", day.main_condition.get_emoji())),
            Span::styled(
                format!("{:<13}", day.main_condition.to_string()),
                Style::default().fg(tone_color(&day.main_condition)),
            ),
            Span::styled(
                format!(
                    "{:>3.0}{} / {:>3.0}{}",
                    day.temp_max, temp_unit, day.temp_min, temp_unit
                ),
                Style::default().fg(Color::White),
            ),
            Span::raw("   "),
            Span::styled(format!("💧 {:>3}%", pop), Style::default().fg(Color::Blue)),
        ]));
    }

    lines
}

/// Assemble the whole scrollable page.
pub fn build_page<'a>(
    hourly: &[HourlyForecast],
    daily: &[DailyForecast],
    location: &Location,
    config: &WeatherConfig,
) -> Text<'a> {
    let mut lines: Vec<Line> = Vec::new();
    lines.extend(build_current_section(hourly, location, config));
    lines.push(Line::from(""));
    lines.extend(build_hourly_section(hourly, location, config));
    lines.push(Line::from(""));
    lines.extend(build_daily_section(daily, location, config));
    Text::from(lines)
}

/// Render the full frame: title, scrollable body, help bar.
pub fn render(
    f: &mut Frame,
    hourly: &[HourlyForecast],
    daily: &[DailyForecast],
    location: &Location,
    config: &WeatherConfig,
    scroll: u16,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.area());

    render_title(f, chunks[0], location, config);

    let body = Paragraph::new(build_page(hourly, daily, location, config))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Forecast")
                .style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));
    f.render_widget(body, chunks[1]);

    render_help(f, chunks[2]);
}

fn render_title(f: &mut Frame, area: Rect, location: &Location, config: &WeatherConfig) {
    let title = Paragraph::new(Line::from(vec![
        Span::styled(
            format!("Weather Man - {}", location.name),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            format!("[{}]", location.country),
            Style::default().fg(Color::Gray),
        ),
        Span::raw("  "),
        Span::styled(
            format!("({})", temp_unit_label(&config.units)),
            Style::default().fg(Color::Yellow),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(title, area);
}

fn render_help(f: &mut Frame, area: Rect) {
    let help = Paragraph::new(Line::from(vec![
        Span::styled("Keys: ", Style::default().fg(Color::Cyan)),
        Span::styled("↑/↓", Style::default().fg(Color::Yellow)),
        Span::raw(" Scroll | "),
        Span::styled("PgUp/PgDn", Style::default().fg(Color::Yellow)),
        Span::raw(" Page | "),
        Span::styled("g/G", Style::default().fg(Color::Yellow)),
        Span::raw(" Top/Bottom | "),
        Span::styled("q/ESC", Style::default().fg(Color::Yellow)),
        Span::raw(" Quit"),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(help, area);
}

/// Total number of content lines (used to clamp scrolling).
pub fn page_line_count(
    hourly: &[HourlyForecast],
    daily: &[DailyForecast],
    location: &Location,
    config: &WeatherConfig,
) -> u16 {
    build_page(hourly, daily, location, config).lines.len() as u16
}

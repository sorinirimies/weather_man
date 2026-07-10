//! Single-page, scrollable weather view rendering (no animations).

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};
use weather_man_core::{
    temp_unit_label, tone_color_fn, CurrentView, DayRow, ForecastView, HourRow, Location,
    WeatherConfig,
};

tone_color_fn!(
    /// Map a weather condition tone to a ratatui colour.
    pub fn tone_color -> Color {
        sunny:   Color::Yellow,
        cloudy:  Color::Gray,
        wet:     Color::Blue,
        storm:   Color::Magenta,
        snow:    Color::White,
        fog:     Color::DarkGray,
        neutral: Color::Gray,
    }
);

fn section_header<'a>(title: &str) -> Line<'a> {
    Line::from(Span::styled(
        title.to_string(),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))
}

fn missing<'a>(msg: &str) -> Line<'a> {
    Line::from(Span::styled(
        msg.to_string(),
        Style::default().fg(Color::Red),
    ))
}

/// Build the current-conditions section from a pre-computed view.
pub fn current_lines<'a>(current: Option<&CurrentView>) -> Vec<Line<'a>> {
    let mut lines = vec![section_header("🌡  Current Conditions")];

    let Some(c) = current else {
        lines.push(missing("No current weather data available."));
        return lines;
    };

    lines.push(Line::from(vec![
        Span::raw(format!("{} ", c.emoji)),
        Span::styled(c.condition.clone(), Style::default().fg(tone_color(c.tone))),
        Span::raw("   "),
        Span::styled(
            c.temperature.clone(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!("  (feels {})", c.feels_like)),
    ]));

    lines.push(Line::from(vec![
        Span::styled("Local time: ", Style::default().fg(Color::Gray)),
        Span::raw(c.local_time.clone()),
        Span::raw("   "),
        Span::styled("Humidity: ", Style::default().fg(Color::Gray)),
        Span::raw(c.humidity.clone()),
        Span::raw("   "),
        Span::styled("Wind: ", Style::default().fg(Color::Gray)),
        Span::raw(c.wind.clone()),
    ]));

    lines
}

/// Build the next-24-hours section from pre-computed rows.
pub fn hourly_lines<'a>(hours: &[HourRow]) -> Vec<Line<'a>> {
    let mut lines = vec![section_header("🕓  Next 24 Hours")];

    if hours.is_empty() {
        lines.push(missing("No hourly forecast data available."));
        return lines;
    }

    for h in hours {
        lines.push(Line::from(vec![
            Span::styled(format!("{}  ", h.time), Style::default().fg(Color::Cyan)),
            Span::raw(format!("{} ", h.emoji)),
            Span::styled(
                format!("{:>7}", h.temperature),
                Style::default().fg(Color::White),
            ),
            Span::raw("   "),
            Span::styled(
                format!("💧 {:>3}%", h.pop_percent),
                Style::default().fg(Color::Blue),
            ),
            Span::raw("   "),
            Span::styled(h.condition.clone(), Style::default().fg(tone_color(h.tone))),
        ]));
    }

    lines
}

/// Build the 7-day forecast section from pre-computed rows.
pub fn daily_lines<'a>(days: &[DayRow]) -> Vec<Line<'a>> {
    let mut lines = vec![section_header("📅  7-Day Forecast")];

    if days.is_empty() {
        lines.push(missing("No daily forecast data available."));
        return lines;
    }

    for d in days {
        lines.push(Line::from(vec![
            Span::styled(
                format!("{:<10}", d.label),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!("{:<6}", d.date), Style::default().fg(Color::Gray)),
            Span::raw(format!("{} ", d.emoji)),
            Span::styled(
                format!("{:<13}", d.condition),
                Style::default().fg(tone_color(d.tone)),
            ),
            Span::styled(
                format!("{:>13}", d.temp_high_low),
                Style::default().fg(Color::White),
            ),
            Span::raw("   "),
            Span::styled(
                format!("💧 {:>3}%", d.pop_percent),
                Style::default().fg(Color::Blue),
            ),
        ]));
    }

    lines
}

/// Assemble the whole scrollable page from a forecast view-model.
pub fn build_page<'a>(view: &ForecastView) -> Text<'a> {
    let mut lines: Vec<Line> = Vec::new();
    lines.extend(current_lines(view.current.as_ref()));
    lines.push(Line::from(""));
    lines.extend(hourly_lines(&view.hours));
    lines.push(Line::from(""));
    lines.extend(daily_lines(&view.days));
    Text::from(lines)
}

/// Render the full frame: title, scrollable body, help bar.
pub fn render(
    f: &mut Frame,
    view: &ForecastView,
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

    let body = Paragraph::new(build_page(view))
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
pub fn page_line_count(view: &ForecastView) -> u16 {
    build_page(view).lines.len() as u16
}

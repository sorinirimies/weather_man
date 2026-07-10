//! View rendering for the Iced GUI.

use crate::app::{App, Loaded, Message, Status};
use crate::theme::{condition_color, ACCENT, MUTED};
use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Alignment, Element, Length};
use weather_man_core::{
    convert_to_local, day_label, pop_percent, temp_unit_label, weekday_name, wind_direction_label,
    wind_unit_label, DailyForecast, HourlyForecast, Location, WeatherConfig,
};

use chrono::Timelike;

/// Top-level view.
pub fn view(app: &App) -> Element<'_, Message> {
    let body: Element<Message> = match (&app.status, &app.loaded) {
        (Status::Error(err), _) => error_view(err),
        (Status::Loading, None) => centered("Loading weather…"),
        (_, Some(loaded)) => loaded_view(loaded, &app.config),
        (_, None) => centered("No data"),
    };

    column![search_bar(app), body]
        .spacing(12)
        .padding(16)
        .into()
}

fn search_bar(app: &App) -> Element<'_, Message> {
    let units_label = if app.config.units == "imperial" {
        "°F"
    } else {
        "°C"
    };

    row![
        text("⛅ Weather Man").size(24).color(ACCENT),
        Space::with_width(Length::Fill),
        text_input("Search city…", &app.query)
            .on_input(Message::QueryChanged)
            .on_submit(Message::Search)
            .width(Length::Fixed(240.0))
            .padding(8),
        button(text("Search")).on_press(Message::Search).padding(8),
        button(text(units_label))
            .on_press(Message::ToggleUnits)
            .padding(8),
    ]
    .spacing(10)
    .align_y(Alignment::Center)
    .into()
}

fn loaded_view<'a>(loaded: &'a Loaded, config: &'a WeatherConfig) -> Element<'a, Message> {
    scrollable(
        column![
            current_card(loaded, config),
            section_title("Next 24 Hours"),
            hourly_strip(&loaded.hourly, &loaded.location, config),
            section_title("7-Day Forecast"),
            daily_list(&loaded.daily, &loaded.location, config),
        ]
        .spacing(16),
    )
    .height(Length::Fill)
    .into()
}

fn current_card<'a>(loaded: &'a Loaded, config: &'a WeatherConfig) -> Element<'a, Message> {
    let c = &loaded.current;
    let temp_unit = temp_unit_label(&config.units);
    let wind_unit = wind_unit_label(&config.units);
    let local = convert_to_local(&c.timestamp, &loaded.location.timezone);

    let content = column![
        row![
            text(c.main_condition.get_emoji()).size(48),
            column![
                text(format!(
                    "{}, {}",
                    loaded.location.name, loaded.location.country
                ))
                .size(20)
                .color(ACCENT),
                text(c.main_condition.to_string())
                    .size(16)
                    .color(condition_color(&c.main_condition)),
            ]
            .spacing(2),
            Space::with_width(Length::Fill),
            text(format!("{:.1}{}", c.temperature, temp_unit)).size(44),
        ]
        .spacing(16)
        .align_y(Alignment::Center),
        row![
            stat("Feels like", format!("{:.1}{}", c.feels_like, temp_unit)),
            stat("Humidity", format!("{}%", c.humidity)),
            stat(
                "Wind",
                format!(
                    "{:.1} {} {}",
                    c.wind_speed,
                    wind_unit,
                    wind_direction_label(c.wind_direction)
                )
            ),
            stat(
                "Local time",
                format!("{:02}:{:02}", local.hour(), local.minute())
            ),
        ]
        .spacing(24),
    ]
    .spacing(14);

    card(content.into())
}

fn hourly_strip<'a>(
    hourly: &'a [HourlyForecast],
    location: &'a Location,
    config: &'a WeatherConfig,
) -> Element<'a, Message> {
    let temp_unit = temp_unit_label(&config.units);
    let mut r = row![].spacing(10);

    for hour in hourly.iter().take(24) {
        let local = convert_to_local(&hour.timestamp, &location.timezone);
        let cell = column![
            text(format!("{:02}:00", local.hour()))
                .size(13)
                .color(MUTED),
            text(hour.main_condition.get_emoji()).size(24),
            text(format!("{:.0}{}", hour.temperature, temp_unit)).size(15),
            text(format!("💧{}%", pop_percent(hour.pop)))
                .size(12)
                .color(condition_color(&hour.main_condition)),
        ]
        .spacing(4)
        .align_x(Alignment::Center);
        r = r.push(card(cell.into()));
    }

    scrollable(r)
        .direction(scrollable::Direction::Horizontal(
            scrollable::Scrollbar::new(),
        ))
        .width(Length::Fill)
        .into()
}

fn daily_list<'a>(
    daily: &'a [DailyForecast],
    location: &'a Location,
    config: &'a WeatherConfig,
) -> Element<'a, Message> {
    let temp_unit = temp_unit_label(&config.units);
    let mut col = column![].spacing(8);

    for (i, day) in daily.iter().take(7).enumerate() {
        let local = convert_to_local(&day.date, &location.timezone);
        let label = day_label(i, weekday_name(&local));
        let line = row![
            text(label)
                .size(15)
                .width(Length::Fixed(110.0))
                .color(ACCENT),
            text(day.main_condition.get_emoji()).size(20),
            text(day.main_condition.to_string())
                .size(14)
                .width(Length::Fixed(130.0))
                .color(condition_color(&day.main_condition)),
            Space::with_width(Length::Fill),
            text(format!("💧 {}%", pop_percent(day.pop)))
                .size(14)
                .color(MUTED),
            Space::with_width(Length::Fixed(24.0)),
            text(format!(
                "{:.0}{} / {:.0}{}",
                day.temp_max, temp_unit, day.temp_min, temp_unit
            ))
            .size(15),
        ]
        .spacing(12)
        .align_y(Alignment::Center);
        col = col.push(card(line.into()));
    }

    col.into()
}

fn stat<'a>(label: &'a str, value: String) -> Element<'a, Message> {
    column![text(label).size(12).color(MUTED), text(value).size(16),]
        .spacing(2)
        .into()
}

fn section_title(title: &str) -> Element<'_, Message> {
    text(title.to_string()).size(18).color(ACCENT).into()
}

fn card(content: Element<'_, Message>) -> Element<'_, Message> {
    container(content)
        .padding(12)
        .style(container::rounded_box)
        .into()
}

fn centered(msg: &str) -> Element<'_, Message> {
    container(text(msg.to_string()).size(18).color(MUTED))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

fn error_view(err: &str) -> Element<'_, Message> {
    container(
        column![
            text("⚠ Could not load weather").size(20),
            text(err.to_string()).size(14).color(MUTED),
            button(text("Retry")).on_press(Message::Search).padding(8),
        ]
        .spacing(12)
        .align_x(Alignment::Center),
    )
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}

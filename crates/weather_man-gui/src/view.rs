//! View rendering for the Iced GUI.

use crate::app::{App, Loaded, Message, Status};
use crate::theme::{tone_color, ACCENT, MUTED};
use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Alignment, Element, Length};
use weather_man_core::{CurrentView, DayRow, ForecastView, HourRow};

/// Top-level view.
pub fn view(app: &App) -> Element<'_, Message> {
    let body: Element<Message> = match (&app.status, &app.loaded) {
        (Status::Error(err), _) => error_view(err),
        (Status::Loading, None) => centered("Loading weather…"),
        (_, Some(loaded)) => loaded_view(loaded),
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

fn loaded_view(loaded: &Loaded) -> Element<'_, Message> {
    let ForecastView {
        current,
        hours,
        days,
    } = &loaded.view;

    let mut col = column![].spacing(16);

    if let Some(c) = current {
        col = col.push(current_card(c));
    }
    col = col
        .push(section_title("Next 24 Hours"))
        .push(hourly_strip(hours))
        .push(section_title("7-Day Forecast"))
        .push(daily_list(days));

    scrollable(col).height(Length::Fill).into()
}

fn current_card(c: &CurrentView) -> Element<'_, Message> {
    let content = column![
        row![
            text(c.emoji).size(48),
            column![
                text(c.location_line.clone()).size(20).color(ACCENT),
                text(c.condition.clone()).size(16).color(tone_color(c.tone)),
            ]
            .spacing(2),
            Space::with_width(Length::Fill),
            text(c.temperature.clone()).size(44),
        ]
        .spacing(16)
        .align_y(Alignment::Center),
        row![
            stat("Feels like", c.feels_like.clone()),
            stat("Humidity", c.humidity.clone()),
            stat("Wind", c.wind.clone()),
            stat("Local time", c.local_time.clone()),
        ]
        .spacing(24),
    ]
    .spacing(14);

    card(content.into())
}

fn hourly_strip(hours: &[HourRow]) -> Element<'_, Message> {
    let mut r = row![].spacing(10);

    for h in hours {
        let cell = column![
            text(h.time.clone()).size(13).color(MUTED),
            text(h.emoji).size(24),
            text(h.temperature.clone()).size(15),
            text(format!("💧{}%", h.pop_percent))
                .size(12)
                .color(tone_color(h.tone)),
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

fn daily_list(days: &[DayRow]) -> Element<'_, Message> {
    let mut col = column![].spacing(8);

    for d in days {
        let line = row![
            text(d.label.clone())
                .size(15)
                .width(Length::Fixed(110.0))
                .color(ACCENT),
            text(d.emoji).size(20),
            text(d.condition.clone())
                .size(14)
                .width(Length::Fixed(130.0))
                .color(tone_color(d.tone)),
            Space::with_width(Length::Fill),
            text(format!("💧 {}%", d.pop_percent)).size(14).color(MUTED),
            Space::with_width(Length::Fixed(24.0)),
            text(d.temp_high_low.clone()).size(15),
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

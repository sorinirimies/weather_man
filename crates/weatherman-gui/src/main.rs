//! Desktop entry point for the Iced weather GUI.

use weatherman_gui::app::App;
use weatherman_gui::view::view;

fn theme(_state: &App) -> iced::Theme {
    iced::Theme::TokyoNight
}

fn main() -> iced::Result {
    iced::application(App::new, App::update, view)
        .title(App::title)
        .theme(theme)
        .window_size((960.0, 720.0))
        .run()
}

//! Desktop entry point for the Iced weather GUI.

use weatherman_gui::app::App;
use weatherman_gui::view::view;

fn main() -> iced::Result {
    iced::application(App::title, App::update, view)
        .theme(|_| iced::Theme::TokyoNight)
        .window_size((960.0, 720.0))
        .run_with(App::new)
}

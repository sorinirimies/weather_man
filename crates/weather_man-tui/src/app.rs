//! The terminal application: owns state, the event loop, and terminal setup.

use crate::view;
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, Stdout};
use weather_man_core::{DailyForecast, HourlyForecast, Location, WeatherConfig};

/// The single-page terminal weather UI.
pub struct WeatherTui {
    hourly: Vec<HourlyForecast>,
    daily: Vec<DailyForecast>,
    location: Location,
    config: WeatherConfig,
    scroll: u16,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl WeatherTui {
    /// Create a new TUI, entering the alternate screen.
    pub fn new(
        hourly: Vec<HourlyForecast>,
        daily: Vec<DailyForecast>,
        location: Location,
        config: WeatherConfig,
    ) -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self {
            hourly,
            daily,
            location,
            config,
            scroll: 0,
            terminal,
        })
    }

    /// Run the event loop until the user quits.
    pub fn run(&mut self) -> Result<()> {
        loop {
            let total =
                view::page_line_count(&self.hourly, &self.daily, &self.location, &self.config);
            let scroll = self.scroll;

            self.terminal.draw(|f| {
                view::render(
                    f,
                    &self.hourly,
                    &self.daily,
                    &self.location,
                    &self.config,
                    scroll,
                );
            })?;

            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Down | KeyCode::Char('j') => {
                        self.scroll = scroll.saturating_add(1).min(total);
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        self.scroll = scroll.saturating_sub(1);
                    }
                    KeyCode::PageDown => {
                        self.scroll = scroll.saturating_add(10).min(total);
                    }
                    KeyCode::PageUp => {
                        self.scroll = scroll.saturating_sub(10);
                    }
                    KeyCode::Char('g') | KeyCode::Home => self.scroll = 0,
                    KeyCode::Char('G') | KeyCode::End => self.scroll = total,
                    _ => {}
                }
            }
        }

        self.restore();
        Ok(())
    }

    fn restore(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        let _ = self.terminal.show_cursor();
    }
}

impl Drop for WeatherTui {
    fn drop(&mut self) {
        self.restore();
        println!();
    }
}

# weather_man-tui

Terminal weather app built with [Ratatui](https://ratatui.rs/). A single, scrollable
page with current conditions, the next 24 hours, and a 7-day forecast — keyboard-driven
and great over SSH.

Powered by [`weather_man-core`](https://crates.io/crates/weather_man-core) —
data from [Open-Meteo](https://open-meteo.com/), no API key required.

## Install

```
cargo install weather_man-tui
```

## Run

```
# Auto-detected location
weather_man-tui

# Specific city
weather_man-tui --location "New York"

# Imperial units
weather_man-tui --units imperial

# JSON output (no TUI)
weather_man-tui --json
```

## Keys

| Key | Action |
|-----|--------|
| `↑`/`↓` or `j`/`k` | Scroll |
| `PgUp`/`PgDn` | Page up/down |
| `g` / `G` | Top / bottom |
| `q` / `Esc` | Quit |

## License

MIT

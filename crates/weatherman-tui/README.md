# weatherman-tui

Terminal weather app built with [Ratatui](https://ratatui.rs/). A single, scrollable
page with current conditions, the next 24 hours, and a 7-day forecast — keyboard-driven
and great over SSH.

Powered by [`weatherman-core`](https://crates.io/crates/weatherman-core) —
data from [Open-Meteo](https://open-meteo.com/), no API key required.

## Install

```
cargo install weatherman-tui
```

## Run

```
# Auto-detected location
weatherman-tui

# Specific city
weatherman-tui --location "New York"

# Imperial units
weatherman-tui --units imperial

# JSON output (no TUI)
weatherman-tui --json
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

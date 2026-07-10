# VHS tape — weather_man TUI · Demo
# Shows the single-page terminal weather app: current conditions, the next
# 24 hours, and the 7-day forecast, with scrolling.
#
# Run: vhs crates/weather_man-tui/examples/vhs/tui-demo.tape

Output crates/weather_man-tui/examples/vhs/generated/tui-demo.gif

Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 800
Set FontSize 15
Set PlaybackSpeed 1.0

# ── Launch on a fixed location (hidden while it loads) ────────────────────────
Hide
Type "cargo run -q -p weather_man-tui -- --location 'Berlin'"
Enter
Sleep 6s
Show

# ── Read the current conditions ───────────────────────────────────────────────
Sleep 2s

# ── Scroll through the next 24 hours ──────────────────────────────────────────
Down@300ms 6
Sleep 1s
Down@300ms 6
Sleep 1s

# ── Page down to the 7-day forecast ───────────────────────────────────────────
PageDown
Sleep 2s
PageDown
Sleep 2s

# ── Jump back to the top ──────────────────────────────────────────────────────
Type "g"
Sleep 1500ms

# ── Quit ──────────────────────────────────────────────────────────────────────
Type "q"
Sleep 500ms

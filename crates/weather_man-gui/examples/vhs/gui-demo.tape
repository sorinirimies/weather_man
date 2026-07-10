# VHS tape — weather_man GUI · Demo
# Shows the Iced desktop app: current conditions card, hourly strip, and the
# 7-day forecast, plus searching for a city.
#
# Requires a display/compositor. Run: vhs crates/weather_man-gui/examples/vhs/gui-demo.tape

Output crates/weather_man-gui/examples/vhs/generated/gui-demo.gif

Set Theme "Catppuccin Mocha"
Set Width 1400
Set Height 900
Set FontSize 15
Set PlaybackSpeed 1.0

# ── Launch the GUI (hidden while it builds + loads) ───────────────────────────
Hide
Type "cargo run -q -p weather_man"
Enter
Sleep 12s
Show

# ── Look at the auto-detected location ────────────────────────────────────────
Sleep 3s

# ── Search for a city ─────────────────────────────────────────────────────────
Type "Tokyo"
Sleep 500ms
Enter
Sleep 4s

# ── Toggle to imperial units ──────────────────────────────────────────────────
Sleep 2s

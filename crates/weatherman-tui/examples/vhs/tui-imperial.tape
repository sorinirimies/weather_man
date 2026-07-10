# VHS tape — weatherman TUI · Imperial units
# Same single-page view rendered with imperial units (°F, mph).
#
# Run: vhs crates/weatherman-tui/examples/vhs/tui-imperial.tape

Output crates/weatherman-tui/examples/vhs/generated/tui-imperial.gif

Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 800
Set FontSize 15
Set PlaybackSpeed 1.0

Hide
Type "cargo run -q -p weatherman-tui -- --location 'New York' --units imperial"
Enter
Sleep 6s
Show

Sleep 2s
Down@300ms 8
Sleep 1s
PageDown
Sleep 2s
Type "q"
Sleep 500ms

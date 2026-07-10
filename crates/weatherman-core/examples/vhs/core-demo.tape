# VHS tape — weatherman-core · Library Demo
# Runs the `report` example, showing the core library fetching a forecast and
# printing the ready-to-render view-model — no GUI or TUI involved.
#
# Run: vhs crates/weatherman-core/examples/vhs/core-demo.tape

Output crates/weatherman-core/examples/vhs/generated/core-demo.gif

Set Theme "Catppuccin Mocha"
Set Width 1100
Set Height 760
Set FontSize 15
Set PlaybackSpeed 1.0

Hide
Type "cargo run -q -p weatherman-core --example report -- 'Berlin'"
Enter
Sleep 6s
Show

Sleep 4s

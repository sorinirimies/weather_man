#!/usr/bin/env nu
# Prints the current workspace version from Cargo.toml.
# Usage: nu scripts/version.nu

# Read workspace.package.version from a Cargo.toml (defaults to ./Cargo.toml).
# Exposed so tests can point it at a fixture file.
export def read_version [path: string = "Cargo.toml"]: nothing -> string {
    open $path | get workspace.package.version
}

def main [] {
    read_version | print
}

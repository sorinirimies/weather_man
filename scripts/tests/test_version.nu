#!/usr/bin/env nu
# ── weatherman · test_version.nu ───────────────────────────────────────────
# Tests for scripts/version.nu — reading the workspace version from Cargo.toml.

use std/assert
use runner.nu *
use ../version.nu [read_version]

# ── Helpers ─────────────────────────────────────────────────────────────────

def make_workspace_cargo [version: string]: nothing -> string {
    $'[workspace]
members = [
    "crates/weatherman-core",
]
resolver = "2"

[workspace.package]
version = "($version)"
edition = "2021"

[workspace.dependencies]
weatherman-core = { path = "crates/weatherman-core", version = "($version)" }
'
}

def write_temp_cargo [version: string]: nothing -> string {
    let dir = (mktemp -d)
    let path = ($dir | path join "Cargo.toml")
    make_workspace_cargo $version | save --force $path
    $path
}

# ── Tests ───────────────────────────────────────────────────────────────────

def "test version: reads simple semver" [] {
    let path = (write_temp_cargo "1.2.3")
    assert equal (read_version $path) "1.2.3"
    rm -rf ($path | path dirname)
}

def "test version: reads pre-release version" [] {
    let path = (write_temp_cargo "0.5.0-rc.1")
    assert equal (read_version $path) "0.5.0-rc.1"
    rm -rf ($path | path dirname)
}

def "test version: reads zero version" [] {
    let path = (write_temp_cargo "0.0.0")
    assert equal (read_version $path) "0.0.0"
    rm -rf ($path | path dirname)
}

def "test version: reads large version numbers" [] {
    let path = (write_temp_cargo "12.345.6789")
    assert equal (read_version $path) "12.345.6789"
    rm -rf ($path | path dirname)
}

def "test version: matches the real workspace Cargo.toml" [] {
    # The real repo file should parse to a valid semver.
    let repo_root = ($env.CURRENT_FILE | path dirname | path dirname | path dirname)
    let ver = (read_version ($repo_root | path join "Cargo.toml"))
    assert ($ver | find --regex '^\d+\.\d+\.\d+' | is-not-empty)
}

# ── Main ────────────────────────────────────────────────────────────────────

def main [] { run-tests }

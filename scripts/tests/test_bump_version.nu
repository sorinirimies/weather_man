#!/usr/bin/env nu
# ── weatherman · test_bump_version.nu ──────────────────────────────────────
# Tests for scripts/bump_version.nu pure helpers:
#   is_valid_version, set_workspace_version, set_core_dep_version.

use std/assert
use runner.nu *
use ../bump_version.nu [is_valid_version set_workspace_version set_core_dep_version]

# ── Helpers ─────────────────────────────────────────────────────────────────

def make_workspace_cargo [version: string]: nothing -> string {
    $'[workspace]
members = [
    "crates/weatherman-core",
    "crates/weatherman-gui",
    "crates/weatherman-tui",
]
resolver = "2"

[workspace.package]
version    = "($version)"
edition    = "2021"

[workspace.dependencies]
weatherman-core = { path = "crates/weatherman-core", version = "($version)" }
'
}

def dep_version [cargo: string]: nothing -> string {
    $cargo
    | lines
    | where { |l| $l =~ '^weatherman-core\s*=' }
    | first
    | parse --regex 'version\s*=\s*"(?P<ver>[^"]+)"'
    | get ver
    | first
}

def pkg_version [cargo: string]: nothing -> string {
    $cargo
    | lines
    | each { |l| $l | str trim }
    | where { |l| $l | str starts-with 'version' }
    | where { |l| not ($l | str contains 'workspace = true') }
    | first
    | parse --regex 'version\s*=\s*"(?P<ver>[^"]+)"'
    | get ver
    | first
}

# ── is_valid_version ────────────────────────────────────────────────────────

def "test bump: accepts plain semver" [] {
    assert (is_valid_version "1.2.3")
    assert (is_valid_version "0.0.0")
    assert (is_valid_version "12.345.6789")
}

def "test bump: accepts pre-release semver" [] {
    assert (is_valid_version "1.0.0-rc.1")
    assert (is_valid_version "0.3.0-beta.2")
}

def "test bump: rejects invalid versions" [] {
    assert (not (is_valid_version "1.2"))
    assert (not (is_valid_version "v1.2.3"))
    assert (not (is_valid_version "1.2.3.4"))
    assert (not (is_valid_version "abc"))
    assert (not (is_valid_version ""))
}

# ── set_workspace_version ───────────────────────────────────────────────────

def "test bump: rewrites workspace version" [] {
    let cargo = (make_workspace_cargo "0.3.0")
    let updated = (set_workspace_version $cargo "0.4.0")
    assert equal (pkg_version $updated) "0.4.0"
}

# ── set_core_dep_version ────────────────────────────────────────────────────

def "test bump: rewrites core dependency version" [] {
    let cargo = (make_workspace_cargo "0.3.0")
    let updated = (set_core_dep_version $cargo "0.4.0")
    assert equal (dep_version $updated) "0.4.0"
}

def "test bump: full bump keeps workspace and dep versions in sync" [] {
    let cargo = (make_workspace_cargo "0.3.0")
    let bumped = (set_core_dep_version (set_workspace_version $cargo "1.0.0") "1.0.0")
    assert equal (pkg_version $bumped) "1.0.0"
    assert equal (dep_version $bumped) "1.0.0"
}

def "test bump: leaves unrelated lines untouched" [] {
    let cargo = (make_workspace_cargo "0.3.0")
    let updated = (set_core_dep_version $cargo "0.4.0")
    assert ($updated | str contains 'edition    = "2021"')
    assert ($updated | str contains 'resolver = "2"')
}

# ── Main ────────────────────────────────────────────────────────────────────

def main [] { run-tests }

#!/usr/bin/env nu
# ── weatherman · test_upgrade_deps.nu ──────────────────────────────────────
# Tests for scripts/upgrade_deps.nu pure helpers: commit_label, all_passed.

use std/assert
use runner.nu *
use ../upgrade_deps.nu [commit_label all_passed]

# ── commit_label ────────────────────────────────────────────────────────────

def "test upgrade: commit label for toml changes" [] {
    let label = (commit_label true false "2026-07-10")
    assert equal $label "chore: upgrade dependencies 2026-07-10"
}

def "test upgrade: commit label for lock-only changes" [] {
    let label = (commit_label false true "2026-07-10")
    assert equal $label "chore: update Cargo.lock 2026-07-10"
}

def "test upgrade: toml dirty takes precedence over lock" [] {
    let label = (commit_label true true "2026-07-10")
    assert equal $label "chore: upgrade dependencies 2026-07-10"
}

def "test upgrade: empty label when nothing changed" [] {
    let label = (commit_label false false "2026-07-10")
    assert equal $label ""
}

# ── all_passed ──────────────────────────────────────────────────────────────

def "test upgrade: all_passed true when all true" [] {
    assert (all_passed [true true true])
}

def "test upgrade: all_passed false when any false" [] {
    assert (not (all_passed [true false true]))
}

def "test upgrade: all_passed true for empty list" [] {
    assert (all_passed [])
}

# ── Main ────────────────────────────────────────────────────────────────────

def main [] { run-tests }

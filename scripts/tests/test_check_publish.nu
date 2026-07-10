#!/usr/bin/env nu
# ── weatherman · test_check_publish.nu ─────────────────────────────────────
# Tests for scripts/check_publish.nu — the publish_order invariant.

use std/assert
use runner.nu *
use ../check_publish.nu [publish_order]

def "test publish: order lists all three crates" [] {
    let order = (publish_order)
    assert equal ($order | length) 3
}

def "test publish: core is published first" [] {
    let order = (publish_order)
    assert equal ($order | first) "weatherman-core"
}

def "test publish: core precedes the crates that depend on it" [] {
    let order = (publish_order)
    let core_idx = ($order | enumerate | where item == "weatherman-core" | get index | first)
    let gui_idx = ($order | enumerate | where item == "weatherman" | get index | first)
    let tui_idx = ($order | enumerate | where item == "weatherman-tui" | get index | first)
    assert ($core_idx < $gui_idx)
    assert ($core_idx < $tui_idx)
}

def "test publish: order has no duplicates" [] {
    let order = (publish_order)
    assert equal ($order | uniq | length) ($order | length)
}

# ── Main ────────────────────────────────────────────────────────────────────

def main [] { run-tests }

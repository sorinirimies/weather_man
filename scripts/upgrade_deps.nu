#!/usr/bin/env nu
# ─── weather_man · Dependency Upgrade Script ───────────────────────────────────
# Upgrades workspace dependencies using cargo-edit, then cross-checks everything.
#
# Prerequisites: cargo install cargo-edit
#
# Usage:
#   nu scripts/upgrade_deps.nu           # interactive upgrade
#   nu scripts/upgrade_deps.nu --check   # dry-run: only list outdated deps
# ───────────────────────────────────────────────────────────────────────────────

def header [msg: string] { print $"\n(ansi cyan_bold)═══ ($msg) ═══(ansi reset)\n" }
def step   [msg: string] { print $"  (ansi green_bold)▶(ansi reset) ($msg)" }
def err    [msg: string] { print $"  (ansi red_bold)✖(ansi reset) ($msg)" }
def ok     [msg: string] { print $"  (ansi green)✔(ansi reset) ($msg)" }

def preflight [] {
    header "weather_man · Pre-flight Checks"
    if not ("Cargo.toml" | path exists) {
        err "No Cargo.toml found — run this script from the workspace root."
        exit 1
    }
    let has_upgrade = (do { cargo upgrade --version } | complete)
    if $has_upgrade.exit_code != 0 {
        err "`cargo upgrade` not found. Install it with: cargo install cargo-edit"
        exit 1
    }
    ok "cargo-edit is installed"
}

def list_outdated [] {
    header "weather_man · Outdated Dependencies (dry-run)"
    step "Running `cargo upgrade --dry-run` …"
    let result = (do { cargo upgrade --workspace --dry-run } | complete)
    print $result.stdout
    if ($result.stderr | str trim | is-not-empty) { print $result.stderr }
}

def do_upgrade [] {
    header "weather_man · Upgrading Dependencies"
    step "Running `cargo upgrade --workspace` …"
    let result = (do { cargo upgrade --workspace } | complete)
    print $result.stdout
    if $result.exit_code != 0 {
        err "cargo upgrade failed"
        print $result.stderr
        exit 1
    }
    ok "Dependencies upgraded in Cargo.toml(s)"

    step "Running `cargo update` to refresh Cargo.lock …"
    let lock_result = (do { cargo update } | complete)
    if $lock_result.exit_code != 0 {
        err "cargo update failed"
        print $lock_result.stderr
        exit 1
    }
    ok "Cargo.lock updated"
}

def cross_check [] {
    header "weather_man · Cross-checks"

    step "cargo check --workspace …"
    let chk = (do { cargo check --workspace } | complete)
    if $chk.exit_code != 0 { err "cargo check failed:"; print $chk.stderr; exit 1 }
    ok "workspace compiles"

    step "cargo clippy --workspace --all-targets -- -D warnings …"
    let clip = (do { cargo clippy --workspace --all-targets -- -D warnings } | complete)
    if $clip.exit_code != 0 { err "clippy found warnings/errors:"; print $clip.stderr; exit 1 }
    ok "clippy clean"

    step "cargo test --workspace …"
    let tst = (do { cargo test --workspace } | complete)
    if $tst.exit_code != 0 { err "tests failed:"; print $tst.stderr; exit 1 }
    ok "all tests pass"

    step "cargo doc --no-deps -p weather_man-core …"
    let doc = (do { cargo doc --no-deps -p weather_man-core } | complete)
    if $doc.exit_code != 0 { err "doc generation failed:"; print $doc.stderr; exit 1 }
    ok "docs build cleanly for weather_man-core"
}

def main [--check (-c)] {
    preflight
    if $check {
        list_outdated
        print $"\n(ansi cyan)Dry-run complete. Re-run without --check to apply upgrades.(ansi reset)\n"
    } else {
        list_outdated
        do_upgrade
        cross_check
        header "weather_man · Done"
        ok "All dependencies upgraded and cross-checked."
        print ""
        print "  Next steps:"
        print "    1. Review the diff:       git diff"
        print "    2. Commit the changes:    git commit -am \"chore: upgrade dependencies\""
        print "    3. Push and let CI verify: git push"
        print ""
    }
}

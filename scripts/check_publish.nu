#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# weatherman — Pre-publish checks
# ──────────────────────────────────────────────────────────────────────────────
# Runs documentation checks, a dry-run publish for the core crate, clippy and
# tests before an actual `cargo publish`.
#
# Usage: nu scripts/check_publish.nu
# ──────────────────────────────────────────────────────────────────────────────

# The order crates must be published in (dependencies first). Exported so tests
# and other scripts share one source of truth.
export def publish_order []: nothing -> list<string> {
    ["weatherman-core" "weatherman" "weatherman-tui"]
}

def main [] {
    print "══════════════════════════════════════════════════════════"
    print "  weatherman — Pre-publish checks"
    print "══════════════════════════════════════════════════════════"
    print ""

    print "── Step 1: Documentation checks ──"
    let doc_crates = (publish_order)
    for crate in $doc_crates {
        print $"  📖 Checking docs for ($crate)..."
        let result = (do { cargo doc -p $crate --no-deps } | complete)
        if $result.exit_code != 0 {
            print $"  ❌ Doc check failed for ($crate):"
            print $result.stderr
            exit 1
        }
        print $"  ✅ ($crate) docs OK"
    }
    print ""

    print "── Step 2: Publish dry-run (weatherman-core) ──"
    let publish_result = (do { cargo publish --dry-run -p weatherman-core } | complete)
    if $publish_result.exit_code != 0 {
        print "  ❌ Publish dry-run failed for weatherman-core:"
        print $publish_result.stderr
        exit 1
    }
    print "  ✅ weatherman-core publish dry-run OK"
    print ""

    print "── Step 3: Cargo clippy (workspace) ──"
    let clippy_result = (do { cargo clippy --workspace --all-targets -- -D warnings } | complete)
    if $clippy_result.exit_code != 0 {
        print "  ❌ Clippy found warnings/errors:"
        print $clippy_result.stderr
        exit 1
    }
    print "  ✅ Clippy passed"
    print ""

    print "── Step 4: Cargo test (workspace) ──"
    let test_result = (do { cargo test --workspace } | complete)
    if $test_result.exit_code != 0 {
        print "  ❌ Tests failed:"
        print $test_result.stderr
        exit 1
    }
    print "  ✅ All tests passed"
    print ""

    print "══════════════════════════════════════════════════════════"
    print "  ✅ All pre-publish checks passed!"
    print ""
    print "  Publish order (wait ~30s between each for crates.io indexing):"
    print "    1. cargo publish -p weatherman-core"
    print "    2. cargo publish -p weatherman"
    print "    3. cargo publish -p weatherman-tui"
    print "══════════════════════════════════════════════════════════"
}

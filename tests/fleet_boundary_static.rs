//! Fleet boundary static checks — mechanical proof that sunshine stays on
//! the render side of the fleet's ownership lines.
//!
//! Purpose: turn `specs/fleet-boundaries.md`'s `fb_claim_owner` and
//! `fb_geometry_owner` invariants into build failures instead of review
//! findings.
//! Responsibilities: inspect only the built binary's CLI surface and the
//! checked-in `Cargo.lock`; report the observed names when a boundary is
//! crossed. Neither check reads source or spec text.
//! Rationale: the boundary is a deletion-test invariant, and the cheapest
//! durable evidence is the surface the tool publishes plus the crates it
//! links; deriving the assertions from data files keeps them stable under
//! prose rewording.

use std::collections::BTreeSet;
use std::process::Command;

const BIN: &str = env!("CARGO_BIN_EXE_sunshine");

/// Checked-in allowlist of every package `Cargo.lock` may resolve.
///
/// Adding any dependency, even one the engine denylist would accept, requires
/// an explicit edit to `tests/fleet_boundary_deps.txt`.
const DEPS_SNAPSHOT: &str = include_str!("fleet_boundary_deps.txt");

/// The top level's subcommand set, sorted.
const SUBCOMMANDS: [&str; 2] = ["audit", "render"];

/// The render arguments, sorted.
const RENDER_OPTIONS: [&str; 4] = ["--dsc", "--format", "--manifest", "--out"];

/// The audit arguments, sorted.
const AUDIT_OPTIONS: [&str; 1] = ["--against"];

/// Claim lifecycle (extraction, adoption, invalidation, contradiction)
/// belongs to bajan: no command, flag, or argument may name such a path.
const CLAIM_DENIED: [&str; 8] = [
    "create", "add", "adopt", "mutate", "invalidate", "contradict", "store", "write-claim",
];

/// PDF text extraction and OCR engines belong to other fleet components.
const ENGINE_DENIED: [&str; 8] = [
    "pdf-extract", "pdfium", "mupdf", "pdfminer", "pdf-rs", "pdftotext", "poppler", "tesseract",
];

/// Geometry authority is sunset's: no flag or argument may name text
/// extraction or locator derivation.
const GEOMETRY_DENIED: [&str; 5] = ["extract", "page-range", "coordinates", "bbox", "ocr"];

/// Parsed `--help` surface of one command level.
#[derive(Default)]
struct HelpSurface {
    commands: Vec<String>,
    options: Vec<String>,
    positionals: Vec<String>,
}

impl HelpSurface {
    fn all_names(&self) -> Vec<String> {
        self.commands
            .iter()
            .chain(self.options.iter())
            .chain(self.positionals.iter())
            .cloned()
            .collect()
    }
}

/// Run the binary with `args` and require a successful help exit.
fn help(args: &[&str]) -> String {
    let output = Command::new(BIN)
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("failed to run {BIN} {args:?}: {e}"));
    assert_eq!(
        output.status.code(),
        Some(0),
        "{BIN} {args:?} must exit 0; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .unwrap_or_else(|e| panic!("{BIN} {args:?} help was not UTF-8: {e}"))
}

/// Take the name column of a clap help entry.
fn entry_head(entry: &str) -> &str {
    match entry.find("  ") {
        Some(at) => &entry[..at],
        None => entry,
    }
}

/// Pair every indented help entry with the section heading above it.
fn section_entries(text: &str) -> Vec<(&str, &str)> {
    let mut section = "";
    let mut entries = Vec::new();
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        if line.starts_with(char::is_whitespace) {
            entries.push((section, line.trim_start()));
        } else {
            section = line.trim();
        }
    }
    entries
}

/// Parse every entry under `section`, flattening each entry's names.
fn entries_in(
    entries: &[(&str, &str)],
    section: &str,
    parse: fn(&str) -> Vec<String>,
) -> Vec<String> {
    entries
        .iter()
        .filter(|(name, _)| *name == section)
        .flat_map(|(_, entry)| parse(entry))
        .collect()
}

fn command_names(entry: &str) -> Vec<String> {
    entry
        .split_whitespace()
        .next()
        .map(|name| name.trim_end_matches(',').to_string())
        .into_iter()
        .collect()
}

fn option_names(entry: &str) -> Vec<String> {
    entry_head(entry)
        .split_whitespace()
        .filter_map(|token| token.trim_end_matches(',').split('=').next())
        .filter_map(|token| token.strip_prefix("--"))
        .map(|long| format!("--{long}"))
        .collect()
}

fn positional_name(entry: &str) -> Vec<String> {
    let name = entry_head(entry).split_whitespace().next();
    name.map(|name| {
        name.trim_start_matches('<')
            .trim_end_matches('>')
            .to_ascii_lowercase()
    })
    .into_iter()
    .collect()
}

fn parse_help(text: &str) -> HelpSurface {
    let entries = section_entries(text);
    HelpSurface {
        commands: entries_in(&entries, "Commands:", command_names),
        options: entries_in(&entries, "Options:", option_names),
        positionals: entries_in(&entries, "Arguments:", positional_name),
    }
}

fn normalize(name: &str) -> String {
    name.to_ascii_lowercase().replace('_', "-")
}

/// Match a denied capability token on `-`-delimited segments, so short tokens
/// such as `add` or `ocr` cannot fire on unrelated names like `address` or
/// `micro`.
fn matches_denied(name: &str, token: &str) -> bool {
    let normalized_name = normalize(name);
    let normalized_token = normalize(token);
    let segments: Vec<&str> = normalized_name.split('-').collect();
    let denied: Vec<&str> = normalized_token.split('-').collect();
    segments
        .windows(denied.len())
        .any(|window| window == denied.as_slice())
}

/// Every `name matches token` pair among `names`, in observed order.
fn denied_hits(names: &[String], denied: &[&str]) -> Vec<String> {
    names
        .iter()
        .flat_map(|name| denied.iter().map(move |token| (name, token)))
        .filter(|(name, token)| matches_denied(name, token))
        .map(|(name, token)| format!("{name} matches {token:?}"))
        .collect()
}

/// Names on `surface` must not imply any of the denied capabilities.
fn assert_no_denied_names(surface_name: &str, surface: &HelpSurface, denied: &[&str]) {
    let names = surface.all_names();
    let hits = denied_hits(&names, denied);
    assert!(
        hits.is_empty(),
        "{surface_name} exposes names implying a foreign capability: {hits:?}; observed names: {names:?}"
    );
}

/// Run the denied-name check over each named command level.
fn assert_denied_names_absent(levels: &[(&str, &HelpSurface)], denied: &[&str]) {
    for &(surface_name, surface) in levels {
        assert_no_denied_names(surface_name, surface, denied);
    }
}

/// clap-generated conveniences are not application surface.
fn strip_clap_builtins(names: &[String]) -> Vec<String> {
    names
        .iter()
        .filter(|name| !matches!(name.as_str(), "--help" | "--version" | "-h" | "-V"))
        .cloned()
        .collect()
}

/// The three CLI levels the boundary pins: top level, render, and audit.
fn surfaces() -> (HelpSurface, HelpSurface, HelpSurface) {
    (
        parse_help(&help(&["--help"])),
        parse_help(&help(&["render", "--help"])),
        parse_help(&help(&["audit", "--help"])),
    )
}

/// Application options in stable order, with clap's builtins removed.
fn sorted_app_options(surface: &HelpSurface) -> Vec<String> {
    let mut names = strip_clap_builtins(&surface.options);
    names.sort();
    names
}

/// Application commands in stable order, with clap's `help` removed.
fn app_commands(top: &HelpSurface) -> Vec<String> {
    let mut names: Vec<String> = top
        .commands
        .iter()
        .filter(|name| *name != "help")
        .cloned()
        .collect();
    names.sort();
    names
}

fn strings(names: &[&str]) -> Vec<String> {
    names.iter().map(|name| name.to_string()).collect()
}

/// An observed name set must equal `expected` exactly.
fn assert_same_names(context: &str, observed: Vec<String>, expected: &[&str]) {
    let expected = strings(expected);
    assert_eq!(
        observed, expected,
        "{context} must be exactly {expected:?}; observed: {observed:?}"
    );
}

fn assert_exact_positionals(surface_name: &str, surface: &HelpSurface, expected: &[&str]) {
    let expected = strings(expected);
    assert_eq!(
        surface.positionals,
        expected,
        "{surface_name} must expose exactly the positional(s) {expected:?}"
    );
}

fn assert_no_nested_commands(levels: &[(&str, &HelpSurface)]) {
    for &(surface_name, surface) in levels {
        assert!(
            surface.commands.is_empty(),
            "{surface_name} must expose no nested subcommand; observed: {:?}",
            surface.commands
        );
    }
}

#[test]
fn p_claim_owner() {
    let (top, render, audit) = surfaces();

    assert_same_names("subcommand set", app_commands(&top), &SUBCOMMANDS);
    let top_options = sorted_app_options(&top);
    assert!(
        top_options.is_empty(),
        "the top level must publish no application option beyond clap builtins; observed: {top_options:?}"
    );
    assert_same_names(
        "render arguments",
        sorted_app_options(&render),
        &RENDER_OPTIONS,
    );
    assert_same_names("audit arguments", sorted_app_options(&audit), &AUDIT_OPTIONS);
    assert_exact_positionals("audit", &audit, &["artifact"]);

    let claim_levels = [
        ("top-level CLI", &top),
        ("render", &render),
        ("audit", &audit),
    ];
    assert_denied_names_absent(&claim_levels, &CLAIM_DENIED);

    // The claim input surface is pinned structurally above by the exact
    // option, positional, and command sets: render takes input only through
    // its four flags, audit only through --against plus its one positional,
    // and neither level nests further commands.
    assert_exact_positionals("render", &render, &[]);
    assert_no_nested_commands(&[("render", &render), ("audit", &audit)]);
}

/// Package names parsed from `Cargo.lock`'s `name = "..."` entries.
fn parse_lock_packages(lock: &str) -> BTreeSet<String> {
    lock.lines()
        .filter_map(|line| line.trim().strip_prefix("name = "))
        .map(|value| value.trim().trim_matches('"').to_string())
        .collect()
}

/// Package names parsed from the checked-in allowlist snapshot.
fn parse_snapshot_names(snapshot: &str) -> BTreeSet<String> {
    snapshot
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}

/// Names in `left` that `right` lacks.
fn only_in(left: &BTreeSet<String>, right: &BTreeSet<String>) -> Vec<String> {
    left.difference(right).cloned().collect()
}

fn lock_packages() -> BTreeSet<String> {
    let lock_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock");
    let lock = std::fs::read_to_string(&lock_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", lock_path.display()));
    let packages = parse_lock_packages(&lock);
    assert!(
        !packages.is_empty(),
        "no package names parsed from {}",
        lock_path.display()
    );
    packages
}

/// Fast, readable error path: a denied engine name reports why it is foreign,
/// which set equality alone cannot.
fn assert_no_denied_packages(packages: &BTreeSet<String>) {
    let names: Vec<String> = packages.iter().cloned().collect();
    let violations = denied_hits(&names, &ENGINE_DENIED);
    assert!(
        violations.is_empty(),
        "dependency list contains a PDF text-extraction or OCR engine: {violations:?}; observed dependencies: {packages:?}"
    );
}

#[test]
fn p_geometry_owner() {
    // The snapshot is the allowlist: set equality catches any resolved
    // package drift, including additions the denylist would accept. Any
    // dependency addition requires an explicit edit to the snapshot.
    let packages = lock_packages();
    let expected = parse_snapshot_names(DEPS_SNAPSHOT);
    let unexpected = only_in(&packages, &expected);
    assert!(
        unexpected.is_empty(),
        "Cargo.lock package set drifted from tests/fleet_boundary_deps.txt; unexpected: {unexpected:?}"
    );
    let missing = only_in(&expected, &packages);
    assert!(
        missing.is_empty(),
        "Cargo.lock package set drifted from tests/fleet_boundary_deps.txt; missing: {missing:?}"
    );
    assert_no_denied_packages(&packages);

    let (top, render, audit) = surfaces();

    // The render input surface is pinned exactly so this scenario observes
    // the CLI even when run alone: four flags, no positional, no nesting.
    assert_same_names(
        "render arguments",
        sorted_app_options(&render),
        &RENDER_OPTIONS,
    );
    assert_exact_positionals("render", &render, &[]);
    assert_no_nested_commands(&[("render", &render), ("audit", &audit)]);

    // Geometry authority is sunset's: no flag or argument may name text
    // extraction or locator derivation.
    let geometry_levels = [
        ("top-level CLI", &top),
        ("render", &render),
        ("audit", &audit),
    ];
    assert_denied_names_absent(&geometry_levels, &GEOMETRY_DENIED);
}

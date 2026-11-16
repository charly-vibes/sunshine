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

/// Parsed `--help` surface of one command level.
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

fn parse_help(text: &str) -> HelpSurface {
    let mut surface = HelpSurface {
        commands: Vec::new(),
        options: Vec::new(),
        positionals: Vec::new(),
    };
    let mut section = "";
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if !line.starts_with(char::is_whitespace) {
            section = line.trim();
            continue;
        }
        let entry = line.trim_start();
        match section {
            "Commands:" => {
                if let Some(name) = entry.split_whitespace().next() {
                    surface.commands.push(name.trim_end_matches(',').to_string());
                }
            }
            "Options:" => {
                let head = entry_head(entry);
                for token in head.split_whitespace() {
                    let token = token.trim_end_matches(',').split('=').next().unwrap_or("");
                    if let Some(long) = token.strip_prefix("--") {
                        surface.options.push(format!("--{long}"));
                    }
                }
            }
            "Arguments:" => {
                let head = entry_head(entry);
                if let Some(name) = head.split_whitespace().next() {
                    let name = name
                        .trim_start_matches('<')
                        .trim_end_matches('>')
                        .to_ascii_lowercase();
                    surface.positionals.push(name);
                }
            }
            _ => {}
        }
    }
    surface
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

/// Names on `surface` must not imply any of the denied capabilities.
fn assert_no_denied_names(surface_name: &str, surface: &HelpSurface, denied: &[&str]) {
    let names = surface.all_names();
    let mut hits = Vec::new();
    for name in &names {
        for token in denied {
            if matches_denied(name, token) {
                hits.push(format!("{name} matches {token:?}"));
            }
        }
    }
    assert!(
        hits.is_empty(),
        "{surface_name} exposes names implying a foreign capability: {hits:?}; observed names: {names:?}"
    );
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

#[test]
fn p_claim_owner() {
    let (top, render, audit) = surfaces();

    let mut subcommands: Vec<String> = top
        .commands
        .iter()
        .filter(|name| *name != "help")
        .cloned()
        .collect();
    subcommands.sort();
    assert_eq!(
        subcommands,
        vec!["audit".to_string(), "render".to_string()],
        "subcommand set must be exactly {{render, audit}}; observed: {:?}",
        top.commands
    );

    let top_options = sorted_app_options(&top);
    assert!(
        top_options.is_empty(),
        "the top level must publish no application option beyond clap builtins; observed: {top_options:?}"
    );

    assert_eq!(
        sorted_app_options(&render),
        vec![
            "--dsc".to_string(),
            "--format".to_string(),
            "--manifest".to_string(),
            "--out".to_string()
        ],
        "render arguments must be exactly {{--manifest, --dsc, --format, --out}}; observed: {:?}",
        render.options
    );

    assert_eq!(
        sorted_app_options(&audit),
        vec!["--against".to_string()],
        "audit arguments must be exactly {{--against}}; observed: {:?}",
        audit.options
    );
    assert_eq!(
        audit.positionals,
        vec!["artifact".to_string()],
        "audit must take exactly one positional artifact; observed: {:?}",
        audit.positionals
    );

    // Claim lifecycle (extraction, adoption, invalidation, contradiction)
    // belongs to bajan: no command, flag, or argument may name such a path.
    let claim_denied = [
        "create",
        "add",
        "adopt",
        "mutate",
        "invalidate",
        "contradict",
        "store",
        "write-claim",
    ];
    for (surface_name, surface) in [
        ("top-level CLI", &top),
        ("render", &render),
        ("audit", &audit),
    ] {
        assert_no_denied_names(surface_name, surface, &claim_denied);
    }

    // The claim input surface is pinned structurally above by the exact
    // option, positional, and command sets: render takes input only through
    // its four flags, audit only through --against plus its one positional,
    // and neither level nests further commands.
    assert!(
        render.positionals.is_empty(),
        "render must take no positional input; observed: {:?}",
        render.positionals
    );
    for (surface_name, surface) in [("render", &render), ("audit", &audit)] {
        assert!(
            surface.commands.is_empty(),
            "{surface_name} must expose no nested subcommand; observed: {:?}",
            surface.commands
        );
    }
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

#[test]
fn p_geometry_owner() {
    let lock_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock");
    let lock = std::fs::read_to_string(&lock_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", lock_path.display()));

    let packages = parse_lock_packages(&lock);
    assert!(
        !packages.is_empty(),
        "no package names parsed from {}",
        lock_path.display()
    );

    // The snapshot is the allowlist: set equality catches any resolved
    // package drift, including additions the denylist would accept. Any
    // dependency addition requires an explicit edit to the snapshot.
    let expected = parse_snapshot_names(DEPS_SNAPSHOT);
    let unexpected: Vec<&str> = packages.difference(&expected).map(String::as_str).collect();
    let missing: Vec<&str> = expected.difference(&packages).map(String::as_str).collect();
    assert!(
        unexpected.is_empty() && missing.is_empty(),
        "Cargo.lock package set drifted from tests/fleet_boundary_deps.txt; unexpected: {unexpected:?}; missing: {missing:?}"
    );

    // Fast, readable error path: a denied engine name reports why it is
    // foreign, which set equality alone cannot.
    let engine_denied = [
        "pdf-extract",
        "pdfium",
        "mupdf",
        "pdfminer",
        "pdf-rs",
        "pdftotext",
        "poppler",
        "tesseract",
    ];
    let mut violations = Vec::new();
    for package in &packages {
        for token in &engine_denied {
            if matches_denied(package, token) {
                violations.push(format!("{package} matches {token:?}"));
            }
        }
    }
    assert!(
        violations.is_empty(),
        "dependency list contains a PDF text-extraction or OCR engine: {violations:?}; observed dependencies: {packages:?}"
    );

    let (top, render, audit) = surfaces();

    // The render input surface is pinned exactly so this scenario observes
    // the CLI even when run alone: four flags, no positional, no nesting.
    assert_eq!(
        sorted_app_options(&render),
        vec![
            "--dsc".to_string(),
            "--format".to_string(),
            "--manifest".to_string(),
            "--out".to_string()
        ],
        "render arguments must be exactly {{--manifest, --dsc, --format, --out}}; observed: {:?}",
        render.options
    );
    assert!(
        render.positionals.is_empty(),
        "render must take no positional input; observed: {:?}",
        render.positionals
    );
    for (surface_name, surface) in [("render", &render), ("audit", &audit)] {
        assert!(
            surface.commands.is_empty(),
            "{surface_name} must expose no nested subcommand; observed: {:?}",
            surface.commands
        );
    }

    // Geometry authority is sunset's: no flag or argument may name text
    // extraction or locator derivation.
    let geometry_denied = ["extract", "page-range", "coordinates", "bbox", "ocr"];
    for (surface_name, surface) in [
        ("top-level CLI", &top),
        ("render", &render),
        ("audit", &audit),
    ] {
        assert_no_denied_names(surface_name, surface, &geometry_denied);
    }
}
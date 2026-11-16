//! sunshine — compile claim anchor manifests into tagged, auditable documents.
//!
//! The render layer of the charly-vibes fleet: sunset owns geometry (DSC
//! descriptors), bajan owns claims (the claim graph), sunshine owns only
//! the transformation of claim anchors into rendered artifacts — see
//! `specs/fleet-boundaries.md`.

mod envelope;

use clap::{Parser, Subcommand};
use envelope::{ok_envelope, EnvelopeKind, WeaveError};
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "sunshine",
    about = "Compile claim anchor manifests into tagged, auditable documents",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Render a manifest + DSC descriptor into a tagged document
    Render {
        /// Claim anchor manifest (JSON) exported from bajan
        #[arg(long)]
        manifest: std::path::PathBuf,
        /// DSC descriptor (JSON) describing document structure
        #[arg(long)]
        dsc: std::path::PathBuf,
        /// Output format (closed set in v1: pdf, md, org, html)
        #[arg(long)]
        format: String,
        /// Output path
        #[arg(long)]
        out: std::path::PathBuf,
    },
    /// Re-parse a rendered artifact and diff it against its manifest
    Audit {
        /// The rendered artifact to audit
        artifact: std::path::PathBuf,
        /// The manifest the artifact must round-trip to
        #[arg(long)]
        against: std::path::PathBuf,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Render { manifest, dsc, format, out } => {
            render(&manifest, &dsc, &format, &out)
        }
        Command::Audit { artifact, against } => audit(&artifact, &against),
    };
    match result {
        Ok(env) => {
            println!("{}", serde_json::to_string_pretty(&env).expect("envelope serializes"));
            if env.ok { ExitCode::SUCCESS } else { ExitCode::FAILURE }
        }
        Err(err) => {
            println!("{}", serde_json::to_string_pretty(&err.to_envelope()).expect("envelope serializes"));
            ExitCode::FAILURE
        }
    }
}

/// `specs/render-contract.md` (`rc_purity`): render is a pure function of
/// (manifest, descriptor, source). v1 wires the contract surface; the
/// backends land spec-first (roadmap in `.beads/issues.jsonl`).
fn render(
    manifest: &std::path::Path,
    _dsc: &std::path::Path,
    format: &str,
    _out: &std::path::Path,
) -> Result<envelope::Envelope, WeaveError> {
    // rc_closed_formats: the format set is closed and enumerated.
    const FORMATS: [&str; 4] = ["pdf", "md", "org", "html"];
    if !FORMATS.contains(&format) {
        return Err(WeaveError::new(
            "E_FORMAT",
            format!("unknown format: {format}"),
            format!("use one of: {}", FORMATS.join(", ")),
        ));
    }
    // rc_input_present: refuse missing inputs loudly, before any work.
    if !manifest.exists() {
        return Err(WeaveError::new(
            "E_MANIFEST_MISSING",
            format!("manifest not found: {}", manifest.display()),
            "pass --manifest with a bajan-exported claim anchor manifest",
        ));
    }
    Err(WeaveError::new(
        "E_UNIMPLEMENTED",
        "backend not yet implemented (v0.1 wires the contract surface)",
        "track roadmap issues in .beads/issues.jsonl; start with the anchor.manifest spec",
    ))
}

/// `specs/audit-contract.md` (`ac_roundtrip`): audit(render(m)) == m under
/// published canonicalization. v0.1 refuses artifacts it has no parser for.
fn audit(_artifact: &std::path::Path, against: &std::path::Path) -> Result<envelope::Envelope, WeaveError> {
    if !against.exists() {
        return Err(WeaveError::new(
            "E_MANIFEST_MISSING",
            format!("manifest not found: {}", against.display()),
            "pass --against with the manifest the artifact was rendered from",
        ));
    }
    Err(WeaveError::new(
        "E_UNIMPLEMENTED",
        "audit parser not yet implemented (v0.1 wires the contract surface)",
        "track roadmap issues in .beads/issues.jsonl; every backend ships with its audit parser or does not ship",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_format_is_refused_with_remediation() {
        let err = render(std::path::Path::new("/nonexistent"), std::path::Path::new("/nonexistent"), "rst", std::path::Path::new("/tmp/out"))
            .expect_err("rst is not in the closed format set");
        assert_eq!(err.code, "E_FORMAT");
        assert!(err.remediation.contains("pdf"), "remediation names the closed set");
    }

    #[test]
    fn missing_manifest_refused_before_format_work() {
        let err = render(std::path::Path::new("/nonexistent"), std::path::Path::new("/nonexistent"), "md", std::path::Path::new("/tmp/out"))
            .expect_err("missing manifest must refuse");
        assert_eq!(err.code, "E_MANIFEST_MISSING");
    }
}

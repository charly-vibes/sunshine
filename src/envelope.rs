//! Suite envelope — the fleet's single answer shape for every command.
//!
//! Every weave command answers `{ok, envelope_version, cli_version,
//! envelope_kind, data}` (see `specs/render-contract.md`, `rc_envelope`):
//! exit status mirrors `ok`, and every error carries a non-empty
//! remediation hint. Pure serialization, no I/O.

pub const ENVELOPE_VERSION: &str = "0.1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvelopeKind {
    Ok,
    Error,
    Render,
    Audit,
}

impl EnvelopeKind {
    pub fn as_str(self) -> &'static str {
        match self {
            EnvelopeKind::Ok => "ok",
            EnvelopeKind::Error => "error",
            EnvelopeKind::Render => "render",
            EnvelopeKind::Audit => "audit",
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct Envelope {
    pub ok: bool,
    pub envelope_version: &'static str,
    pub cli_version: &'static str,
    pub envelope_kind: &'static str,
    pub data: serde_json::Value,
}

pub const CLI_VERSION: &str = env!("CARGO_PKG_VERSION");

/// A machine-readable error with a remediation hint (`rc_envelope`:
/// every error carries non-empty remediation).
#[derive(Debug)]
pub struct WeaveError {
    pub code: String,
    pub message: String,
    pub remediation: String,
}

impl WeaveError {
    pub fn new(code: impl Into<String>, message: impl Into<String>, remediation: impl Into<String>) -> Self {
        Self { code: code.into(), message: message.into(), remediation: remediation.into() }
    }

    pub fn to_envelope(&self) -> Envelope {
        Envelope {
            ok: false,
            envelope_version: ENVELOPE_VERSION,
            cli_version: CLI_VERSION,
            envelope_kind: EnvelopeKind::Error.as_str(),
            data: serde_json::json!({
                "code": self.code,
                "message": self.message,
                "remediation": self.remediation,
            }),
        }
    }
}

pub fn ok_envelope(kind: EnvelopeKind, data: serde_json::Value) -> Envelope {
    Envelope {
        ok: true,
        envelope_version: ENVELOPE_VERSION,
        cli_version: CLI_VERSION,
        envelope_kind: kind.as_str(),
        data,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_envelope_carries_nonempty_remediation() {
        let err = WeaveError::new("E_FORMAT", "unknown format", "use one of: pdf, md, org, html");
        let env = err.to_envelope();
        assert!(!env.ok);
        let rem = env.data["remediation"].as_str().unwrap();
        assert!(!rem.trim().is_empty(), "remediation must be non-empty");
    }

    #[test]
    fn ok_envelope_shape() {
        let env = ok_envelope(EnvelopeKind::Render, serde_json::json!({"outputs": []}));
        assert!(env.ok);
        assert_eq!(env.envelope_version, "0.1");
        assert_eq!(env.envelope_kind, "render");
    }
}

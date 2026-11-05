//! Failures returned by backend and frontend calls.

use thiserror::Error;

/// How serious a backend failure is for the caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// The operation cannot continue.
    Fatal,
    /// The operation finished with a non-fatal problem.
    Warning,
}

/// Why a backend call failed.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    /// No enumerated adapter satisfied the device descriptor.
    #[error("No compatible physical device found")]
    NoCompatibleDevice,

    /// The backend rejected an operation it does not implement.
    #[error("Unsupported operation: {0}")]
    Unsupported(&'static str),

    /// A backend-specific failure, carrying a display message.
    #[error("Backend error: {0}")]
    Backend(String),
}

/// Failure returned by backend and frontend calls.
#[derive(Debug)]
pub struct Error {
    /// Classification of the failure.
    pub kind: ErrorKind,
    /// Whether the caller should treat the failure as fatal.
    pub severity: Severity,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}] {}", self.severity, self.kind)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.kind)
    }
}

impl Error {
    /// Builds a fatal error.
    pub fn fatal(kind: ErrorKind) -> Self {
        Self {
            kind,
            severity: Severity::Fatal,
        }
    }

    /// Builds a warning error.
    pub fn warning(kind: ErrorKind) -> Self {
        Self {
            kind,
            severity: Severity::Warning,
        }
    }

    /// Builds a fatal [`ErrorKind::Unsupported`] error.
    pub fn unsupported(message: &'static str) -> Self {
        Self::fatal(ErrorKind::Unsupported(message))
    }
}

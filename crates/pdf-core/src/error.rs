use std::path::PathBuf;

use thiserror::Error;

/// Errors produced by the PDF core.
///
/// Variants intentionally stay engine-agnostic: callers should be able to
/// replace `lopdf` with another engine without rewriting the error handling.
#[derive(Debug, Error)]
pub enum PdfCoreError {
    #[error("could not read the file at {path}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("the file is not a valid PDF: {0}")]
    InvalidPdf(String),

    #[error("the document is encrypted and requires a password")]
    PasswordRequired,

    #[error("the password is incorrect")]
    WrongPassword,

    #[error("page {page} is out of range: the document has {total} page(s)")]
    PageOutOfRange { page: u32, total: u32 },

    #[error("{0}")]
    Other(String),
}

pub type PdfCoreResult<T> = Result<T, PdfCoreError>;

impl PdfCoreError {
    /// Map an engine error to a core error, distinguishing encryption problems
    /// so the UI can ask for a password instead of reporting a corrupt file.
    pub(crate) fn from_engine(error: impl std::fmt::Display) -> Self {
        let message = error.to_string();
        let lower = message.to_ascii_lowercase();
        if lower.contains("password") || lower.contains("encrypt") {
            PdfCoreError::PasswordRequired
        } else {
            PdfCoreError::InvalidPdf(message)
        }
    }
}

use std::path::{Path, PathBuf};

use lopdf::Document;

use crate::error::{PdfCoreError, PdfCoreResult};

/// A loaded PDF document.
///
/// Wraps the current engine's document type. All tool modules operate through
/// this type so the engine stays replaceable.
pub struct PdfDocument {
    inner: Document,
    source: PathBuf,
}

impl PdfDocument {
    /// Open a PDF from disk.
    pub fn open(path: impl AsRef<Path>) -> PdfCoreResult<Self> {
        let path = path.as_ref();
        let inner = Document::load(path).map_err(|error| map_load_error(path, error))?;
        Ok(Self {
            inner,
            source: path.to_path_buf(),
        })
    }

    /// Load a PDF from memory. Used by tests and by tools that build documents.
    pub fn from_bytes(bytes: &[u8]) -> PdfCoreResult<Self> {
        let inner = Document::load_mem(bytes).map_err(PdfCoreError::from_engine)?;
        Ok(Self {
            inner,
            source: PathBuf::new(),
        })
    }

    /// Number of pages in the document.
    pub fn page_count(&self) -> u32 {
        self.inner.get_pages().len() as u32
    }

    /// The path this document was loaded from (empty when loaded from memory).
    pub fn source_path(&self) -> &Path {
        &self.source
    }
}

fn map_load_error(path: &Path, error: lopdf::Error) -> PdfCoreError {
    // A read failure should be reported as I/O; everything else is treated as
    // a malformed or protected PDF by `from_engine`.
    if let lopdf::Error::IO(io) = &error {
        return PdfCoreError::Io {
            path: path.to_path_buf(),
            source: std::io::Error::new(io.kind(), io.to_string()),
        };
    }
    PdfCoreError::from_engine(error)
}

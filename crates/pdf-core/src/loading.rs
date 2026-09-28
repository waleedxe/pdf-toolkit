//! Central document loading.
//!
//! All PDF loads go through here so untrusted input is handled consistently and
//! bounded. `max_decompressed_size` caps how large any single stream may expand
//! to while loading, defending against "decompression bomb" PDFs.

use std::path::Path;

use lopdf::{Document, LoadOptions};

use crate::error::{PdfCoreError, PdfCoreResult};

/// Upper bound (256 MiB) on the decompressed size of any single stream during
/// loading of an untrusted PDF.
pub const MAX_DECOMPRESSED_BYTES: usize = 256 * 1024 * 1024;

/// Load a PDF from disk with resource limits applied.
pub(crate) fn load(path: &Path) -> PdfCoreResult<Document> {
    load_with(path, options())
}

/// Load a PDF from memory with resource limits applied.
pub(crate) fn load_mem(bytes: &[u8]) -> PdfCoreResult<Document> {
    Document::load_mem_with_options(bytes, options()).map_err(PdfCoreError::from_engine)
}

fn options() -> LoadOptions {
    LoadOptions::with_max_decompressed_size(MAX_DECOMPRESSED_BYTES)
}

fn load_with(path: &Path, options: LoadOptions) -> PdfCoreResult<Document> {
    Document::load_with_options(path, options).map_err(|error| map_load_error(path, error))
}

fn map_load_error(path: &Path, error: lopdf::Error) -> PdfCoreError {
    if let lopdf::Error::IO(io) = &error {
        return PdfCoreError::Io {
            path: path.to_path_buf(),
            source: std::io::Error::new(io.kind(), io.to_string()),
        };
    }
    PdfCoreError::from_engine(error)
}

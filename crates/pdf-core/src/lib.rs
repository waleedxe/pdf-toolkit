//! Engine-agnostic PDF core for PDF Toolkit.
//!
//! This crate owns all real PDF manipulation logic and hides the underlying
//! engine (currently `lopdf`) behind stable, testable APIs. The Tauri layer
//! only calls into this crate; it never talks to the engine directly.
//!
//! Keeping the core as a standalone library means it can be unit-tested on any
//! platform (including CI and Linux dev machines) without the Tauri GUI
//! dependencies.

pub mod document;
pub mod error;
pub mod tools;

pub use document::PdfDocument;
pub use error::{PdfCoreError, PdfCoreResult};
pub use tools::{
    extract_pages, merge, page_count, parse_page_selection, remove_pages, rotate_pages, split_pdf,
};

/// Version of the PDF core crate, surfaced through the app bridge.
pub const fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

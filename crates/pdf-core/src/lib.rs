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
pub mod text;
pub mod tools;

pub use document::PdfDocument;
pub use error::{PdfCoreError, PdfCoreResult};
pub use text::{embed_font, shape_text, ShapedText, EMBEDDED_FONT, EMBEDDED_FONT_NAME};
pub use tools::{
    add_page_numbers, extract_pages, images_to_pdf, merge, page_count, parse_page_selection,
    protect, remove_pages, rotate_pages, split_pdf, unlock, watermark_image, watermark_text,
    FitMode, Horizontal, ImageWatermark, ImagesToPdfOptions, Orientation, PageNumberOptions,
    PageSize, Position, ProtectOptions, TextWatermark, Vertical,
};

/// Version of the PDF core crate, surfaced through the app bridge.
pub const fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

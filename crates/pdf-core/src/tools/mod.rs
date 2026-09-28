//! PDF tool operations.
//!
//! Each tool lives in its own module and exposes a plain Rust function. The
//! Tauri layer wraps these; nothing here knows about Tauri or the UI.

pub mod merge;
pub mod pages;
pub mod range;

pub use merge::merge;
pub use pages::{extract_pages, page_count, remove_pages, rotate_pages, split_pdf};
pub use range::parse_page_selection;

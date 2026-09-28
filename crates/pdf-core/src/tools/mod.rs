//! PDF tool operations.
//!
//! Each tool lives in its own module and exposes a plain Rust function. The
//! Tauri layer wraps these; nothing here knows about Tauri or the UI.

mod content;

pub mod images;
pub mod merge;
pub mod page_numbers;
pub mod pages;
pub mod range;
pub mod security;
pub mod watermark;

pub use images::{images_to_pdf, FitMode, ImagesToPdfOptions, Orientation, PageSize};
pub use merge::merge;
pub use page_numbers::{add_page_numbers, Horizontal, PageNumberOptions, Vertical};
pub use pages::{extract_pages, page_count, remove_pages, rotate_pages, split_pdf};
pub use range::parse_page_selection;
pub use security::{protect, unlock, ProtectOptions};
pub use watermark::{watermark_image, watermark_text, ImageWatermark, Position, TextWatermark};

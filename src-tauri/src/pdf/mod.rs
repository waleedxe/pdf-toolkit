//! PDF service layer (thin re-export).
//!
//! All orchestration lives in the `pdf-service` crate so it is fully testable
//! without the Tauri GUI dependencies. This module keeps the `pdf::` path used
//! by the command layer and preserves the adapter boundary.

pub use pdf_service::*;

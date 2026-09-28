//! Tauri application entry point.
//!
//! This layer is intentionally thin: it registers plugins and commands, and
//! delegates all real work to module services (`commands`, `pdf`), which in
//! turn call the engine-agnostic `pdf-core` crate.

mod commands;
mod pdf;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(commands::ThumbnailCache::default())
        .invoke_handler(tauri::generate_handler![
            commands::app_info,
            commands::merge_pdfs,
            commands::extract_pages,
            commands::remove_pages,
            commands::rotate_pages,
            commands::split_pdf,
            commands::pdf_info,
            commands::render_thumbnail,
            commands::reorder_pages,
            commands::pdf_to_images,
            commands::images_to_pdf,
            commands::watermark_pdf,
            commands::add_page_numbers,
            commands::protect_pdf,
            commands::unlock_pdf
        ])
        .run(tauri::generate_context!())
        .expect("error while running the PDF Toolkit application");
}

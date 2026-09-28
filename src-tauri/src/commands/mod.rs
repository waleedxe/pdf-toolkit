//! Tauri command surface.
//!
//! Commands stay thin: validate arguments, call a service in `pdf`, and return
//! serializable results. All heavy lifting happens off the UI thread in the
//! job system (added in a later phase).

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Basic application information, used to verify the frontend/bridge wiring.
#[derive(Debug, Serialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub pdf_core_version: String,
}

#[tauri::command]
pub fn app_info() -> AppInfo {
    AppInfo {
        name: "PDF Toolkit".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        pdf_core_version: pdf_core::version().to_string(),
    }
}

/// Path of the file a tool produced, plus how many pages it contains.
#[derive(Debug, Serialize)]
pub struct OperationResult {
    pub output: String,
    pub page_count: u32,
}

#[derive(Debug, Deserialize)]
pub struct MergeRequest {
    pub inputs: Vec<String>,
    pub output: String,
}

#[derive(Debug, Serialize)]
pub struct MergeResult {
    pub output: String,
    pub page_count: u32,
}

/// Merge PDFs. The originals are never modified; a new file is written.
#[tauri::command]
pub fn merge_pdfs(request: MergeRequest) -> Result<MergeResult, String> {
    let inputs: Vec<PathBuf> = request.inputs.iter().map(PathBuf::from).collect();
    let page_count =
        pdf::merge(&inputs, Path::new(&request.output)).map_err(|error| error.to_string())?;

    Ok(MergeResult {
        output: request.output,
        page_count,
    })
}

#[derive(Debug, Deserialize)]
pub struct ExtractRequest {
    pub input: String,
    pub pages: String,
    pub output: String,
}

/// Extract pages such as `1,4,7` or `1-4,8,11-15` into a new PDF.
#[tauri::command]
pub fn extract_pages(request: ExtractRequest) -> Result<OperationResult, String> {
    let page_count = pdf::extract(
        Path::new(&request.input),
        &request.pages,
        Path::new(&request.output),
    )
    .map_err(|error| error.to_string())?;

    Ok(OperationResult {
        output: request.output,
        page_count,
    })
}

#[derive(Debug, Deserialize)]
pub struct RemoveRequest {
    pub input: String,
    pub pages: String,
    pub output: String,
}

/// Remove pages and save the remaining ones to a new PDF.
#[tauri::command]
pub fn remove_pages(request: RemoveRequest) -> Result<OperationResult, String> {
    let page_count = pdf::remove(
        Path::new(&request.input),
        &request.pages,
        Path::new(&request.output),
    )
    .map_err(|error| error.to_string())?;

    Ok(OperationResult {
        output: request.output,
        page_count,
    })
}

#[derive(Debug, Deserialize)]
pub struct RotateRequest {
    pub input: String,
    /// Empty or omitted means "all pages".
    #[serde(default)]
    pub pages: Option<String>,
    pub rotation: i64,
    pub output: String,
}

/// Rotate all or selected pages by a multiple of 90 degrees.
#[tauri::command]
pub fn rotate_pages(request: RotateRequest) -> Result<OperationResult, String> {
    let page_count = pdf::rotate(
        Path::new(&request.input),
        request.pages.as_deref(),
        request.rotation,
        Path::new(&request.output),
    )
    .map_err(|error| error.to_string())?;

    Ok(OperationResult {
        output: request.output,
        page_count,
    })
}

#[derive(Debug, Deserialize)]
pub struct SplitRequest {
    pub input: String,
    /// Either `every-page` or `ranges`.
    pub mode: String,
    /// Required when `mode` is `ranges`.
    #[serde(default)]
    pub pages: Option<String>,
    pub output_dir: String,
    pub base_name: String,
}

#[derive(Debug, Serialize)]
pub struct SplitResult {
    pub outputs: Vec<String>,
    pub page_count: u32,
}

/// Split a PDF into several files inside an output directory.
#[tauri::command]
pub fn split_pdf(request: SplitRequest) -> Result<SplitResult, String> {
    let mode = pdf::SplitMode::parse(&request.mode).map_err(|error| error.to_string())?;
    let outputs = pdf::split(
        Path::new(&request.input),
        mode,
        request.pages.as_deref(),
        Path::new(&request.output_dir),
        &request.base_name,
    )
    .map_err(|error| error.to_string())?;

    let page_count = pdf_core::page_count(Path::new(&request.input))
        .map_err(|error| error.to_string())?;

    Ok(SplitResult {
        outputs: outputs
            .into_iter()
            .map(|path| path.to_string_lossy().into_owned())
            .collect(),
        page_count,
    })
}

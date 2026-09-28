//! Tauri command surface.
//!
//! Commands stay thin: validate arguments, call a service in `pdf`, and return
//! serializable results. All heavy lifting happens off the UI thread in the
//! job system (added in a later phase).

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use base64::Engine as _;
use pdf_render::ImageFormat;
use serde::{Deserialize, Serialize};
use tauri::State;

/// In-memory cache of rendered thumbnails, keyed by `path|page|width`.
#[derive(Default)]
pub struct ThumbnailCache(pub Mutex<HashMap<String, ThumbnailResult>>);

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

    let page_count =
        pdf_core::page_count(Path::new(&request.input)).map_err(|error| error.to_string())?;

    Ok(SplitResult {
        outputs: outputs
            .into_iter()
            .map(|path| path.to_string_lossy().into_owned())
            .collect(),
        page_count,
    })
}

#[derive(Debug, Deserialize)]
pub struct PdfInfoRequest {
    pub input: String,
}

#[derive(Debug, Serialize)]
pub struct PdfInfoResult {
    pub page_count: u32,
}

/// Number of pages in a PDF, used to lay out the page grids.
#[tauri::command]
pub fn pdf_info(request: PdfInfoRequest) -> Result<PdfInfoResult, String> {
    let page_count =
        pdf_core::page_count(Path::new(&request.input)).map_err(|error| error.to_string())?;
    Ok(PdfInfoResult { page_count })
}

#[derive(Debug, Clone, Serialize)]
pub struct ThumbnailResult {
    pub data_url: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Deserialize)]
pub struct ThumbnailRequest {
    pub input: String,
    pub page: u32,
    pub max_width: u32,
}

/// Render a page thumbnail as a `data:` URL, cached in memory.
#[tauri::command]
pub fn render_thumbnail(
    request: ThumbnailRequest,
    cache: State<'_, ThumbnailCache>,
) -> Result<ThumbnailResult, String> {
    let key = format!("{}|{}|{}", request.input, request.page, request.max_width);
    if let Ok(store) = cache.0.lock() {
        if let Some(hit) = store.get(&key) {
            return Ok(hit.clone());
        }
    }

    let image = pdf::thumbnail(Path::new(&request.input), request.page, request.max_width)
        .map_err(|error| error.to_string())?;
    let encoded = base64::engine::general_purpose::STANDARD.encode(&image.bytes);
    let result = ThumbnailResult {
        data_url: format!("data:image/png;base64,{encoded}"),
        width: image.width,
        height: image.height,
    };

    if let Ok(mut store) = cache.0.lock() {
        store.insert(key, result.clone());
    }
    Ok(result)
}

#[derive(Debug, Deserialize)]
pub struct ReorderRequest {
    pub input: String,
    /// The new 1-based page order.
    pub order: Vec<u32>,
    pub output: String,
}

/// Reorder pages and write a new PDF.
#[tauri::command]
pub fn reorder_pages(request: ReorderRequest) -> Result<OperationResult, String> {
    let page_count = pdf::reorder(
        Path::new(&request.input),
        &request.order,
        Path::new(&request.output),
    )
    .map_err(|error| error.to_string())?;

    Ok(OperationResult {
        output: request.output,
        page_count,
    })
}

#[derive(Debug, Deserialize)]
pub struct PdfToImagesRequest {
    pub input: String,
    /// Empty or omitted means "all pages".
    #[serde(default)]
    pub pages: Option<String>,
    pub dpi: f32,
    /// Either `png` or `jpg`.
    pub format: String,
    pub quality: u8,
    pub output_dir: String,
    pub base_name: String,
}

#[derive(Debug, Serialize)]
pub struct ImagesResult {
    pub outputs: Vec<String>,
    pub page_count: u32,
}

/// Render PDF pages to PNG/JPG files.
#[tauri::command]
pub fn pdf_to_images(request: PdfToImagesRequest) -> Result<ImagesResult, String> {
    let format = ImageFormat::parse(&request.format).map_err(|error| error.to_string())?;
    let outputs = pdf::to_images(
        Path::new(&request.input),
        request.pages.as_deref(),
        request.dpi,
        format,
        request.quality,
        Path::new(&request.output_dir),
        &request.base_name,
    )
    .map_err(|error| error.to_string())?;

    Ok(ImagesResult {
        outputs: outputs
            .into_iter()
            .map(|path| path.to_string_lossy().into_owned())
            .collect(),
        page_count: pdf_core::page_count(Path::new(&request.input))
            .map_err(|error| error.to_string())?,
    })
}

#[derive(Debug, Deserialize)]
pub struct ImagesToPdfRequest {
    pub images: Vec<String>,
    pub output: String,
    pub page_size: String,
    pub orientation: String,
    pub fit: String,
    pub margin: f32,
}

/// Convert images into a single PDF, one page per image.
#[tauri::command]
pub fn images_to_pdf(request: ImagesToPdfRequest) -> Result<OperationResult, String> {
    let images: Vec<PathBuf> = request.images.iter().map(PathBuf::from).collect();
    let page_count = pdf::images_to_pdf(
        &images,
        &request.page_size,
        &request.orientation,
        &request.fit,
        request.margin,
        Path::new(&request.output),
    )
    .map_err(|error| error.to_string())?;

    Ok(OperationResult {
        output: request.output,
        page_count,
    })
}

#[derive(Debug, Deserialize)]
pub struct WatermarkRequest {
    pub input: String,
    #[serde(default)]
    pub pages: Option<String>,
    /// `text` or `image`.
    pub kind: String,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub image: Option<String>,
    pub font_size: f32,
    pub opacity: f32,
    pub rotation: f32,
    pub color: f32,
    pub scale: f32,
    pub position: String,
    pub output: String,
}

/// Add a text or image watermark.
#[tauri::command]
pub fn watermark_pdf(request: WatermarkRequest) -> Result<OperationResult, String> {
    let input = Path::new(&request.input);
    let output = Path::new(&request.output);
    let pages = request.pages.as_deref();

    let result = match request.kind.as_str() {
        "text" => pdf::watermark_text(
            input,
            pages,
            request.text.as_deref().unwrap_or(""),
            request.font_size,
            request.opacity,
            request.rotation,
            request.color,
            &request.position,
            output,
        ),
        "image" => {
            let image = request
                .image
                .as_deref()
                .ok_or_else(|| "choose a watermark image".to_string())?;
            pdf::watermark_image(
                input,
                Path::new(image),
                pages,
                request.scale,
                request.opacity,
                request.rotation,
                &request.position,
                output,
            )
        }
        other => return Err(format!("unknown watermark kind \"{other}\"")),
    };

    Ok(OperationResult {
        output: request.output,
        page_count: result.map_err(|error| error.to_string())?,
    })
}

#[derive(Debug, Deserialize)]
pub struct PageNumbersRequest {
    pub input: String,
    #[serde(default)]
    pub pages: Option<String>,
    pub vertical: String,
    pub horizontal: String,
    pub start: i64,
    pub font_size: f32,
    pub margin: f32,
    pub color: f32,
    pub output: String,
}

/// Add page numbers.
#[tauri::command]
pub fn add_page_numbers(request: PageNumbersRequest) -> Result<OperationResult, String> {
    let page_count = pdf::add_page_numbers(
        Path::new(&request.input),
        request.pages.as_deref(),
        &request.vertical,
        &request.horizontal,
        request.start,
        request.font_size,
        request.margin,
        request.color,
        Path::new(&request.output),
    )
    .map_err(|error| error.to_string())?;

    Ok(OperationResult {
        output: request.output,
        page_count,
    })
}

#[derive(Debug, Deserialize)]
pub struct ProtectRequest {
    pub input: String,
    pub password: String,
    pub allow_printing: bool,
    pub allow_copying: bool,
    pub allow_modifying: bool,
    pub output: String,
}

/// Encrypt a PDF with a password.
#[tauri::command]
pub fn protect_pdf(request: ProtectRequest) -> Result<OperationResult, String> {
    let page_count = pdf::protect(
        Path::new(&request.input),
        &request.password,
        request.allow_printing,
        request.allow_copying,
        request.allow_modifying,
        Path::new(&request.output),
    )
    .map_err(|error| error.to_string())?;

    Ok(OperationResult {
        output: request.output,
        page_count,
    })
}

#[derive(Debug, Deserialize)]
pub struct UnlockRequest {
    pub input: String,
    pub password: String,
    pub output: String,
}

/// Remove protection using a known password.
#[tauri::command]
pub fn unlock_pdf(request: UnlockRequest) -> Result<OperationResult, String> {
    let page_count = pdf::unlock(
        Path::new(&request.input),
        &request.password,
        Path::new(&request.output),
    )
    .map_err(|error| error.to_string())?;

    Ok(OperationResult {
        output: request.output,
        page_count,
    })
}

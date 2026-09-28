//! Application-level PDF services.
//!
//! Orchestrates the engine crates (`pdf-core` for manipulation, `pdf-render`
//! for rasterization) with safe output naming. This crate has no Tauri
//! dependency, so it is fully unit-testable on any platform; the Tauri command
//! layer only validates arguments and forwards calls here.

pub mod filesystem;

use std::path::{Path, PathBuf};

use pdf_core::parse_page_selection;
use pdf_core::{
    FitMode, Horizontal, ImageWatermark, ImagesToPdfOptions, Orientation, PageNumberOptions,
    PageSize, Position, ProtectOptions, TextWatermark, Vertical,
};
use pdf_render::{ImageFormat, RenderedImage};

pub use pdf_core::{PdfCoreError, PdfDocument};

/// Merge several PDFs into a new file, preserving input order.
///
/// The originals are never modified. Returns the page count of the result.
pub fn merge(inputs: &[PathBuf], output: &Path) -> Result<u32, PdfCoreError> {
    pdf_core::merge(inputs, output)
}

/// Extract the pages named by `selection` (e.g. `1-4,8,11-15`) into `output`.
pub fn extract(input: &Path, selection: &str, output: &Path) -> Result<u32, PdfCoreError> {
    let total = pdf_core::page_count(input)?;
    let pages = parse_page_selection(selection, total)?;
    pdf_core::extract_pages(input, &pages, output)
}

/// Remove the pages named by `selection` and write the rest to `output`.
pub fn remove(input: &Path, selection: &str, output: &Path) -> Result<u32, PdfCoreError> {
    let total = pdf_core::page_count(input)?;
    let pages = parse_page_selection(selection, total)?;
    pdf_core::remove_pages(input, &pages, output)
}

/// Rotate selected pages (`None` means all) clockwise by `rotation` degrees.
pub fn rotate(
    input: &Path,
    selection: Option<&str>,
    rotation: i64,
    output: &Path,
) -> Result<u32, PdfCoreError> {
    let total = pdf_core::page_count(input)?;
    let pages = match selection {
        Some(value) if !value.trim().is_empty() => Some(parse_page_selection(value, total)?),
        _ => None,
    };
    pdf_core::rotate_pages(input, pages.as_deref(), rotation, output)
}

/// Reorder pages by writing them in the given 1-based order.
pub fn reorder(input: &Path, order: &[u32], output: &Path) -> Result<u32, PdfCoreError> {
    if order.is_empty() {
        return Err(PdfCoreError::Other(
            "the new page order is empty".to_string(),
        ));
    }
    pdf_core::extract_pages(input, order, output)
}

/// Convert a list of images (in order) into a single PDF.
pub fn images_to_pdf(
    images: &[PathBuf],
    page_size: &str,
    orientation: &str,
    fit: &str,
    margin: f32,
    output: &Path,
) -> Result<u32, PdfCoreError> {
    let options = ImagesToPdfOptions {
        page_size: PageSize::parse(page_size)?,
        orientation: Orientation::parse(orientation)?,
        fit: FitMode::parse(fit)?,
        margin,
    };
    pdf_core::images_to_pdf(images, options, output)
}

/// Parse an optional page selection against a document's page count.
fn parse_optional_pages(
    selection: Option<&str>,
    total: u32,
) -> Result<Option<Vec<u32>>, PdfCoreError> {
    match selection {
        Some(value) if !value.trim().is_empty() => Ok(Some(parse_page_selection(value, total)?)),
        _ => Ok(None),
    }
}

/// Stamp a text watermark (Arabic-aware) onto selected pages.
#[allow(clippy::too_many_arguments)]
pub fn watermark_text(
    input: &Path,
    selection: Option<&str>,
    text: &str,
    font_size: f32,
    opacity: f32,
    rotation: f32,
    color: f32,
    position: &str,
    output: &Path,
) -> Result<u32, PdfCoreError> {
    let total = pdf_core::page_count(input)?;
    let pages = parse_optional_pages(selection, total)?;
    let watermark = TextWatermark {
        text: text.to_string(),
        font_size,
        opacity,
        rotation,
        color,
        position: Position::parse(position)?,
    };
    pdf_core::watermark_text(input, pages.as_deref(), &watermark, output)
}

/// Stamp an image watermark onto selected pages.
#[allow(clippy::too_many_arguments)]
pub fn watermark_image(
    input: &Path,
    image_path: &Path,
    selection: Option<&str>,
    scale: f32,
    opacity: f32,
    rotation: f32,
    position: &str,
    output: &Path,
) -> Result<u32, PdfCoreError> {
    let total = pdf_core::page_count(input)?;
    let pages = parse_optional_pages(selection, total)?;
    let watermark = ImageWatermark {
        scale,
        opacity,
        rotation,
        position: Position::parse(position)?,
    };
    pdf_core::watermark_image(input, image_path, pages.as_deref(), &watermark, output)
}

/// Add page numbers to selected pages.
#[allow(clippy::too_many_arguments)]
pub fn add_page_numbers(
    input: &Path,
    selection: Option<&str>,
    vertical: &str,
    horizontal: &str,
    start: i64,
    font_size: f32,
    margin: f32,
    color: f32,
    output: &Path,
) -> Result<u32, PdfCoreError> {
    let total = pdf_core::page_count(input)?;
    let pages = parse_optional_pages(selection, total)?;
    let options = PageNumberOptions {
        vertical: Vertical::parse(vertical)?,
        horizontal: Horizontal::parse(horizontal)?,
        start,
        font_size,
        margin,
        color,
    };
    pdf_core::add_page_numbers(input, pages.as_deref(), &options, output)
}

/// Encrypt a PDF with a password (AES-128).
pub fn protect(
    input: &Path,
    user_password: &str,
    allow_printing: bool,
    allow_copying: bool,
    allow_modifying: bool,
    output: &Path,
) -> Result<u32, PdfCoreError> {
    let options = ProtectOptions {
        user_password: user_password.to_string(),
        owner_password: None,
        allow_printing,
        allow_copying,
        allow_modifying,
    };
    pdf_core::protect(input, &options, output)
}

/// Remove protection using the correct password.
pub fn unlock(input: &Path, password: &str, output: &Path) -> Result<u32, PdfCoreError> {
    pdf_core::unlock(input, password, output)
}

/// How Split decides which pages belong together.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitMode {
    /// One output file per page.
    EveryPage,
    /// One output file per contiguous run of the selection.
    Ranges,
}

impl SplitMode {
    pub fn parse(value: &str) -> Result<Self, PdfCoreError> {
        match value {
            "every-page" => Ok(Self::EveryPage),
            "ranges" => Ok(Self::Ranges),
            other => Err(PdfCoreError::Other(format!(
                "unknown split mode \"{other}\""
            ))),
        }
    }
}

/// Split `input` into several files inside `output_dir`.
///
/// File names are `{base_name}_{index:03}.pdf`, made unique against existing
/// files. Returns the generated paths (and never overwrites anything).
pub fn split(
    input: &Path,
    mode: SplitMode,
    selection: Option<&str>,
    output_dir: &Path,
    base_name: &str,
) -> Result<Vec<PathBuf>, PdfCoreError> {
    let total = pdf_core::page_count(input)?;

    let groups: Vec<Vec<u32>> = match mode {
        SplitMode::EveryPage => (1..=total).map(|page| vec![page]).collect(),
        SplitMode::Ranges => {
            let value = selection.unwrap_or("").trim();
            if value.is_empty() {
                return Err(PdfCoreError::Other(
                    "enter at least one page range to split".to_string(),
                ));
            }
            let pages = parse_page_selection(value, total)?;
            group_contiguous(&pages)
        }
    };

    let stem = filesystem::unique_run_stem(output_dir, base_name, "pdf");
    let outputs: Vec<PathBuf> = (0..groups.len())
        .map(|index| output_dir.join(format!("{stem}_{:03}.pdf", index + 1)))
        .collect();

    pdf_core::split_pdf(input, &groups, &outputs)?;
    Ok(outputs)
}

/// Render one page as a PNG thumbnail (width capped at `max_width`).
pub fn thumbnail(input: &Path, page: u32, max_width: u32) -> Result<RenderedImage, PdfCoreError> {
    pdf_render::render_thumbnail(input, page, max_width)
}

/// Render selected pages to image files inside `output_dir`.
pub fn to_images(
    input: &Path,
    selection: Option<&str>,
    dpi: f32,
    format: ImageFormat,
    quality: u8,
    output_dir: &Path,
    base_name: &str,
) -> Result<Vec<PathBuf>, PdfCoreError> {
    let total = pdf_core::page_count(input)?;
    let pages: Vec<u32> = match selection {
        Some(value) if !value.trim().is_empty() => parse_page_selection(value, total)?,
        _ => (1..=total).collect(),
    };

    let stem = filesystem::unique_run_stem(output_dir, base_name, format.extension());
    let scale = pdf_render::scale_for_dpi(dpi);

    let mut outputs: Vec<PathBuf> = Vec::with_capacity(pages.len());
    pdf_render::render_pages(
        input,
        &pages,
        scale,
        format,
        quality,
        |page_number, image| {
            let path = output_dir.join(format!("{stem}_{page_number:03}.{}", format.extension()));
            std::fs::write(&path, &image.bytes).map_err(|source| PdfCoreError::Io {
                path: path.clone(),
                source,
            })?;
            outputs.push(path);
            Ok(())
        },
    )?;

    Ok(outputs)
}

/// Collapse an ordered page list into contiguous runs: `[1,2,3,8,11,12]` →
/// `[[1,2,3],[8],[11,12]]`.
fn group_contiguous(pages: &[u32]) -> Vec<Vec<u32>> {
    let mut groups: Vec<Vec<u32>> = Vec::new();
    let mut current: Vec<u32> = Vec::new();

    for &page in pages {
        match current.last() {
            Some(&last) if page == last + 1 => current.push(page),
            Some(_) => {
                groups.push(std::mem::take(&mut current));
                current.push(page);
            }
            None => current.push(page),
        }
    }
    if !current.is_empty() {
        groups.push(current);
    }
    groups
}

#[cfg(test)]
mod tests {
    use super::group_contiguous;

    #[test]
    fn groups_contiguous_runs() {
        assert_eq!(
            group_contiguous(&[1, 2, 3, 8, 11, 12]),
            vec![vec![1, 2, 3], vec![8], vec![11, 12]]
        );
        assert_eq!(group_contiguous(&[]), Vec::<Vec<u32>>::new());
        assert_eq!(group_contiguous(&[5]), vec![vec![5]]);
    }
}

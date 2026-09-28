//! PDF rasterization adapter for PDF Toolkit.
//!
//! Wraps [`hayro`] (pure Rust, Apache-2.0/MIT) behind a small, stable API so
//! the renderer can be replaced without touching the rest of the app. Used for
//! page thumbnails and PDF-to-images.
//!
//! Rendering is CPU-only and never touches the UI thread: callers run it inside
//! the Tauri job system.

use std::path::Path;

use hayro::hayro_interpret::InterpreterSettings;
use hayro::hayro_syntax::page::Page;
use hayro::hayro_syntax::Pdf;
use hayro::vello_cpu::color::palette::css::WHITE;
use hayro::vello_cpu::Pixmap;
use hayro::{render, RenderCache, RenderSettings};
use image::codecs::jpeg::JpegEncoder;
use image::{DynamicImage, RgbaImage};

use pdf_core::{PdfCoreError, PdfCoreResult};

/// Upper bound on either rendered dimension, to keep memory usage sane on
/// hostile or huge documents.
pub const MAX_DIMENSION: u32 = 10_000;

/// Output image format for PDF-to-images.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Png,
    Jpeg,
}

impl ImageFormat {
    pub fn parse(value: &str) -> PdfCoreResult<Self> {
        match value.to_ascii_lowercase().as_str() {
            "png" => Ok(Self::Png),
            "jpg" | "jpeg" => Ok(Self::Jpeg),
            other => Err(PdfCoreError::Other(format!(
                "unsupported image format \"{other}\""
            ))),
        }
    }

    pub fn extension(self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Jpeg => "jpg",
        }
    }
}

/// A rendered page: encoded image bytes plus the pixel dimensions.
pub struct RenderedImage {
    pub bytes: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

/// Convert a DPI value into a renderer scale factor (PDF points are 72/inch).
pub fn scale_for_dpi(dpi: f32) -> f32 {
    (dpi / 72.0).clamp(0.1, 16.0)
}

/// Render a single page at an explicit scale.
pub fn render_page(
    input: &Path,
    page_number: u32,
    scale: f32,
    format: ImageFormat,
    quality: u8,
) -> PdfCoreResult<RenderedImage> {
    let data = read(input)?;
    let pdf = open(data)?;
    let pages = pdf.pages();
    let page = page_ref(pages, page_number)?;
    render_page_ref(page, scale, format, quality)
}

/// Render a page as a PNG whose width is at most `max_width` pixels.
pub fn render_thumbnail(
    input: &Path,
    page_number: u32,
    max_width: u32,
) -> PdfCoreResult<RenderedImage> {
    let data = read(input)?;
    let pdf = open(data)?;
    let pages = pdf.pages();
    let page = page_ref(pages, page_number)?;

    let width = page_width(page);
    let scale = if width > 0.0 {
        max_width as f32 / width
    } else {
        1.0
    };
    render_page_ref(page, scale, ImageFormat::Png, 100)
}

/// Render several pages, loading the document only once.
///
/// `on_page` is called for each rendered page so the caller can write the file
/// and report progress without buffering every page in memory.
pub fn render_pages<F>(
    input: &Path,
    pages_to_render: &[u32],
    scale: f32,
    format: ImageFormat,
    quality: u8,
    mut on_page: F,
) -> PdfCoreResult<()>
where
    F: FnMut(u32, &RenderedImage) -> PdfCoreResult<()>,
{
    let data = read(input)?;
    let pdf = open(data)?;
    let pages = pdf.pages();

    for &page_number in pages_to_render {
        let page = page_ref(pages, page_number)?;
        let rendered = render_page_ref(page, scale, format, quality)?;
        on_page(page_number, &rendered)?;
    }
    Ok(())
}

fn render_page_ref(
    page: &Page<'_>,
    scale: f32,
    format: ImageFormat,
    quality: u8,
) -> PdfCoreResult<RenderedImage> {
    let (width, height) = page.render_dimensions();
    let scale = safe_scale(scale, width, height);

    let cache = RenderCache::new();
    let interpreter = InterpreterSettings::default();
    let settings = RenderSettings {
        x_scale: scale,
        y_scale: scale,
        bg_color: WHITE,
        ..Default::default()
    };

    let pixmap = render(page, &cache, &interpreter, &settings);
    encode(pixmap, format, quality)
}

fn encode(pixmap: Pixmap, format: ImageFormat, quality: u8) -> PdfCoreResult<RenderedImage> {
    let width = pixmap.width() as u32;
    let height = pixmap.height() as u32;

    let bytes = match format {
        ImageFormat::Png => pixmap
            .into_png()
            .map_err(|error| PdfCoreError::Other(format!("failed to encode PNG: {error:?}")))?,
        ImageFormat::Jpeg => {
            // The background is opaque white, so the pixmap's alpha is 255 and
            // premultiplied RGB equals straight RGB.
            let raw = pixmap.data_as_u8_slice().to_vec();
            let rgba = RgbaImage::from_raw(width, height, raw).ok_or_else(|| {
                PdfCoreError::Other("rendered image has an invalid size".to_string())
            })?;
            let rgb = DynamicImage::ImageRgba8(rgba).to_rgb8();

            let mut out = Vec::new();
            let mut encoder = JpegEncoder::new_with_quality(&mut out, quality.clamp(1, 100));
            encoder
                .encode_image(&rgb)
                .map_err(|error| PdfCoreError::Other(format!("failed to encode JPEG: {error}")))?;
            out
        }
    };

    Ok(RenderedImage {
        bytes,
        width,
        height,
    })
}

fn safe_scale(requested: f32, width: f32, height: f32) -> f32 {
    if width <= 0.0 || height <= 0.0 {
        return requested.clamp(0.01, 16.0);
    }
    let limit = (MAX_DIMENSION as f32 / width).min(MAX_DIMENSION as f32 / height);
    requested.min(limit).clamp(0.01, 16.0)
}

fn page_width(page: &Page<'_>) -> f32 {
    page.render_dimensions().0
}

fn page_ref<'a, 'b>(pages: &'a [Page<'b>], page_number: u32) -> PdfCoreResult<&'a Page<'b>> {
    let total = pages.len() as u32;
    if page_number < 1 || page_number > total {
        return Err(PdfCoreError::PageOutOfRange {
            page: page_number,
            total,
        });
    }
    pages.get((page_number - 1) as usize).ok_or({
        PdfCoreError::PageOutOfRange {
            page: page_number,
            total,
        }
    })
}

fn read(path: &Path) -> PdfCoreResult<Vec<u8>> {
    std::fs::read(path).map_err(|source| PdfCoreError::Io {
        path: path.to_path_buf(),
        source,
    })
}

fn open(data: Vec<u8>) -> PdfCoreResult<Pdf> {
    Pdf::new(data).map_err(|error| PdfCoreError::Other(format!("could not read PDF: {error:?}")))
}

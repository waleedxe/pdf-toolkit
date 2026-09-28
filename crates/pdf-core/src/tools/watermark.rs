//! Watermark tool: stamps text or an image onto selected pages.
//!
//! Text watermarks are shaped and embedded with the bundled Amiri font, so
//! Arabic renders correctly (connected letters, right-to-left). Images are
//! embedded as compressed XObjects.

use std::path::Path;

use lopdf::{dictionary, Document, Object, ObjectId, Stream};

use crate::error::{PdfCoreError, PdfCoreResult};
use crate::text;
use crate::tools::content;

/// Where the watermark is anchored on the page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    Center,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Position {
    pub fn parse(value: &str) -> PdfCoreResult<Self> {
        match value.to_ascii_lowercase().replace('_', "-").as_str() {
            "center" => Ok(Self::Center),
            "top-left" => Ok(Self::TopLeft),
            "top-right" => Ok(Self::TopRight),
            "bottom-left" => Ok(Self::BottomLeft),
            "bottom-right" => Ok(Self::BottomRight),
            other => Err(PdfCoreError::Other(format!(
                "unknown watermark position \"{other}\""
            ))),
        }
    }

    fn anchor(self, width: f32, height: f32) -> (f32, f32) {
        match self {
            Self::Center => (width * 0.5, height * 0.5),
            Self::TopLeft => (width * 0.22, height * 0.82),
            Self::TopRight => (width * 0.78, height * 0.82),
            Self::BottomLeft => (width * 0.22, height * 0.18),
            Self::BottomRight => (width * 0.78, height * 0.18),
        }
    }
}

/// Options for a text watermark.
pub struct TextWatermark {
    pub text: String,
    pub font_size: f32,
    pub opacity: f32,
    /// Rotation in degrees, counter-clockwise.
    pub rotation: f32,
    /// Grayscale fill value, 0 (black) to 1 (white).
    pub color: f32,
    pub position: Position,
}

/// Options for an image watermark.
pub struct ImageWatermark {
    /// Fraction of the page's shorter side used by the image, 0–1.
    pub scale: f32,
    pub opacity: f32,
    /// Rotation in degrees, counter-clockwise.
    pub rotation: f32,
    pub position: Position,
}

/// Stamp shaped text onto the selected pages (`None` means every page).
pub fn watermark_text(
    input: &Path,
    pages: Option<&[u32]>,
    watermark: &TextWatermark,
    output: &Path,
) -> PdfCoreResult<u32> {
    if watermark.text.trim().is_empty() {
        return Err(PdfCoreError::Other(
            "enter some text for the watermark".to_string(),
        ));
    }

    let mut document = content::load(input)?;
    let total = document.get_pages().len() as u32;
    let selected = content::selected_pages(pages, total)?;

    let shaped = text::shape_text(text::EMBEDDED_FONT, &watermark.text, watermark.font_size)?;
    let font_id = text::embed_font(&mut document, text::EMBEDDED_FONT, text::EMBEDDED_FONT_NAME)?;
    let opacity = watermark.opacity.clamp(0.0, 1.0);
    let gs_id = document.add_object(dictionary! {
        "Type" => "ExtGState",
        "ca" => Object::Real(opacity),
        "CA" => Object::Real(opacity),
    });

    let page_ids = page_ids(&document, &selected);
    let radians = watermark.rotation.to_radians();
    let (sin, cos) = radians.sin_cos();
    let neg_sin = -sin;
    let font_size = watermark.font_size;
    let color = watermark.color.clamp(0.0, 1.0);
    let text_offset_x = -shaped.advance / 2.0;
    let text_offset_y = -font_size * 0.35;
    let glyphs = shaped.hex();

    for page_id in page_ids {
        let (page_width, page_height) = content::page_size(&document, page_id)?;
        let (anchor_x, anchor_y) = watermark.position.anchor(page_width, page_height);
        let operations = format!(
            "q\n/GSwm gs\n{color:.3} g\n1 0 0 1 {anchor_x:.4} {anchor_y:.4} cm\n{cos:.5} {sin:.5} {neg_sin:.5} {cos:.5} 0 0 cm\nBT\n/Fwm {font_size:.4} Tf\n{text_offset_x:.4} {text_offset_y:.4} Td\n{glyphs} Tj\nET\nQ\n"
        );

        content::append_content(
            &mut document,
            page_id,
            Stream::new(dictionary! {}, operations.into_bytes()),
        )?;
        content::set_resource(&mut document, page_id, b"Font", "Fwm", font_id)?;
        content::set_resource(&mut document, page_id, b"ExtGState", "GSwm", gs_id)?;
    }

    document
        .save(output)
        .map_err(|error| PdfCoreError::Other(format!("failed to write PDF: {error}")))?;
    Ok(total)
}

/// Stamp an image onto the selected pages (`None` means every page).
pub fn watermark_image(
    input: &Path,
    image_path: &Path,
    pages: Option<&[u32]>,
    watermark: &ImageWatermark,
    output: &Path,
) -> PdfCoreResult<u32> {
    let decoded = image::open(image_path).map_err(|error| {
        PdfCoreError::Other(format!("could not read the watermark image: {error}"))
    })?;
    let rgb = decoded.to_rgb8();
    let (image_width, image_height) = (rgb.width() as f32, rgb.height() as f32);
    if image_width < 1.0 || image_height < 1.0 {
        return Err(PdfCoreError::Other(
            "the watermark image has no usable size".to_string(),
        ));
    }
    let compressed = content::deflate(&rgb.into_raw())?;

    let mut document = content::load(input)?;
    let total = document.get_pages().len() as u32;
    let selected = content::selected_pages(pages, total)?;

    let image_id = document.add_object(Stream::new(
        dictionary! {
            "Type" => "XObject",
            "Subtype" => "Image",
            "Width" => image_width as i64,
            "Height" => image_height as i64,
            "ColorSpace" => "DeviceRGB",
            "BitsPerComponent" => 8,
            "Filter" => "FlateDecode",
        },
        compressed,
    ));

    let opacity = watermark.opacity.clamp(0.0, 1.0);
    let gs_id = document.add_object(dictionary! {
        "Type" => "ExtGState",
        "ca" => Object::Real(opacity),
        "CA" => Object::Real(opacity),
    });

    let page_ids = page_ids(&document, &selected);
    let radians = watermark.rotation.to_radians();
    let (sin, cos) = radians.sin_cos();
    let neg_sin = -sin;
    let scale = watermark.scale.clamp(0.05, 1.0);
    let aspect = image_width / image_height;

    for page_id in page_ids {
        let (page_width, page_height) = content::page_size(&document, page_id)?;
        let base = page_width.min(page_height) * scale;
        let (draw_w, draw_h) = if aspect >= 1.0 {
            (base, base / aspect)
        } else {
            (base * aspect, base)
        };
        let (anchor_x, anchor_y) = watermark.position.anchor(page_width, page_height);
        let start_x = -draw_w / 2.0;
        let start_y = -draw_h / 2.0;
        let operations = format!(
            "q\n/GSwm gs\n1 0 0 1 {anchor_x:.4} {anchor_y:.4} cm\n{cos:.5} {sin:.5} {neg_sin:.5} {cos:.5} 0 0 cm\n{draw_w:.4} 0 0 {draw_h:.4} {start_x:.4} {start_y:.4} cm\n/Imwm Do\nQ\n"
        );

        content::append_content(
            &mut document,
            page_id,
            Stream::new(dictionary! {}, operations.into_bytes()),
        )?;
        content::set_resource(&mut document, page_id, b"XObject", "Imwm", image_id)?;
        content::set_resource(&mut document, page_id, b"ExtGState", "GSwm", gs_id)?;
    }

    document
        .save(output)
        .map_err(|error| PdfCoreError::Other(format!("failed to write PDF: {error}")))?;
    Ok(total)
}

fn page_ids(document: &Document, pages: &[u32]) -> Vec<ObjectId> {
    let map = document.get_pages();
    pages
        .iter()
        .filter_map(|page| map.get(page).copied())
        .collect()
}

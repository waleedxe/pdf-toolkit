//! Page Numbers tool: stamps a running number on selected pages.

use std::path::Path;

use lopdf::{dictionary, Document, ObjectId, Stream};

use crate::error::{PdfCoreError, PdfCoreResult};
use crate::text;
use crate::tools::content;

/// Vertical placement of the number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vertical {
    Top,
    Bottom,
}

impl Vertical {
    pub fn parse(value: &str) -> PdfCoreResult<Self> {
        match value.to_ascii_lowercase().as_str() {
            "top" => Ok(Self::Top),
            "bottom" => Ok(Self::Bottom),
            other => Err(PdfCoreError::Other(format!(
                "unknown page-number position \"{other}\""
            ))),
        }
    }
}

/// Horizontal alignment of the number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Horizontal {
    Left,
    Center,
    Right,
}

impl Horizontal {
    pub fn parse(value: &str) -> PdfCoreResult<Self> {
        match value.to_ascii_lowercase().as_str() {
            "left" => Ok(Self::Left),
            "center" => Ok(Self::Center),
            "right" => Ok(Self::Right),
            other => Err(PdfCoreError::Other(format!(
                "unknown page-number alignment \"{other}\""
            ))),
        }
    }
}

/// Options for [`add_page_numbers`].
pub struct PageNumberOptions {
    pub vertical: Vertical,
    pub horizontal: Horizontal,
    /// Number printed on the first selected page.
    pub start: i64,
    pub font_size: f32,
    /// Distance from the page edge, in points.
    pub margin: f32,
    /// Grayscale fill value, 0 (black) to 1 (white).
    pub color: f32,
}

/// Add page numbers to the selected pages (`None` means every page).
pub fn add_page_numbers(
    input: &Path,
    pages: Option<&[u32]>,
    options: &PageNumberOptions,
    output: &Path,
) -> PdfCoreResult<u32> {
    let mut document = content::load(input)?;
    let total = document.get_pages().len() as u32;
    let selected = content::selected_pages(pages, total)?;

    let font_id = text::embed_font(&mut document, text::EMBEDDED_FONT, text::EMBEDDED_FONT_NAME)?;
    let page_ids = page_ids(&document, &selected);

    for (index, page_id) in page_ids.iter().enumerate() {
        let label = (options.start + index as i64).to_string();
        let shaped = text::shape_text(text::EMBEDDED_FONT, &label, options.font_size)?;
        let (page_width, page_height) = content::page_size(&document, *page_id)?;

        let x = match options.horizontal {
            Horizontal::Left => options.margin,
            Horizontal::Center => (page_width - shaped.advance) / 2.0,
            Horizontal::Right => page_width - options.margin - shaped.advance,
        };
        let y = match options.vertical {
            Vertical::Top => page_height - options.margin - options.font_size,
            Vertical::Bottom => options.margin,
        };

        let color = options.color.clamp(0.0, 1.0);
        let font_size = options.font_size;
        let glyphs = shaped.hex();
        let operations = format!(
            "q\n{color:.3} g\nBT\n/Fwm {font_size:.4} Tf\n{x:.4} {y:.4} Td\n{glyphs} Tj\nET\nQ\n"
        );

        content::append_content(
            &mut document,
            *page_id,
            Stream::new(dictionary! {}, operations.into_bytes()),
        )?;
        content::set_resource(&mut document, *page_id, b"Font", "Fwm", font_id)?;
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

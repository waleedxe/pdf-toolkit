use std::path::{Path, PathBuf};

use lopdf::{dictionary, Document, Object, Stream};

use crate::error::{PdfCoreError, PdfCoreResult};
use crate::tools::content;

/// Target page size for Images to PDF.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageSize {
    A4,
    Letter,
    /// Page matches the image's own pixel dimensions (1 px = 1 pt).
    Fit,
}

impl PageSize {
    pub fn parse(value: &str) -> PdfCoreResult<Self> {
        match value.to_ascii_lowercase().as_str() {
            "a4" => Ok(Self::A4),
            "letter" => Ok(Self::Letter),
            "fit" => Ok(Self::Fit),
            other => Err(PdfCoreError::Other(format!(
                "unknown page size \"{other}\""
            ))),
        }
    }

    /// Base dimensions in portrait orientation (points).
    fn base(self) -> (f32, f32) {
        match self {
            Self::A4 => (595.276, 841.89),
            Self::Letter => (612.0, 792.0),
            Self::Fit => (0.0, 0.0),
        }
    }
}

/// Page orientation, resolved against the image when `Auto`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Auto,
    Portrait,
    Landscape,
}

impl Orientation {
    pub fn parse(value: &str) -> PdfCoreResult<Self> {
        match value.to_ascii_lowercase().as_str() {
            "auto" => Ok(Self::Auto),
            "portrait" => Ok(Self::Portrait),
            "landscape" => Ok(Self::Landscape),
            other => Err(PdfCoreError::Other(format!(
                "unknown orientation \"{other}\""
            ))),
        }
    }
}

/// How the image is fitted onto the page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FitMode {
    /// Scale to fit entirely inside the margins, preserving aspect ratio.
    Contain,
    /// Fill the margins, preserving aspect ratio (may crop).
    Cover,
    /// Fill the margins, ignoring aspect ratio.
    Stretch,
}

impl FitMode {
    pub fn parse(value: &str) -> PdfCoreResult<Self> {
        match value.to_ascii_lowercase().as_str() {
            "contain" => Ok(Self::Contain),
            "cover" => Ok(Self::Cover),
            "stretch" => Ok(Self::Stretch),
            other => Err(PdfCoreError::Other(format!("unknown fit mode \"{other}\""))),
        }
    }
}

/// Options for [`images_to_pdf`].
#[derive(Debug, Clone, Copy)]
pub struct ImagesToPdfOptions {
    pub page_size: PageSize,
    pub orientation: Orientation,
    pub fit: FitMode,
    /// Margin in points, applied on every side.
    pub margin: f32,
}

impl Default for ImagesToPdfOptions {
    fn default() -> Self {
        Self {
            page_size: PageSize::A4,
            orientation: Orientation::Auto,
            fit: FitMode::Contain,
            margin: 0.0,
        }
    }
}

/// Turn a list of images (in order) into a single PDF, one page per image.
///
/// Returns the page count. Images are embedded losslessly as Flate-compressed
/// RGB, so JPG/PNG/WebP inputs all work regardless of their original encoding.
pub fn images_to_pdf(
    images: &[PathBuf],
    options: ImagesToPdfOptions,
    output: &Path,
) -> PdfCoreResult<u32> {
    if images.is_empty() {
        return Err(PdfCoreError::Other(
            "select at least one image to convert".to_string(),
        ));
    }

    let mut document = Document::with_version("1.5");
    let pages_id = document.new_object_id();
    let mut kids: Vec<Object> = Vec::new();

    for path in images {
        let decoded = image::open(path).map_err(|error| map_image_error(path, &error))?;
        let rgb = decoded.to_rgb8();
        let (image_width, image_height) = (rgb.width() as f32, rgb.height() as f32);
        if image_width < 1.0 || image_height < 1.0 {
            return Err(PdfCoreError::Other(format!(
                "the image at {} has no usable size",
                path.display()
            )));
        }

        let (page_width, page_height) = resolve_page(options, image_width, image_height);
        let margin = options
            .margin
            .max(0.0)
            .min(page_width.min(page_height) / 2.0 - 1.0);
        let (draw_w, draw_h, x, y) = placement(
            options.fit,
            page_width,
            page_height,
            image_width,
            image_height,
            margin,
        );

        let compressed = content::deflate(&rgb.into_raw())?;
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

        let content = match options.fit {
            FitMode::Cover => format!(
                "q\n{} {} {} {} re W n\n{} 0 0 {} {} {} cm\n/Im0 Do\nQ\n",
                num(margin),
                num(margin),
                num(page_width - margin * 2.0),
                num(page_height - margin * 2.0),
                num(draw_w),
                num(draw_h),
                num(x),
                num(y),
            ),
            _ => format!(
                "q\n{} 0 0 {} {} {} cm\n/Im0 Do\nQ\n",
                num(draw_w),
                num(draw_h),
                num(x),
                num(y),
            ),
        };
        let content_id = document.add_object(Stream::new(dictionary! {}, content.into_bytes()));

        let page_id = document.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "MediaBox" => vec![
                Object::Real(0.0),
                Object::Real(0.0),
                Object::Real(page_width),
                Object::Real(page_height),
            ],
            "Contents" => content_id,
            "Resources" => dictionary! {
                "XObject" => dictionary! { "Im0" => image_id },
            },
        });
        kids.push(page_id.into());
    }

    let count = kids.len() as i64;
    document.objects.insert(
        pages_id,
        Object::Dictionary(dictionary! {
            "Type" => "Pages",
            "Kids" => kids,
            "Count" => count,
        }),
    );
    let catalog_id = document.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    document.trailer.set("Root", catalog_id);

    document
        .save(output)
        .map_err(|error| PdfCoreError::Other(format!("failed to write PDF: {error}")))?;

    Ok(count as u32)
}

fn resolve_page(options: ImagesToPdfOptions, image_width: f32, image_height: f32) -> (f32, f32) {
    if options.page_size == PageSize::Fit {
        return (image_width, image_height);
    }

    let (base_w, base_h) = options.page_size.base();
    let (long, short) = if base_w >= base_h {
        (base_w, base_h)
    } else {
        (base_h, base_w)
    };

    match options.orientation {
        Orientation::Portrait => (short, long),
        Orientation::Landscape => (long, short),
        Orientation::Auto => {
            if image_width > image_height {
                (long, short)
            } else {
                (short, long)
            }
        }
    }
}

fn placement(
    fit: FitMode,
    page_width: f32,
    page_height: f32,
    image_width: f32,
    image_height: f32,
    margin: f32,
) -> (f32, f32, f32, f32) {
    let available_w = (page_width - margin * 2.0).max(1.0);
    let available_h = (page_height - margin * 2.0).max(1.0);

    match fit {
        FitMode::Stretch => (available_w, available_h, margin, margin),
        FitMode::Contain => {
            let scale = (available_w / image_width).min(available_h / image_height);
            let draw_w = image_width * scale;
            let draw_h = image_height * scale;
            (
                draw_w,
                draw_h,
                (page_width - draw_w) / 2.0,
                (page_height - draw_h) / 2.0,
            )
        }
        FitMode::Cover => {
            let scale = (available_w / image_width).max(available_h / image_height);
            let draw_w = image_width * scale;
            let draw_h = image_height * scale;
            (
                draw_w,
                draw_h,
                (page_width - draw_w) / 2.0,
                (page_height - draw_h) / 2.0,
            )
        }
    }
}

fn num(value: f32) -> String {
    format!("{value:.4}")
}

fn map_image_error(path: &Path, error: &image::ImageError) -> PdfCoreError {
    PdfCoreError::Other(format!(
        "could not read the image at {}: {error}",
        path.display()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_orientation_swaps_landscape_images() {
        let options = ImagesToPdfOptions::default();
        assert_eq!(resolve_page(options, 1000.0, 500.0), (841.89, 595.276));
        assert_eq!(resolve_page(options, 500.0, 1000.0), (595.276, 841.89));
    }

    #[test]
    fn fit_page_size_uses_image_dimensions() {
        let options = ImagesToPdfOptions {
            page_size: PageSize::Fit,
            ..Default::default()
        };
        assert_eq!(resolve_page(options, 321.0, 123.0), (321.0, 123.0));
    }

    #[test]
    fn contain_preserves_aspect_ratio() {
        let (w, h, x, y) = placement(FitMode::Contain, 100.0, 100.0, 200.0, 100.0, 0.0);
        assert_eq!((w, h), (100.0, 50.0));
        assert_eq!((x, y), (0.0, 25.0));
    }
}

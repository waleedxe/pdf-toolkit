//! Rendering tests using a programmatically generated PDF.

use std::path::PathBuf;

use lopdf::content::{Content, Operation};
use lopdf::{dictionary, Document, Object, Stream};
use pdf_render::{render_page, render_thumbnail, ImageFormat, MAX_DIMENSION};

fn build_pdf(page_count: usize) -> Vec<u8> {
    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();

    let font_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    });
    let resources_id = doc.add_object(dictionary! {
        "Font" => dictionary! { "F1" => font_id },
    });

    let mut kids: Vec<Object> = Vec::new();
    for index in 0..page_count {
        let content = Content {
            operations: vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec!["F1".into(), 36.into()]),
                Operation::new("Td", vec![72.into(), 700.into()]),
                Operation::new(
                    "Tj",
                    vec![Object::string_literal(format!("Page {}", index + 1))],
                ),
                Operation::new("ET", vec![]),
            ],
        };
        let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));
        let page_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
            "Contents" => content_id,
            "Resources" => resources_id,
        });
        kids.push(page_id.into());
    }

    doc.objects.insert(
        pages_id,
        Object::Dictionary(dictionary! {
            "Type" => "Pages",
            "Kids" => kids,
            "Count" => page_count as i64,
        }),
    );
    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    doc.trailer.set("Root", catalog_id);

    let mut bytes = Vec::new();
    doc.save_to(&mut bytes)
        .expect("failed to serialize test PDF");
    bytes
}

fn sample_path() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("pdf-render-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("temp dir");
    let path = dir.join("sample.pdf");
    std::fs::write(&path, build_pdf(2)).expect("write fixture");
    path
}

#[test]
fn renders_a_page_as_png() {
    let path = sample_path();
    let rendered = render_page(&path, 1, 1.0, ImageFormat::Png, 90).expect("render should succeed");

    assert!(rendered.width > 0 && rendered.height > 0);
    assert_eq!(&rendered.bytes[..8], b"\x89PNG\r\n\x1a\n");
}

#[test]
fn renders_a_page_as_jpeg() {
    let path = sample_path();
    let rendered =
        render_page(&path, 2, 1.0, ImageFormat::Jpeg, 80).expect("render should succeed");

    assert!(rendered.width > 0);
    assert_eq!(&rendered.bytes[..2], &[0xFF, 0xD8]);
}

#[test]
fn thumbnail_is_scaled_down() {
    let path = sample_path();
    let thumb = render_thumbnail(&path, 1, 120).expect("thumbnail should succeed");

    assert!(thumb.width <= 120, "width {} exceeded 120", thumb.width);
    assert!(thumb.height > 0);
    assert_eq!(&thumb.bytes[..8], b"\x89PNG\r\n\x1a\n");
}

#[test]
fn rejects_out_of_range_pages() {
    let path = sample_path();
    assert!(render_page(&path, 99, 1.0, ImageFormat::Png, 90).is_err());
}

#[test]
fn caps_absurd_scale() {
    let path = sample_path();
    let rendered = render_page(&path, 1, 1000.0, ImageFormat::Png, 90).expect("render");

    assert!(rendered.width <= MAX_DIMENSION);
    assert!(rendered.height <= MAX_DIMENSION);
}

fn dark_pixels(png: &[u8]) -> usize {
    let image = image::load_from_memory(png).expect("decode png").to_luma8();
    image.pixels().filter(|pixel| pixel.0[0] < 200).count()
}

#[test]
fn renders_an_arabic_watermark() {
    let source = sample_path();
    let dir = source.parent().expect("parent").to_path_buf();
    let output = dir.join("watermarked.pdf");

    let watermark = pdf_core::TextWatermark {
        text: "مسودة".to_string(),
        font_size: 96.0,
        opacity: 1.0,
        rotation: 0.0,
        color: 0.0,
        position: pdf_core::Position::Center,
    };
    pdf_core::watermark_text(&source, None, &watermark, &output).expect("watermark");

    let before = render_page(&source, 1, 1.0, ImageFormat::Png, 90).expect("render base");
    let after = render_page(&output, 1, 1.0, ImageFormat::Png, 90).expect("render watermarked");

    // The watermark must add a substantial amount of ink to the page.
    let added = dark_pixels(&after.bytes).saturating_sub(dark_pixels(&before.bytes));
    assert!(
        added > 500,
        "expected the Arabic watermark to render visible pixels, added {added}"
    );
}

//! Tests for generated text (watermark / page numbers), including Arabic.

use std::path::{Path, PathBuf};

use image::{Rgb, RgbImage};
use lopdf::content::{Content, Operation};
use lopdf::{dictionary, Document, Object, Stream};
use pdf_core::{
    add_page_numbers, shape_text, watermark_image, watermark_text, Horizontal, ImageWatermark,
    PageNumberOptions, PdfDocument, Position, TextWatermark, Vertical,
};

fn build_pdf(page_count: usize, with_text: bool) -> Vec<u8> {
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
        let operations = if with_text {
            vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec!["F1".into(), 24.into()]),
                Operation::new("Td", vec![72.into(), 700.into()]),
                Operation::new(
                    "Tj",
                    vec![Object::string_literal(format!("Page {}", index + 1))],
                ),
                Operation::new("ET", vec![]),
            ]
        } else {
            vec![]
        };
        let content = Content { operations };
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
    doc.save_to(&mut bytes).expect("serialize");
    bytes
}

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("pdf-core-text-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("temp dir");
    dir
}

fn write_pdf(dir: &Path, name: &str, pages: usize, with_text: bool) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, build_pdf(pages, with_text)).expect("write fixture");
    path
}

#[test]
fn arabic_text_shapes_into_non_empty_glyphs() {
    let shaped = shape_text(pdf_core::EMBEDDED_FONT, "مسودة", 48.0).expect("shape");
    assert!(!shaped.glyphs.is_empty());
    assert!(shaped.glyphs.iter().any(|glyph| *glyph != 0));
    assert!(shaped.advance > 0.0);
}

#[test]
fn mixed_arabic_and_english_shapes() {
    let shaped = shape_text(pdf_core::EMBEDDED_FONT, "Draft مسودة 2026", 24.0).expect("shape");
    assert!(!shaped.glyphs.is_empty());
    // Glyph ids are emitted as 4-hex-digit pairs.
    let hex = shaped.hex();
    assert!(hex.starts_with('<') && hex.ends_with('>'));
    assert_eq!(hex.len() - 2, shaped.glyphs.len() * 4);
}

#[test]
fn text_watermark_embeds_font_and_keeps_pages() {
    let dir = temp_dir("wm-text");
    let source = write_pdf(&dir, "source.pdf", 2, true);
    let output = dir.join("watermarked.pdf");

    let watermark = TextWatermark {
        text: "مسودة Draft".to_string(),
        font_size: 48.0,
        opacity: 0.35,
        rotation: 45.0,
        color: 0.5,
        position: Position::Center,
    };
    let count = watermark_text(&source, None, &watermark, &output).expect("watermark");
    assert_eq!(count, 2);
    assert_eq!(PdfDocument::open(&output).unwrap().page_count(), 2);

    // The embedded font name proves a Type0 font was added to the document.
    let bytes = std::fs::read(&output).unwrap();
    assert!(contains(&bytes, b"Amiri-Regular"));
    assert!(contains(&bytes, b"Identity-H"));

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn image_watermark_adds_an_xobject() {
    let dir = temp_dir("wm-image");
    let source = write_pdf(&dir, "source.pdf", 1, false);
    let output = dir.join("watermarked.pdf");

    let mut logo = RgbImage::new(30, 30);
    for pixel in logo.pixels_mut() {
        *pixel = Rgb([10, 120, 200]);
    }
    let logo_path = dir.join("logo.png");
    logo.save(&logo_path).unwrap();

    let watermark = ImageWatermark {
        scale: 0.5,
        opacity: 0.8,
        rotation: 0.0,
        position: Position::Center,
    };
    watermark_image(&source, &logo_path, None, &watermark, &output).expect("watermark image");

    let bytes = std::fs::read(&output).unwrap();
    assert!(contains(&bytes, b"Imwm"));
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn page_numbers_are_added_to_selected_pages() {
    let dir = temp_dir("page-numbers");
    let source = write_pdf(&dir, "source.pdf", 3, true);
    let output = dir.join("numbered.pdf");

    let options = PageNumberOptions {
        vertical: Vertical::Bottom,
        horizontal: Horizontal::Center,
        start: 1,
        font_size: 12.0,
        margin: 36.0,
        color: 0.0,
    };
    let count = add_page_numbers(&source, Some(&[1, 3]), &options, &output).expect("numbers");
    assert_eq!(count, 3);
    assert_eq!(PdfDocument::open(&output).unwrap().page_count(), 3);

    let bytes = std::fs::read(&output).unwrap();
    assert!(contains(&bytes, b"Amiri-Regular"));
    std::fs::remove_dir_all(&dir).ok();
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

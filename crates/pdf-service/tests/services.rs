//! Integration tests for the application-level services.

use std::path::{Path, PathBuf};

use lopdf::content::{Content, Operation};
use lopdf::{dictionary, Document, Object, Stream};
use pdf_render::ImageFormat;
use pdf_service::filesystem::{unique_path, unique_run_stem};
use pdf_service::{extract, reorder, split, to_images, SplitMode};

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
    doc.save_to(&mut bytes).expect("serialize");
    bytes
}

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("pdf-service-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("temp dir");
    dir
}

fn write_pdf(dir: &Path, name: &str, pages: usize) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, build_pdf(pages)).expect("write fixture");
    path
}

fn page_count(path: &std::path::Path) -> u32 {
    pdf_core::page_count(path).expect("page count")
}

#[test]
fn unique_path_never_overwrites() {
    let dir = temp_dir("unique");
    let first = unique_path(&dir, "report", "pdf");
    assert_eq!(first.file_name().unwrap(), "report.pdf");
    std::fs::write(&first, b"x").unwrap();

    let second = unique_path(&dir, "report", "pdf");
    assert_eq!(second.file_name().unwrap(), "report_1.pdf");

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn unique_run_stem_avoids_existing_runs() {
    let dir = temp_dir("run");
    std::fs::write(dir.join("doc_split_001.pdf"), b"x").unwrap();
    assert_eq!(unique_run_stem(&dir, "doc_split", "pdf"), "doc_split_1");
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn extract_and_reorder_produce_expected_pages() {
    let dir = temp_dir("extract-reorder");
    let source = write_pdf(&dir, "source.pdf", 5);

    let extracted = dir.join("extracted.pdf");
    assert_eq!(extract(&source, "3,1", &extracted).unwrap(), 2);
    assert_eq!(page_count(&extracted), 2);

    let reordered = dir.join("reordered.pdf");
    assert_eq!(reorder(&source, &[5, 4, 3, 2, 1], &reordered).unwrap(), 5);
    assert_eq!(page_count(&reordered), 5);

    assert_eq!(page_count(&source), 5, "original must be untouched");
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn split_ranges_writes_contiguous_groups() {
    let dir = temp_dir("split");
    let source = write_pdf(&dir, "source.pdf", 6);

    let outputs = split(&source, SplitMode::Ranges, Some("1-3,5-6"), &dir, "doc").unwrap();
    assert_eq!(outputs.len(), 2);
    assert_eq!(page_count(&outputs[0]), 3);
    assert_eq!(page_count(&outputs[1]), 2);

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn to_images_writes_png_files() {
    let dir = temp_dir("to-images");
    let source = write_pdf(&dir, "source.pdf", 2);

    let outputs = to_images(&source, Some("2"), 72.0, ImageFormat::Png, 90, &dir, "page").unwrap();

    assert_eq!(outputs.len(), 1);
    let bytes = std::fs::read(&outputs[0]).unwrap();
    assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n");

    std::fs::remove_dir_all(&dir).ok();
}

mod common;

use std::path::{Path, PathBuf};

use lopdf::{Document, Object};
use pdf_core::{
    extract_pages, page_count, remove_pages, rotate_pages, split_pdf, PdfCoreError, PdfDocument,
};

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("pdf-core-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

fn rotation_of(path: &Path, page: u32) -> Option<i64> {
    let document = Document::load(path).expect("output should load");
    let id = document.get_pages()[&page];
    match document.get_object(id).expect("page should exist") {
        Object::Dictionary(dict) => dict
            .get(b"Rotate")
            .ok()
            .and_then(|value| value.as_i64().ok()),
        _ => None,
    }
}

/// The value a page holds directly under `key`, or `None` if it only inherits it.
fn own_attribute(path: &Path, page: u32, key: &[u8]) -> Option<Object> {
    let document = Document::load(path).expect("output should load");
    let id = document.get_pages()[&page];
    match document.get_object(id).expect("page should exist") {
        Object::Dictionary(dict) => dict.get(key).ok().cloned(),
        _ => None,
    }
}

#[test]
fn extract_keeps_selected_pages_and_order() {
    let dir = temp_dir("extract");
    let source = common::write_pdf(&dir, "source.pdf", 5);
    let output = dir.join("extracted.pdf");

    let count = extract_pages(&source, &[3, 1, 5], &output).expect("extract should succeed");
    assert_eq!(count, 3);
    assert_eq!(PdfDocument::open(&output).unwrap().page_count(), 3);
    assert_eq!(PdfDocument::open(&source).unwrap().page_count(), 5);

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn extract_rejects_out_of_range_pages() {
    let dir = temp_dir("extract-range");
    let source = common::write_pdf(&dir, "source.pdf", 3);
    let output = dir.join("extracted.pdf");

    let result = extract_pages(&source, &[4], &output);
    assert!(matches!(
        result,
        Err(PdfCoreError::PageOutOfRange { page: 4, total: 3 })
    ));
    assert!(!output.exists());

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn remove_keeps_the_complement() {
    let dir = temp_dir("remove");
    let source = common::write_pdf(&dir, "source.pdf", 5);
    let output = dir.join("removed.pdf");

    let count = remove_pages(&source, &[2, 4], &output).expect("remove should succeed");
    assert_eq!(count, 3);
    assert_eq!(PdfDocument::open(&output).unwrap().page_count(), 3);

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn remove_rejects_removing_every_page() {
    let dir = temp_dir("remove-all");
    let source = common::write_pdf(&dir, "source.pdf", 2);
    let output = dir.join("removed.pdf");

    let result = remove_pages(&source, &[1, 2], &output);
    assert!(matches!(result, Err(PdfCoreError::Other(_))));
    assert!(!output.exists());

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn split_writes_one_file_per_group() {
    let dir = temp_dir("split");
    let source = common::write_pdf(&dir, "source.pdf", 4);

    let groups = vec![vec![1], vec![2, 3], vec![4]];
    let outputs: Vec<PathBuf> = ["a.pdf", "b.pdf", "c.pdf"]
        .iter()
        .map(|name| dir.join(name))
        .collect();

    let written = split_pdf(&source, &groups, &outputs).expect("split should succeed");
    assert_eq!(written, 4);

    assert_eq!(PdfDocument::open(&outputs[0]).unwrap().page_count(), 1);
    assert_eq!(PdfDocument::open(&outputs[1]).unwrap().page_count(), 2);
    assert_eq!(PdfDocument::open(&outputs[2]).unwrap().page_count(), 1);

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn rotate_all_pages_sets_rotation_metadata() {
    let dir = temp_dir("rotate-all");
    let source = common::write_pdf(&dir, "source.pdf", 3);
    let output = dir.join("rotated.pdf");

    rotate_pages(&source, None, 90, &output).expect("rotate should succeed");
    assert_eq!(rotation_of(&output, 1), Some(90));
    assert_eq!(rotation_of(&output, 3), Some(90));

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn rotate_selected_pages_only_and_accumulates() {
    let dir = temp_dir("rotate-selected");
    let source = common::write_pdf(&dir, "source.pdf", 3);
    let first = dir.join("first.pdf");
    let second = dir.join("second.pdf");

    rotate_pages(&source, Some(&[2]), 90, &first).expect("rotate should succeed");
    assert_eq!(rotation_of(&first, 2), Some(90));
    assert_eq!(rotation_of(&first, 1), None);

    // 90 + 270 = 360, which normalizes back to "no rotation".
    rotate_pages(&first, Some(&[2]), 270, &second).expect("rotate should succeed");
    assert_eq!(rotation_of(&second, 2), None);

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn rotate_rejects_non_multiple_of_90() {
    let dir = temp_dir("rotate-bad");
    let source = common::write_pdf(&dir, "source.pdf", 1);
    let output = dir.join("rotated.pdf");

    let result = rotate_pages(&source, None, 45, &output);
    assert!(matches!(result, Err(PdfCoreError::Other(_))));
    assert!(!output.exists());

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn page_count_reads_documents() {
    let dir = temp_dir("count");
    let source = common::write_pdf(&dir, "source.pdf", 7);
    assert_eq!(page_count(&source).unwrap(), 7);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn extract_materializes_inherited_attributes() {
    let dir = temp_dir("extract-inherited");
    let source = common::write_inherited_pdf(&dir, "source.pdf", 3);
    let output = dir.join("extracted.pdf");

    let count = extract_pages(&source, &[2], &output).expect("extract should succeed");
    assert_eq!(count, 1);

    // The source page inherits both attributes from the parent `/Pages` node;
    // the rebuilt document must carry them on the page itself.
    let media = own_attribute(&output, 1, b"MediaBox").expect("page needs its own MediaBox");
    assert!(matches!(media, Object::Array(values) if values.len() == 4));
    assert!(
        own_attribute(&output, 1, b"Resources").is_some(),
        "page needs its own Resources"
    );

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn split_materializes_inherited_attributes() {
    let dir = temp_dir("split-inherited");
    let source = common::write_inherited_pdf(&dir, "source.pdf", 2);
    let outputs: Vec<PathBuf> = ["a.pdf", "b.pdf"]
        .iter()
        .map(|name| dir.join(name))
        .collect();

    split_pdf(&source, &[vec![1], vec![2]], &outputs).expect("split should succeed");

    for output in &outputs {
        assert!(
            own_attribute(output, 1, b"MediaBox").is_some(),
            "split page needs its own MediaBox"
        );
        assert!(
            own_attribute(output, 1, b"Resources").is_some(),
            "split page needs its own Resources"
        );
    }

    std::fs::remove_dir_all(&dir).ok();
}

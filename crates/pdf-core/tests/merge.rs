mod common;

use pdf_core::tools::merge;
use pdf_core::{PdfCoreError, PdfDocument};

fn temp_dir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("pdf-core-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

#[test]
fn merges_documents_and_preserves_total_page_count() {
    let dir = temp_dir("merge");
    let first = common::write_pdf(&dir, "first.pdf", 2);
    let second = common::write_pdf(&dir, "second.pdf", 3);
    let output = dir.join("merged.pdf");

    let count = merge(&[first.clone(), second.clone()], &output).expect("merge should succeed");
    assert_eq!(count, 5);

    let merged = PdfDocument::open(&output).expect("merged output should load");
    assert_eq!(merged.page_count(), 5);

    // Originals must remain untouched.
    assert_eq!(PdfDocument::open(&first).unwrap().page_count(), 2);
    assert_eq!(PdfDocument::open(&second).unwrap().page_count(), 3);

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn merging_a_single_document_keeps_its_pages() {
    let dir = temp_dir("merge-single");
    let only = common::write_pdf(&dir, "only.pdf", 4);
    let output = dir.join("merged.pdf");

    let count = merge(&[only], &output).expect("merge should succeed");
    assert_eq!(count, 4);

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn merge_rejects_empty_input_list() {
    let dir = temp_dir("merge-empty");
    let output = dir.join("merged.pdf");

    let result = merge(&[], &output);
    assert!(matches!(result, Err(PdfCoreError::Other(_))));

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn merge_fails_safely_on_a_missing_input() {
    let dir = temp_dir("merge-missing");
    let output = dir.join("merged.pdf");

    let result = merge(&[dir.join("does-not-exist.pdf")], &output);
    assert!(matches!(result, Err(PdfCoreError::Io { .. })));
    assert!(!output.exists(), "no output should be produced on failure");

    std::fs::remove_dir_all(&dir).ok();
}

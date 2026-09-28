//! Performance / scale test: documents with 100+ pages.
//!
//! Kept correct-and-bounded rather than timing-sensitive; the goal is to prove
//! large documents complete and produce the right page counts.

mod common;

use std::path::PathBuf;

use pdf_core::{
    extract_pages, merge, page_count, remove_pages, rotate_pages, split_pdf, PdfDocument,
};

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("pdf-core-large-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("temp dir");
    dir
}

#[test]
fn handles_documents_with_more_than_a_hundred_pages() {
    let dir = temp_dir("scale");
    let first = common::write_pdf(&dir, "first.pdf", 150);
    let second = common::write_pdf(&dir, "second.pdf", 150);

    assert_eq!(page_count(&first).unwrap(), 150);

    let extracted = dir.join("extracted.pdf");
    assert_eq!(extract_pages(&first, &[1, 75, 150], &extracted).unwrap(), 3);
    assert_eq!(PdfDocument::open(&extracted).unwrap().page_count(), 3);

    let rotated = dir.join("rotated.pdf");
    rotate_pages(&first, None, 90, &rotated).unwrap();
    assert_eq!(PdfDocument::open(&rotated).unwrap().page_count(), 150);

    let removed = dir.join("removed.pdf");
    let to_remove: Vec<u32> = (1..=140).collect();
    assert_eq!(remove_pages(&first, &to_remove, &removed).unwrap(), 10);

    let outputs: Vec<PathBuf> = ["part_1.pdf", "part_2.pdf"]
        .iter()
        .map(|name| dir.join(name))
        .collect();
    let written = split_pdf(
        &first,
        &[(1..=100).collect(), (101..=150).collect()],
        &outputs,
    )
    .unwrap();
    assert_eq!(written, 150);
    assert_eq!(PdfDocument::open(&outputs[0]).unwrap().page_count(), 100);
    assert_eq!(PdfDocument::open(&outputs[1]).unwrap().page_count(), 50);

    let merged = dir.join("merged.pdf");
    assert_eq!(merge(&[first.clone(), second], &merged).unwrap(), 300);
    assert_eq!(PdfDocument::open(&merged).unwrap().page_count(), 300);

    // Originals remain unchanged.
    assert_eq!(page_count(&first).unwrap(), 150);

    std::fs::remove_dir_all(&dir).ok();
}

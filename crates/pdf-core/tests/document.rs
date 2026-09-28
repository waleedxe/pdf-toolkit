mod common;

use pdf_core::{PdfCoreError, PdfDocument};

#[test]
fn reports_page_count_for_multi_page_document() {
    let bytes = common::build_pdf(3);
    let document = PdfDocument::from_bytes(&bytes).expect("should load generated PDF");
    assert_eq!(document.page_count(), 3);
}

#[test]
fn reports_page_count_for_single_page_document() {
    let bytes = common::build_pdf(1);
    let document = PdfDocument::from_bytes(&bytes).expect("should load generated PDF");
    assert_eq!(document.page_count(), 1);
}

#[test]
fn rejects_malformed_input_safely() {
    let result = PdfDocument::from_bytes(b"this is definitely not a pdf");
    assert!(matches!(result, Err(PdfCoreError::InvalidPdf(_))));
}

#[test]
fn opening_a_missing_file_reports_io_error() {
    let result = PdfDocument::open("/definitely/not/here/nope.pdf");
    assert!(matches!(result, Err(PdfCoreError::Io { .. })));
}

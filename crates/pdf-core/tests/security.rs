mod common;

use std::path::PathBuf;

use lopdf::Document;
use pdf_core::{protect, unlock, PdfCoreError, PdfDocument, ProtectOptions};

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("pdf-core-sec-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("temp dir");
    dir
}

fn protected_options() -> ProtectOptions {
    ProtectOptions {
        user_password: "s3cret".to_string(),
        owner_password: None,
        allow_printing: true,
        allow_copying: true,
        allow_modifying: false,
    }
}

#[test]
fn protect_then_unlock_with_the_correct_password() {
    let dir = temp_dir("roundtrip");
    let source = common::write_pdf(&dir, "source.pdf", 2);
    let protected = dir.join("protected.pdf");

    assert_eq!(
        protect(&source, &protected_options(), &protected).unwrap(),
        2
    );

    // The output must actually be encrypted: the structure still loads, but it
    // is marked encrypted and cannot be decrypted with the wrong password.
    let loaded = Document::load(&protected).unwrap();
    assert!(loaded.is_encrypted());
    assert!(Document::load_with_password(&protected, "wrong").is_err());

    let unlocked = dir.join("unlocked.pdf");
    assert_eq!(unlock(&protected, "s3cret", &unlocked).unwrap(), 2);
    assert_eq!(PdfDocument::open(&unlocked).unwrap().page_count(), 2);

    // The original file is untouched.
    assert_eq!(PdfDocument::open(&source).unwrap().page_count(), 2);

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn unlock_with_the_wrong_password_fails() {
    let dir = temp_dir("wrong");
    let source = common::write_pdf(&dir, "source.pdf", 1);
    let protected = dir.join("protected.pdf");
    protect(&source, &protected_options(), &protected).unwrap();

    let unlocked = dir.join("unlocked.pdf");
    let result = unlock(&protected, "not-it", &unlocked);
    assert!(matches!(result, Err(PdfCoreError::WrongPassword)));
    assert!(!unlocked.exists());

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn unlock_rejects_an_unprotected_document() {
    let dir = temp_dir("plain");
    let source = common::write_pdf(&dir, "source.pdf", 1);
    let unlocked = dir.join("unlocked.pdf");

    let result = unlock(&source, "anything", &unlocked);
    assert!(matches!(result, Err(PdfCoreError::Other(_))));
    assert!(!unlocked.exists());

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn protect_requires_a_password() {
    let dir = temp_dir("empty-password");
    let source = common::write_pdf(&dir, "source.pdf", 1);
    let protected = dir.join("protected.pdf");

    let options = ProtectOptions {
        user_password: String::new(),
        owner_password: None,
        allow_printing: true,
        allow_copying: true,
        allow_modifying: true,
    };
    assert!(matches!(
        protect(&source, &options, &protected),
        Err(PdfCoreError::Other(_))
    ));
    assert!(!protected.exists());

    std::fs::remove_dir_all(&dir).ok();
}

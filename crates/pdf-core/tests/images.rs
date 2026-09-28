use std::path::{Path, PathBuf};

use image::{Rgb, RgbImage};
use pdf_core::{
    images_to_pdf, FitMode, ImagesToPdfOptions, Orientation, PageSize, PdfCoreError, PdfDocument,
};

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("pdf-core-images-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("temp dir");
    dir
}

fn write_png(dir: &Path, name: &str, width: u32, height: u32) -> PathBuf {
    let mut image = RgbImage::new(width, height);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        *pixel = Rgb([(x % 256) as u8, (y % 256) as u8, 128]);
    }
    let path = dir.join(name);
    image.save(&path).expect("save png");
    path
}

fn write_jpg(dir: &Path, name: &str, width: u32, height: u32) -> PathBuf {
    let mut image = RgbImage::new(width, height);
    for pixel in image.pixels_mut() {
        *pixel = Rgb([200, 100, 50]);
    }
    let path = dir.join(name);
    image.save(&path).expect("save jpg");
    path
}

#[test]
fn converts_png_and_jpg_in_order() {
    let dir = temp_dir("convert");
    let first = write_png(&dir, "a.png", 40, 20);
    let second = write_jpg(&dir, "b.jpg", 20, 40);
    let output = dir.join("images.pdf");

    let options = ImagesToPdfOptions {
        page_size: PageSize::A4,
        orientation: Orientation::Auto,
        fit: FitMode::Contain,
        margin: 12.0,
    };

    let count = images_to_pdf(&[first.clone(), second.clone()], options, &output)
        .expect("conversion should succeed");
    assert_eq!(count, 2);
    assert_eq!(PdfDocument::open(&output).unwrap().page_count(), 2);

    // Inputs are untouched.
    assert!(first.exists() && second.exists());

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn rejects_an_empty_image_list() {
    let dir = temp_dir("empty");
    let output = dir.join("images.pdf");
    let result = images_to_pdf(&[], ImagesToPdfOptions::default(), &output);
    assert!(matches!(result, Err(PdfCoreError::Other(_))));
    assert!(!output.exists());
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn rejects_a_file_that_is_not_an_image() {
    let dir = temp_dir("not-image");
    let bogus = dir.join("not.png");
    std::fs::write(&bogus, b"this is not an image").unwrap();
    let output = dir.join("images.pdf");

    let result = images_to_pdf(&[bogus], ImagesToPdfOptions::default(), &output);
    assert!(matches!(result, Err(PdfCoreError::Other(_))));
    assert!(!output.exists());

    std::fs::remove_dir_all(&dir).ok();
}

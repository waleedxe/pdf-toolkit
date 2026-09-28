use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use lopdf::{dictionary, Document, Object, ObjectId};

use crate::error::{PdfCoreError, PdfCoreResult};

/// Merge multiple PDFs into a single document, preserving page order.
///
/// `inputs` are merged in the given order. The original files are never
/// modified; the result is written to `output`.
///
/// Returns the number of pages in the merged document.
pub fn merge(inputs: &[PathBuf], output: &Path) -> PdfCoreResult<u32> {
    if inputs.is_empty() {
        return Err(PdfCoreError::Other(
            "merge requires at least one input file".to_string(),
        ));
    }

    let mut max_id: u32 = 1;
    let mut pages: Vec<Object> = Vec::new();
    let mut objects: BTreeMap<ObjectId, Object> = BTreeMap::new();

    for path in inputs {
        let mut document = Document::load(path).map_err(|error| map_load(path, error))?;

        // Shift every object id so ids stay unique across documents.
        document.renumber_objects_with(max_id);
        max_id = document.max_id + 1;

        for (_, page_id) in document.get_pages() {
            let page = document
                .get_object(page_id)
                .map_err(PdfCoreError::from_engine)?
                .to_owned();
            pages.push(page);
        }

        objects.extend(document.objects);
    }

    if pages.is_empty() {
        return Err(PdfCoreError::Other(
            "the selected files contain no pages to merge".to_string(),
        ));
    }

    let mut merged = Document::with_version("1.5");
    merged.objects = objects;
    merged.max_id = max_id;

    // Build a fresh page tree that references every collected page.
    let pages_id = merged.new_object_id();
    let mut kids: Vec<Object> = Vec::with_capacity(pages.len());
    for page in pages {
        let mut page = page;
        if let Object::Dictionary(dict) = &mut page {
            dict.set("Parent", pages_id);
        }
        kids.push(merged.add_object(page).into());
    }
    let page_count = kids.len() as u32;

    merged.objects.insert(
        pages_id,
        Object::Dictionary(dictionary! {
            "Type" => "Pages",
            "Kids" => kids,
            "Count" => page_count as i64,
        }),
    );

    let catalog_id = merged.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    merged.trailer.set("Root", catalog_id);

    merged.renumber_objects();
    merged
        .save(output)
        .map_err(|error| PdfCoreError::Other(format!("failed to write merged PDF: {error}")))?;

    Ok(page_count)
}

fn map_load(path: &Path, error: lopdf::Error) -> PdfCoreError {
    if let lopdf::Error::IO(io) = &error {
        return PdfCoreError::Io {
            path: path.to_path_buf(),
            source: std::io::Error::new(io.kind(), io.to_string()),
        };
    }
    PdfCoreError::from_engine(error)
}

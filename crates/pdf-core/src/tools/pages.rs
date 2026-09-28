use std::collections::HashSet;
use std::path::{Path, PathBuf};

use lopdf::{dictionary, Document, Object, ObjectId};

use crate::error::{PdfCoreError, PdfCoreResult};

/// Number of pages in a PDF on disk.
pub fn page_count(input: &Path) -> PdfCoreResult<u32> {
    let document = Document::load(input).map_err(|error| map_load(input, error))?;
    Ok(document.get_pages().len() as u32)
}

/// Extract the given 1-based pages, in the given order, into a new document.
pub fn extract_pages(input: &Path, pages: &[u32], output: &Path) -> PdfCoreResult<u32> {
    if pages.is_empty() {
        return Err(PdfCoreError::Other(
            "select at least one page to extract".to_string(),
        ));
    }

    let source = Document::load(input).map_err(|error| map_load(input, error))?;
    let mut document = rebuild(&source, pages)?;
    save(&mut document, output)?;
    Ok(pages.len() as u32)
}

/// Remove the given 1-based pages and write the remaining pages in order.
pub fn remove_pages(input: &Path, pages: &[u32], output: &Path) -> PdfCoreResult<u32> {
    if pages.is_empty() {
        return Err(PdfCoreError::Other(
            "select at least one page to remove".to_string(),
        ));
    }

    let source = Document::load(input).map_err(|error| map_load(input, error))?;
    let total = source.get_pages().len() as u32;
    for &page in pages {
        ensure_in_range(page, total)?;
    }

    let removed: HashSet<u32> = pages.iter().copied().collect();
    let keep: Vec<u32> = (1..=total).filter(|page| !removed.contains(page)).collect();
    if keep.is_empty() {
        return Err(PdfCoreError::Other(
            "cannot remove every page from the document".to_string(),
        ));
    }

    let mut document = rebuild(&source, &keep)?;
    save(&mut document, output)?;
    Ok(keep.len() as u32)
}

/// Split a document into several files, one per page group.
///
/// `groups` holds 1-based page selections (already validated by the caller) and
/// `outputs` must contain exactly one path per group. Returns the total number
/// of pages written across all outputs.
pub fn split_pdf(input: &Path, groups: &[Vec<u32>], outputs: &[PathBuf]) -> PdfCoreResult<u32> {
    if groups.is_empty() {
        return Err(PdfCoreError::Other(
            "provide at least one split group".to_string(),
        ));
    }
    if groups.len() != outputs.len() {
        return Err(PdfCoreError::Other(
            "every split group needs exactly one output path".to_string(),
        ));
    }

    let source = Document::load(input).map_err(|error| map_load(input, error))?;
    let mut written = 0u32;
    for (group, output) in groups.iter().zip(outputs) {
        if group.is_empty() {
            return Err(PdfCoreError::Other(
                "a split group cannot be empty".to_string(),
            ));
        }
        let mut document = rebuild(&source, group)?;
        save(&mut document, output)?;
        written += group.len() as u32;
    }
    Ok(written)
}

/// Rotate selected pages (or all pages when `pages` is `None`) clockwise.
///
/// `rotation` must be a multiple of 90; negative values rotate counter-clockwise.
/// Existing rotation is preserved by adding to the page's current `/Rotate`.
pub fn rotate_pages(
    input: &Path,
    pages: Option<&[u32]>,
    rotation: i64,
    output: &Path,
) -> PdfCoreResult<u32> {
    if rotation % 90 != 0 {
        return Err(PdfCoreError::Other(
            "rotation must be a multiple of 90 degrees".to_string(),
        ));
    }

    let mut document = Document::load(input).map_err(|error| map_load(input, error))?;
    let total = document.get_pages().len() as u32;
    let selected: Vec<u32> = match pages {
        Some(list) => {
            for &page in list {
                ensure_in_range(page, total)?;
            }
            list.to_vec()
        }
        None => (1..=total).collect(),
    };

    let page_map = document.get_pages();
    let ids: Vec<ObjectId> = selected
        .iter()
        .map(|page| {
            page_map
                .get(page)
                .copied()
                .ok_or(PdfCoreError::PageOutOfRange { page: *page, total })
        })
        .collect::<PdfCoreResult<_>>()?;

    for id in ids {
        if let Some(Object::Dictionary(dict)) = document.objects.get_mut(&id) {
            let current = dict
                .get(b"Rotate")
                .ok()
                .and_then(|value| value.as_i64().ok())
                .unwrap_or(0);
            let next = (current + rotation).rem_euclid(360);
            if next == 0 {
                dict.remove(b"Rotate");
            } else {
                dict.set("Rotate", next);
            }
        }
    }

    save(&mut document, output)?;
    Ok(total)
}

/// Build a fresh document containing only `pages`, in the given order.
///
/// Object ids are preserved (only the page tree and catalog are rebuilt), which
/// keeps cross-references inside the copied pages valid. Unreferenced objects
/// from the source are harmless and may be pruned by a later optimization.
fn rebuild(source: &Document, pages: &[u32]) -> PdfCoreResult<Document> {
    let page_map = source.get_pages();
    let total = page_map.len() as u32;

    let mut selected: Vec<ObjectId> = Vec::with_capacity(pages.len());
    for &page in pages {
        ensure_in_range(page, total)?;
        let id = page_map
            .get(&page)
            .copied()
            .ok_or(PdfCoreError::PageOutOfRange { page, total })?;
        selected.push(id);
    }

    let mut document = source.clone();
    let mut objects = std::mem::take(&mut document.objects);

    let mut next_id = document.max_id.saturating_add(1);
    let pages_id: ObjectId = (next_id, 0);
    next_id += 1;

    for &id in &selected {
        if let Some(Object::Dictionary(dict)) = objects.get_mut(&id) {
            dict.set("Parent", pages_id);
        }
    }

    let kids: Vec<Object> = selected.iter().map(|&id| Object::Reference(id)).collect();
    let count = kids.len() as i64;
    objects.insert(
        pages_id,
        Object::Dictionary(dictionary! {
            "Type" => "Pages",
            "Kids" => kids,
            "Count" => count,
        }),
    );

    let catalog_id: ObjectId = (next_id, 0);
    objects.insert(
        catalog_id,
        Object::Dictionary(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        }),
    );

    document.objects = objects;
    document.max_id = next_id;
    document.trailer.set("Root", catalog_id);
    Ok(document)
}

fn ensure_in_range(page: u32, total: u32) -> PdfCoreResult<()> {
    if page < 1 || page > total {
        return Err(PdfCoreError::PageOutOfRange { page, total });
    }
    Ok(())
}

fn save(document: &mut Document, output: &Path) -> PdfCoreResult<()> {
    document
        .save(output)
        .map_err(|error| PdfCoreError::Other(format!("failed to write PDF: {error}")))?;
    Ok(())
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

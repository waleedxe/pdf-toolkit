//! Shared helpers for editing existing pages (used by Watermark and Page
//! Numbers): reading page geometry, appending content streams, and merging
//! resource entries without clobbering existing ones.

use std::io::Write;

use flate2::write::ZlibEncoder;
use flate2::Compression;
use lopdf::{Dictionary, Object, ObjectId, Stream};

use crate::error::{PdfCoreError, PdfCoreResult};

/// Load a PDF, mapping engine errors to the core error model.
pub(crate) fn load(path: &std::path::Path) -> PdfCoreResult<lopdf::Document> {
    lopdf::Document::load(path).map_err(|error| match &error {
        lopdf::Error::IO(io) => PdfCoreError::Io {
            path: path.to_path_buf(),
            source: std::io::Error::new(io.kind(), io.to_string()),
        },
        _ => PdfCoreError::from_engine(error),
    })
}

/// Resolve an optional page selection to a concrete list, validating bounds.
pub(crate) fn selected_pages(pages: Option<&[u32]>, total: u32) -> PdfCoreResult<Vec<u32>> {
    match pages {
        Some(list) => {
            for &page in list {
                if page < 1 || page > total {
                    return Err(PdfCoreError::PageOutOfRange { page, total });
                }
            }
            if list.is_empty() {
                return Err(PdfCoreError::Other(
                    "select at least one page to edit".to_string(),
                ));
            }
            Ok(list.to_vec())
        }
        None => Ok((1..=total).collect()),
    }
}

/// Flate-compress raw pixel or font data for a PDF stream.
pub(crate) fn deflate(data: &[u8]) -> PdfCoreResult<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(data)
        .map_err(|error| PdfCoreError::Other(format!("failed to compress data: {error}")))?;
    encoder
        .finish()
        .map_err(|error| PdfCoreError::Other(format!("failed to compress data: {error}")))
}

/// Width and height of a page in points (from its `/MediaBox`).
pub fn page_size(document: &lopdf::Document, page_id: ObjectId) -> PdfCoreResult<(f32, f32)> {
    let page = as_dictionary(document, page_id)?;
    let media = match page.get(b"MediaBox") {
        Ok(Object::Array(values)) => values,
        _ => {
            return Err(PdfCoreError::Other(
                "the page has no MediaBox to measure".to_string(),
            ))
        }
    };
    if media.len() < 4 {
        return Err(PdfCoreError::Other(
            "the page has a malformed MediaBox".to_string(),
        ));
    }
    let width = number(&media[2]) - number(&media[0]);
    let height = number(&media[3]) - number(&media[1]);
    Ok((width.abs(), height.abs()))
}

/// Append a content stream to a page, preserving its existing content.
pub fn append_content(
    document: &mut lopdf::Document,
    page_id: ObjectId,
    content: Stream,
) -> PdfCoreResult<()> {
    let content_id = document.add_object(content);
    let page = as_dictionary_mut(document, page_id)?;

    let merged = match page.get(b"Contents").ok().cloned() {
        Some(Object::Array(mut array)) => {
            array.push(Object::Reference(content_id));
            Object::Array(array)
        }
        Some(existing) => Object::Array(vec![existing, Object::Reference(content_id)]),
        None => Object::Reference(content_id),
    };
    page.set("Contents", merged);
    Ok(())
}

/// Insert `value` under `resources/<category>/<name>`, creating containers as
/// needed and leaving unrelated resource entries untouched.
pub fn set_resource(
    document: &mut lopdf::Document,
    page_id: ObjectId,
    category: &[u8],
    name: &str,
    value: ObjectId,
) -> PdfCoreResult<()> {
    let page = as_dictionary_mut(document, page_id)?;
    let resources = ensure_dict(page, b"Resources");
    let group = ensure_dict(resources, category);
    group.set(name, Object::Reference(value));
    Ok(())
}

fn ensure_dict<'a>(parent: &'a mut Dictionary, key: &[u8]) -> &'a mut Dictionary {
    if !matches!(parent.get(key), Ok(Object::Dictionary(_))) {
        parent.set(key, Object::Dictionary(Dictionary::new()));
    }
    match parent.get_mut(key) {
        Ok(Object::Dictionary(dict)) => dict,
        _ => unreachable!("the resource container was just created"),
    }
}

fn as_dictionary(document: &lopdf::Document, page_id: ObjectId) -> PdfCoreResult<&Dictionary> {
    match document.get_object(page_id) {
        Ok(Object::Dictionary(dict)) => Ok(dict),
        Ok(_) => Err(PdfCoreError::Other(
            "the page is not a dictionary".to_string(),
        )),
        Err(error) => Err(PdfCoreError::Other(error.to_string())),
    }
}

fn as_dictionary_mut(
    document: &mut lopdf::Document,
    page_id: ObjectId,
) -> PdfCoreResult<&mut Dictionary> {
    match document.objects.get_mut(&page_id) {
        Some(Object::Dictionary(dict)) => Ok(dict),
        Some(_) => Err(PdfCoreError::Other(
            "the page is not a dictionary".to_string(),
        )),
        None => Err(PdfCoreError::Other(
            "the page could not be found".to_string(),
        )),
    }
}

fn number(object: &Object) -> f32 {
    match object {
        Object::Integer(value) => *value as f32,
        Object::Real(value) => *value,
        _ => 0.0,
    }
}

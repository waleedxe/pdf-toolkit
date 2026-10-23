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
    crate::loading::load(path)
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

/// Page attributes a page may inherit from an ancestor `/Pages` node
/// (ISO 32000-1, Table 30).
static INHERITABLE_PAGE_KEYS: [&[u8]; 4] = [b"Resources", b"MediaBox", b"CropBox", b"Rotate"];

/// Maximum depth walked while following `/Parent` links, guarding against
/// malformed page trees that contain reference cycles.
const MAX_PARENT_DEPTH: usize = 64;

/// Width and height of a page in points.
///
/// Prefers `/CropBox` (the visible area) over `/MediaBox`, follows indirect
/// values, and walks the `/Parent` chain so a page that inherits its box from a
/// `/Pages` node is measured correctly.
pub fn page_size(document: &lopdf::Document, page_id: ObjectId) -> PdfCoreResult<(f32, f32)> {
    // Prefer the visible area (`/CropBox`) and fall back to `/MediaBox`; either
    // may be an indirect value or inherited from a `/Pages` ancestor.
    let boxes: [&[u8]; 2] = [b"CropBox", b"MediaBox"];
    let values = boxes
        .iter()
        .find_map(|key| match inherited_value(document, page_id, key) {
            Some(Object::Array(values)) if values.len() >= 4 => Some(values.as_slice()),
            _ => None,
        })
        .ok_or_else(|| PdfCoreError::Other("the page has no MediaBox to measure".to_string()))?;
    let width = number(&values[2]) - number(&values[0]);
    let height = number(&values[3]) - number(&values[1]);
    Ok((width.abs(), height.abs()))
}

/// Resolve an inheritable page attribute from the page or any ancestor
/// `/Pages` node, following indirect references.
fn inherited_value<'a>(
    document: &'a lopdf::Document,
    page_id: ObjectId,
    key: &[u8],
) -> Option<&'a Object> {
    let mut node_id = Some(page_id);
    for _ in 0..MAX_PARENT_DEPTH {
        let node = document.get_dictionary(node_id?).ok()?;
        if let Ok(value) = node.get_deref(key, document) {
            return Some(value);
        }
        node_id = node.get(b"Parent").and_then(Object::as_reference).ok();
    }
    None
}

/// Collect the inheritable attributes a page is missing, taken from its
/// `/Parent` chain.
///
/// Values are returned raw (an indirect reference stays a reference) so a
/// rebuilt page tree becomes self-contained without duplicating shared objects.
pub(crate) fn inherited_attributes(
    document: &lopdf::Document,
    page_id: ObjectId,
) -> Vec<(&'static [u8], Object)> {
    let mut attributes = Vec::new();
    for &key in INHERITABLE_PAGE_KEYS.iter() {
        let has_own = document
            .get_dictionary(page_id)
            .map(|page| page.has(key))
            .unwrap_or(false);
        if has_own {
            continue;
        }
        if let Some(value) = inherited_raw(document, page_id, key) {
            attributes.push((key, value.clone()));
        }
    }
    attributes
}

/// Like [`inherited_value`], but returns the raw value (not dereferenced) and
/// only looks at ancestors, never at the page itself.
fn inherited_raw<'a>(
    document: &'a lopdf::Document,
    page_id: ObjectId,
    key: &[u8],
) -> Option<&'a Object> {
    let mut node_id = document
        .get_dictionary(page_id)
        .ok()
        .and_then(|page| page.get(b"Parent").and_then(Object::as_reference).ok());
    for _ in 0..MAX_PARENT_DEPTH {
        let node = document.get_dictionary(node_id?).ok()?;
        if let Ok(value) = node.get(key) {
            return Some(value);
        }
        node_id = node.get(b"Parent").and_then(Object::as_reference).ok();
    }
    None
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
///
/// Real PDFs commonly store `/Resources` (and the category dictionaries below
/// it) as indirect references; those are resolved and edited in place so the
/// page's existing fonts and images survive. `/Resources` is also inherited from
/// `/Parent` when the page does not define its own.
pub fn set_resource(
    document: &mut lopdf::Document,
    page_id: ObjectId,
    category: &[u8],
    name: &str,
    value: ObjectId,
) -> PdfCoreResult<()> {
    let resources_id = resources_object(document, page_id)?;
    let group_id = ensure_dict_object(document, resources_id, category)?;
    document
        .get_dictionary_mut(group_id)
        .map_err(|error| PdfCoreError::Other(error.to_string()))?
        .set(name, Object::Reference(value));
    Ok(())
}

/// Object id of the dictionary holding a page's effective `/Resources`.
///
/// Follows an indirect reference (`/Resources 5 0 R`) and the `/Parent` chain.
/// Resources already on the page are edited in place (an inline dictionary is
/// promoted to an indirect object first). Resources inherited from an ancestor
/// are copied onto the page, leaving the ancestor and its sibling pages
/// untouched. A fresh dictionary is only created when none exist anywhere.
fn resources_object(document: &mut lopdf::Document, page_id: ObjectId) -> PdfCoreResult<ObjectId> {
    let mut node_id = Some(page_id);
    for _ in 0..MAX_PARENT_DEPTH {
        let Some(current) = node_id else { break };
        let (value, parent) = {
            let node = document
                .get_dictionary(current)
                .map_err(|error| PdfCoreError::Other(error.to_string()))?;
            (
                node.get(b"Resources").ok().cloned(),
                node.get(b"Parent").and_then(Object::as_reference).ok(),
            )
        };
        let inherited_dictionary = match value {
            // Resources already on the page: keep the object and edit it in place.
            Some(Object::Reference(id))
                if current == page_id && document.get_dictionary(id).is_ok() =>
            {
                return Ok(id);
            }
            Some(Object::Reference(id)) => document.get_dictionary(id).ok().cloned(),
            Some(Object::Dictionary(inner)) => Some(inner),
            _ => None,
        };
        if let Some(dictionary) = inherited_dictionary {
            let new_id = document.add_object(Object::Dictionary(dictionary));
            set_page_resources(document, page_id, new_id)?;
            return Ok(new_id);
        }
        node_id = parent;
    }

    let new_id = document.add_object(Object::Dictionary(Dictionary::new()));
    set_page_resources(document, page_id, new_id)?;
    Ok(new_id)
}

/// Point a page's `/Resources` at `resources_id`.
fn set_page_resources(
    document: &mut lopdf::Document,
    page_id: ObjectId,
    resources_id: ObjectId,
) -> PdfCoreResult<()> {
    document
        .get_dictionary_mut(page_id)
        .map_err(|error| PdfCoreError::Other(error.to_string()))?
        .set("Resources", Object::Reference(resources_id));
    Ok(())
}

/// Object id of the dictionary stored at `dict_id[key]`, creating one when the
/// key is absent and promoting an inline dictionary to an indirect object so it
/// can be edited in place.
fn ensure_dict_object(
    document: &mut lopdf::Document,
    dict_id: ObjectId,
    key: &[u8],
) -> PdfCoreResult<ObjectId> {
    let existing = document
        .get_dictionary(dict_id)
        .map_err(|error| PdfCoreError::Other(error.to_string()))?
        .get(key)
        .ok()
        .cloned();
    let new_id = match existing {
        Some(Object::Reference(id)) if document.get_dictionary(id).is_ok() => return Ok(id),
        Some(Object::Dictionary(inner)) => document.add_object(Object::Dictionary(inner)),
        _ => document.add_object(Object::Dictionary(Dictionary::new())),
    };
    document
        .get_dictionary_mut(dict_id)
        .map_err(|error| PdfCoreError::Other(error.to_string()))?
        .set(key, Object::Reference(new_id));
    Ok(new_id)
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

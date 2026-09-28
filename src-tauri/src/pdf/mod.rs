//! PDF service layer.
//!
//! This is the only place in the Tauri app allowed to touch the PDF engine.
//! It re-exports the engine-agnostic core and hosts per-tool services
//! (merge, split, extract, remove, rotate) that translate UI requests into
//! core operations.

use std::path::{Path, PathBuf};

use pdf_core::parse_page_selection;

use crate::filesystem;

pub use pdf_core::{version as core_version, PdfCoreError, PdfDocument};

/// Merge several PDFs into a new file, preserving input order.
///
/// The originals are never modified. Returns the page count of the result.
pub fn merge(inputs: &[PathBuf], output: &Path) -> Result<u32, PdfCoreError> {
    pdf_core::merge(inputs, output)
}

/// Extract the pages named by `selection` (e.g. `1-4,8,11-15`) into `output`.
pub fn extract(input: &Path, selection: &str, output: &Path) -> Result<u32, PdfCoreError> {
    let total = pdf_core::page_count(input)?;
    let pages = parse_page_selection(selection, total)?;
    pdf_core::extract_pages(input, &pages, output)
}

/// Remove the pages named by `selection` and write the rest to `output`.
pub fn remove(input: &Path, selection: &str, output: &Path) -> Result<u32, PdfCoreError> {
    let total = pdf_core::page_count(input)?;
    let pages = parse_page_selection(selection, total)?;
    pdf_core::remove_pages(input, &pages, output)
}

/// Rotate selected pages (`None` means all) clockwise by `rotation` degrees.
pub fn rotate(
    input: &Path,
    selection: Option<&str>,
    rotation: i64,
    output: &Path,
) -> Result<u32, PdfCoreError> {
    let total = pdf_core::page_count(input)?;
    let pages = match selection {
        Some(value) if !value.trim().is_empty() => Some(parse_page_selection(value, total)?),
        _ => None,
    };
    pdf_core::rotate_pages(input, pages.as_deref(), rotation, output)
}

/// How Split decides which pages belong together.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitMode {
    /// One output file per page.
    EveryPage,
    /// One output file per contiguous run of the selection.
    Ranges,
}

impl SplitMode {
    pub fn parse(value: &str) -> Result<Self, PdfCoreError> {
        match value {
            "every-page" => Ok(Self::EveryPage),
            "ranges" => Ok(Self::Ranges),
            other => Err(PdfCoreError::Other(format!(
                "unknown split mode \"{other}\""
            ))),
        }
    }
}

/// Split `input` into several files inside `output_dir`.
///
/// File names are `{base_name}_{index:03}.pdf`, made unique against existing
/// files. Returns the generated paths (and never overwrites anything).
pub fn split(
    input: &Path,
    mode: SplitMode,
    selection: Option<&str>,
    output_dir: &Path,
    base_name: &str,
) -> Result<Vec<PathBuf>, PdfCoreError> {
    let total = pdf_core::page_count(input)?;

    let groups: Vec<Vec<u32>> = match mode {
        SplitMode::EveryPage => (1..=total).map(|page| vec![page]).collect(),
        SplitMode::Ranges => {
            let value = selection.unwrap_or("").trim();
            if value.is_empty() {
                return Err(PdfCoreError::Other(
                    "enter at least one page range to split".to_string(),
                ));
            }
            let pages = parse_page_selection(value, total)?;
            group_contiguous(&pages)
        }
    };

    let stem = filesystem::unique_run_stem(output_dir, base_name);
    let outputs: Vec<PathBuf> = (0..groups.len())
        .map(|index| output_dir.join(format!("{stem}_{:03}.pdf", index + 1)))
        .collect();

    pdf_core::split_pdf(input, &groups, &outputs)?;
    Ok(outputs)
}

/// Collapse an ordered page list into contiguous runs: `[1,2,3,8,11,12]` →
/// `[[1,2,3],[8],[11,12]]`.
fn group_contiguous(pages: &[u32]) -> Vec<Vec<u32>> {
    let mut groups: Vec<Vec<u32>> = Vec::new();
    let mut current: Vec<u32> = Vec::new();

    for &page in pages {
        match current.last() {
            Some(&last) if page == last + 1 => current.push(page),
            Some(_) => {
                groups.push(std::mem::take(&mut current));
                current.push(page);
            }
            None => current.push(page),
        }
    }
    if !current.is_empty() {
        groups.push(current);
    }
    groups
}

#[cfg(test)]
mod tests {
    use super::group_contiguous;

    #[test]
    fn groups_contiguous_runs() {
        assert_eq!(
            group_contiguous(&[1, 2, 3, 8, 11, 12]),
            vec![vec![1, 2, 3], vec![8], vec![11, 12]]
        );
        assert_eq!(group_contiguous(&[]), Vec::<Vec<u32>>::new());
        assert_eq!(group_contiguous(&[5]), vec![vec![5]]);
    }
}

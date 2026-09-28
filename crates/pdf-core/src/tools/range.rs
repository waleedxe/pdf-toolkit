use crate::error::{PdfCoreError, PdfCoreResult};

/// Parse a human page selection into a list of 1-based page numbers.
///
/// Accepts inputs such as `1`, `1,4,7`, and `1-4,8,11-15`. Ranges are inclusive.
/// The result preserves the order the user wrote and contains no duplicates.
///
/// `page_count` is used to reject out-of-range selections with a clear error
/// instead of silently clamping them.
pub fn parse_page_selection(input: &str, page_count: u32) -> PdfCoreResult<Vec<u32>> {
    if page_count == 0 {
        return Err(PdfCoreError::Other(
            "the document has no pages to select".to_string(),
        ));
    }

    let mut pages: Vec<u32> = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for raw_part in input.split(',') {
        let part = raw_part.trim();
        if part.is_empty() {
            return Err(PdfCoreError::Other(
                "empty page selection element".to_string(),
            ));
        }

        let (start, end) = match part.split_once('-') {
            Some((start, end)) => {
                let start = parse_page_number(start)?;
                let end = parse_page_number(end)?;
                if start > end {
                    return Err(PdfCoreError::Other(format!(
                        "invalid range {part}: start is greater than end"
                    )));
                }
                (start, end)
            }
            None => {
                let page = parse_page_number(part)?;
                (page, page)
            }
        };

        for page in start..=end {
            if page < 1 || page > page_count {
                return Err(PdfCoreError::PageOutOfRange {
                    page,
                    total: page_count,
                });
            }
            if seen.insert(page) {
                pages.push(page);
            }
        }
    }

    if pages.is_empty() {
        return Err(PdfCoreError::Other(
            "the page selection did not contain any pages".to_string(),
        ));
    }

    Ok(pages)
}

fn parse_page_number(value: &str) -> PdfCoreResult<u32> {
    let trimmed = value.trim();
    let number: u32 = trimmed
        .parse()
        .map_err(|_| PdfCoreError::Other(format!("\"{trimmed}\" is not a valid page number")))?;
    if number == 0 {
        return Err(PdfCoreError::Other("page numbers start at 1".to_string()));
    }
    Ok(number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_single_page() {
        assert_eq!(parse_page_selection("1", 10).unwrap(), vec![1]);
    }

    #[test]
    fn parses_comma_list_preserving_order() {
        assert_eq!(parse_page_selection("1,4,7", 10).unwrap(), vec![1, 4, 7]);
        assert_eq!(parse_page_selection("7,1,4", 10).unwrap(), vec![7, 1, 4]);
    }

    #[test]
    fn parses_ranges_inclusively() {
        assert_eq!(
            parse_page_selection("1-4,8,11-15", 20).unwrap(),
            vec![1, 2, 3, 4, 8, 11, 12, 13, 14, 15]
        );
    }

    #[test]
    fn ignores_whitespace_and_duplicates() {
        assert_eq!(
            parse_page_selection(" 1 - 3 , 3, 2 ", 5).unwrap(),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn rejects_out_of_range_pages() {
        assert!(matches!(
            parse_page_selection("12", 10),
            Err(PdfCoreError::PageOutOfRange {
                page: 12,
                total: 10
            })
        ));
        assert!(matches!(
            parse_page_selection("1-20", 5),
            Err(PdfCoreError::PageOutOfRange { total: 5, .. })
        ));
    }

    #[test]
    fn rejects_zero_and_non_numeric_input() {
        assert!(parse_page_selection("0", 5).is_err());
        assert!(parse_page_selection("abc", 5).is_err());
        assert!(parse_page_selection("", 5).is_err());
        assert!(parse_page_selection("3-1", 5).is_err());
    }
}

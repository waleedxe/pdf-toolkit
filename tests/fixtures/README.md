# Test Fixtures

Legally safe test inputs for PDF Toolkit. Prefer **generating** fixtures programmatically
(see `crates/pdf-core/tests` and `tests/` generators) so large or sensitive-looking files are
never committed.

## Required fixtures (per spec)

| Fixture | Purpose | Status |
| --- | --- | --- |
| One-page PDF | baseline | generated in `pdf-core` tests |
| Multi-page PDF | page count / ranges / reorder | generated in `pdf-core` tests |
| PDF with images | images-to-pdf / pdf-to-images | generated in `pdf-core` and `pdf-service` tests |
| Arabic PDF | Arabic rendering | generated + rendered in `pdf-core`/`pdf-render` text tests |
| Mixed Arabic/English PDF | mixed text | generated in `pdf-core` text tests |
| Password-protected PDF | protect / unlock | generated in `pdf-core` security tests |
| Malformed PDF | safe failure | covered in `pdf-core` and `pdf-render` tests |
| Large PDF (100+ pages) | performance | generate on demand |
| JPG / PNG images | images-to-pdf | generated via the `image` crate in tests |

All fixtures above are generated programmatically at test time; none are committed.

## Rules

- Never commit confidential or non-redistributable documents.
- Keep committed fixtures small; generate large ones on demand.
- Tests must run **real** operations, not only mocks.

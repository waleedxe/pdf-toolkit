# Test Fixtures

Legally safe test inputs for PDF Toolkit. Prefer **generating** fixtures programmatically
(see `crates/pdf-core/tests` and `tests/` generators) so large or sensitive-looking files are
never committed.

## Required fixtures (per spec)

| Fixture | Purpose | Status |
| --- | --- | --- |
| One-page PDF | baseline | generated in `pdf-core` tests |
| Multi-page PDF | page count / ranges / reorder | generated in `pdf-core` tests |
| PDF with images | images-to-pdf / pdf-to-images | planned |
| Arabic PDF | Arabic rendering | planned |
| Mixed Arabic/English PDF | mixed text | planned |
| Password-protected PDF | protect / unlock | planned |
| Malformed PDF | safe failure | covered in `pdf-core` tests |
| Large PDF (100+ pages) | performance | generate on demand |
| JPG / PNG images | images-to-pdf | planned |

## Rules

- Never commit confidential or non-redistributable documents.
- Keep committed fixtures small; generate large ones on demand.
- Tests must run **real** operations, not only mocks.

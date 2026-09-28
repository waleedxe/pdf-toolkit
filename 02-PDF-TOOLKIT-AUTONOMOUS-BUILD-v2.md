# AUTONOMOUS BUILD SPEC v2 — Local-First PDF Desktop Toolkit

## Mission

You are the primary implementation agent for this repository.

Your job is NOT to merely propose a PDF app, create mock screens, or scaffold a project.

Your job is to autonomously build a complete, tested, production-buildable MVP of a local-first Windows PDF toolkit.

Continue until the MVP defined below is complete, or until you hit a genuine external blocker that cannot be solved inside the available development environment.

Normal engineering problems are NOT blockers.

You are expected to:
- investigate libraries
- choose reliable implementations
- install dependencies
- write code
- run tools
- test real PDF files
- fix failures
- package the app
- configure releases

---

# 1. Product Goal

Build a fast, privacy-focused desktop PDF utility suite.

It should offer the convenience of popular online PDF tool websites, but with local processing.

Main differentiators:

- local-first
- offline
- privacy-focused
- Arabic + English from day one
- RTL interface
- modern Windows desktop UX
- drag-and-drop
- modular tool design
- public product quality

The product is intended for public distribution.

Do not use personal branding.

Use a neutral temporary name such as `PDF Toolkit` until branding is finalized.

---

# 2. Product Principles

1. Files stay on the device
2. No mandatory account
3. No mandatory cloud
4. No hidden uploads
5. Offline MVP
6. Fast operations
7. Reliable output
8. Arabic is first-class
9. English is first-class
10. RTL is first-class
11. Modular architecture
12. Large operations must not freeze UI
13. Never implement the PDF specification from scratch
14. Prefer mature engines over clever custom code
15. Do not destroy original files by default

---

# 3. Mandatory Technology Stack

## Desktop
- Tauri 2

## Frontend
- React
- TypeScript
- Vite

## Backend
- Rust

## State
- Zustand

## Internationalization
- i18next
- react-i18next

## UI
Prefer:
- Tailwind CSS

or a similarly lightweight modular styling solution.

Do NOT use Electron.

---

# 4. Autonomous Execution Contract

You must:

- inspect repository state
- initialize if empty
- research/select PDF libraries
- install dependencies
- implement backend adapters
- implement UI
- process real PDFs
- test outputs
- run lint
- run typecheck
- run Rust checks
- run tests
- run production build
- package Windows release
- configure CI
- write documentation

Do NOT stop at:
- architecture proposal
- UI prototype
- fake processing
- buttons without working backend logic
- one completed PDF tool
- partial MVP

Implement every required MVP feature.

---

# 5. Decision Authority

You are authorized to decide:

- exact PDF libraries
- Rust crate choices
- rendering engine
- frontend components
- job architecture
- temporary-file strategy
- thumbnail strategy
- internal APIs
- testing framework
- error model
- output naming behavior

Do not ask the user to choose between technically equivalent options.

Ask the user only for:
- paid SDK credentials
- signing certificates
- final product name/logo
- publishing credentials
- legal/licensing decisions that cannot be deferred

Use placeholders and keep working when possible.

---

# 6. PDF Engine Strategy

Do not build a PDF parser from scratch.

Evaluate mature options.

Potential components may include:

- Rust PDF crates for manipulation
- PDFium or MuPDF for rendering if licensing/distribution is suitable
- qpdf for structural manipulation/security if appropriate
- Ghostscript for advanced compression if licensing/distribution is acceptable
- image libraries for conversion
- fonts/text shaping libraries for Arabic

Important:
Every external engine must be wrapped behind an internal adapter.

Example:

```text
UI
↓
Tauri command
↓
PDF service
↓
Engine adapter
↓
Selected library/binary
```

This allows engines to be replaced later.

Do NOT spread external binary calls throughout the codebase.

---

# 7. Licensing Review

Before bundling any major engine:
- inspect its license
- document redistribution implications
- prefer permissive solutions where feasible
- do not accidentally force an incompatible public distribution model

Create:
`THIRD_PARTY_LICENSES.md`

Document:
- crate/library
- license
- purpose
- whether bundled

Do not stop development solely because final legal review is pending if a safe development configuration exists.

---

# 8. Required MVP Tools

MVP is NOT complete until all required tools below work with real documents.

## Tool 1 — Merge PDF

- multiple PDFs
- drag-and-drop
- reorder input files
- remove inputs
- merge
- output path
- preserve pages

## Tool 2 — Split PDF

Modes:
- every page
- page ranges
- selected ranges

## Tool 3 — Extract Pages

Support input such as:
- `1`
- `1,4,7`
- `1-4,8,11-15`

Create a new PDF.

## Tool 4 — Remove Pages

- page selection
- range input
- output new file

Never overwrite source by default.

## Tool 5 — Rotate Pages

- 90°
- 180°
- 270°
- all pages
- selected pages

## Tool 6 — Reorder Pages

- thumbnail grid
- drag-and-drop reorder
- save result

## Tool 7 — Images to PDF

Input:
- JPG
- JPEG
- PNG
- WebP if stable

Options:
- order
- page size
- fit mode
- margin
- orientation

## Tool 8 — PDF to Images

Output:
- PNG
- JPG

Options:
- DPI
- quality
- selected pages

## Tool 9 — Watermark PDF

Types:
- text
- image

Options:
- opacity
- rotation
- scale
- position
- selected pages

Arabic text watermark MUST render correctly.

## Tool 10 — Page Numbers

Options:
- top/bottom
- left/center/right
- start number
- font size
- page range

## Tool 11 — Protect PDF

- set password
- encrypt document using a standard supported scheme

## Tool 12 — Unlock PDF

Only with a valid user-provided password.

Do NOT implement password cracking, brute forcing, or bypass.

---

# 9. Required Application UX

Home screen:
- tool cards
- clear categories
- recent tasks if implemented

Tool flow:

```text
Select Tool
↓
Drop Files
↓
Configure Options
↓
Process
↓
Progress
↓
Success
↓
Open File / Open Folder
```

Required:
- drag-and-drop
- file picker
- progress indication
- cancel where practical
- success state
- failure state
- actionable errors
- output location

---

# 10. Arabic + English Requirements

This is mandatory, not optional.

## UI

Implement:
- English
- Arabic
- language switcher
- LTR
- RTL

When Arabic is active:
- layout direction changes correctly
- sidebar/navigation behavior feels natural
- arrows/icons are mirrored where appropriate
- forms remain readable

## Paths

Support:
- Arabic filenames
- Arabic folder names
- Unicode paths

## Arabic PDF text

For generated text:
- watermark
- page numbers/labels where relevant
- future annotations

Requirements:
- Arabic shaping
- RTL ordering
- proper font support
- proper font embedding
- no disconnected letters

Use a redistributable Arabic-capable font if needed.

Document its license.

Test mixed:
- Arabic + English
- Arabic + numbers
- English + Arabic

Do not claim Arabic support based only on translated UI.

---

# 11. PDF Preview / Page Thumbnails

Implement thumbnail previews for page-oriented tools.

Required for:
- reorder
- remove
- rotate
- extract

Do not render full-resolution pages unnecessarily.

Use:
- thumbnail caching
- lazy rendering
- virtualization if required

Large PDFs must not freeze the UI.

---

# 12. Job Architecture

All PDF operations must use a consistent job system.

Conceptually:

```text
Input validation
↓
Job creation
↓
Background processing
↓
Progress events
↓
Completion/failure
↓
Cleanup
```

Job states:
- idle
- queued
- running
- completed
- failed
- cancelled

Where feasible:
- cancellation
- progress percentage
- current page/file

Never perform heavy processing on the React UI thread.

---

# 13. Temporary Files

Use an application-specific temporary directory.

Requirements:
- unique task folders
- safe filenames
- cleanup on success
- cleanup on failure when safe
- cleanup stale jobs on startup if appropriate

Do not leak document content unnecessarily.

---

# 14. Privacy Requirements

The MVP must truthfully support:

> Files are processed locally on your device.

Therefore:
- no PDF uploads
- no cloud conversion
- no remote OCR
- no analytics containing filenames
- no document contents in telemetry
- no hidden third-party processing

If crash logging is added:
- redact paths
- redact filenames where possible
- never attach documents

---

# 15. Security Requirements

PDFs are untrusted input.

- validate input types
- validate path handling
- avoid shell string concatenation
- pass binary arguments safely
- do not expose arbitrary shell execution
- sandbox/capabilities where possible
- handle malformed PDFs gracefully
- enforce resource limits where practical
- protect against path traversal
- never execute embedded PDF content

---

# 16. Suggested Repository Architecture

```text
src/
  app/
  components/
    common/
    file-drop/
    progress/
    page-grid/
    page-thumbnail/
  features/
    home/
    settings/
    history/
  tools/
    merge/
    split/
    extract/
    remove/
    rotate/
    reorder/
    images-to-pdf/
    pdf-to-images/
    watermark/
    page-numbers/
    protect/
    unlock/
  i18n/
    en.json
    ar.json
  stores/
  hooks/
  types/
  utils/

src-tauri/
  src/
    commands/
    jobs/
    filesystem/
    pdf/
      core/
      render/
      text/
      security/
      engines/
      tools/
    errors/
    lib.rs

assets/
  fonts/

tests/
fixtures/

.github/
  workflows/
```

---

# 17. Consistent Tool Contract

Every tool should expose a consistent internal model:

- input files
- options schema
- validation
- backend command
- progress
- output files
- errors

Do not create twelve unrelated architectures.

Use shared primitives.

---

# 18. File Safety

Default behavior:
- never overwrite original
- generate new output

Example naming:

```text
document_merged.pdf
document_split_001.pdf
document_rotated.pdf
document_watermarked.pdf
```

If destination exists:
- prompt
or
- generate safe incremented name

Do not silently overwrite.

---

# 19. Performance

Requirements:

- background processing
- lazy thumbnails
- cache previews
- avoid loading entire huge files into frontend memory
- avoid transferring huge binary blobs through frontend unnecessarily
- use native filesystem paths and backend processing

Test:
- 1 page
- 10 pages
- 100+ pages
- large scanned PDFs

Document performance limitations.

---

# 20. Testing Fixtures

Create/generate legally safe test fixtures.

Required:

- one-page PDF
- multi-page PDF
- PDF with images
- Arabic PDF
- mixed Arabic/English PDF
- password-protected PDF
- malformed PDF
- large PDF fixture or generated test
- JPG/PNG images

Never rely only on mocked processing.

Run real operations.

---

# 21. Tool-Level Tests

For every required tool:

- valid input succeeds
- invalid input fails safely
- result exists
- result opens/parses
- page count is correct
- expected transformation occurred
- original remains unchanged

Additional:

Merge:
- correct page order

Split:
- correct output groups

Rotate:
- correct page rotation metadata/rendering

Reorder:
- correct final order

Protect:
- protected file requires password

Unlock:
- correct password works
- wrong password fails

Watermark:
- verify generated file structurally
- manually inspect Arabic rendering

---

# 22. Development Sequence

## Phase 0 — Foundation

- initialize Tauri 2 + React + TypeScript
- configure lint/typecheck
- strict TS
- Rust fmt/clippy
- i18n
- Arabic/English
- RTL/LTR
- shared UI shell
- shared native bridge
- verify dev run
- verify Windows production build

Continue automatically.

## Phase 1 — PDF Core

Research and select engines.

Then implement completely:

1. Merge
2. Split
3. Extract
4. Remove
5. Rotate

Each must work before continuing.

Continue automatically.

## Phase 2 — Rendering and Page UX

- PDF rendering
- thumbnails
- page selection
- drag reorder
- reorder tool

Continue automatically.

## Phase 3 — Conversion

- Images to PDF
- PDF to Images

Continue automatically.

## Phase 4 — Generated Content

- Arabic-capable font strategy
- text shaping
- Watermark
- Page Numbers

Do not mark complete until Arabic text is visually validated.

Continue automatically.

## Phase 5 — Security

- Protect PDF
- Unlock PDF with known password

Continue automatically.

## Phase 6 — Product Polish

- settings
- language persistence
- recent tasks optional
- robust errors
- cancellation
- accessibility
- performance
- privacy messaging

Continue automatically.

## Phase 7 — Release

- Windows installer
- CI
- release workflow
- README
- THIRD_PARTY_LICENSES
- CHANGELOG
- privacy docs

---

# 23. Mandatory Engineering Loop

After each major feature:

1. implement
2. format
3. lint
4. typecheck
5. Rust fmt
6. clippy
7. tests
8. test real PDF fixture
9. production build if relevant
10. fix all critical failures
11. continue

Do not accumulate broken phases.

---

# 24. Windows Distribution

Target:
- Windows 10
- Windows 11

Produce/configure:
- `.exe`
- NSIS installer

If feasible:
- MSI

Prepare:
- app identifier
- version metadata
- placeholder icon
- uninstall
- clean app-data strategy

Document future:
- code signing
- Microsoft Store packaging
- auto updater

Do not block MVP because code signing is absent.

---

# 25. GitHub CI/CD

Create workflows.

## Pull Request

Run:
- install frontend deps
- lint
- typecheck
- tests
- cargo fmt check
- clippy
- cargo test

## Release

On version tag:
- Windows runner
- build Tauri
- produce installer
- upload to GitHub Release

Include any required external engine setup in the workflow.

---

# 26. Documentation

Create:

- `README.md`
- `CHANGELOG.md`
- `CONTRIBUTING.md`
- `PRIVACY.md`
- `THIRD_PARTY_LICENSES.md`
- `BLOCKERS.md` only if needed

README should include:
- product purpose
- local-first privacy
- supported tools
- Arabic/English support
- build instructions
- install instructions
- architecture
- development
- limitations
- roadmap

---

# 27. Future Features — DO NOT BLOCK MVP

Do not build these until MVP is complete:

- OCR
- searchable scanned PDF
- PDF to Markdown
- Word to PDF
- Excel to PDF
- PowerPoint to PDF
- PDF to Office
- AI summarization
- AI translation
- ask PDF
- digital certificate signatures
- cloud sync
- accounts

Architecture should not prevent them.

---

# 28. Future OCR Architecture

Prepare only enough abstraction that OCR can be added later.

Future OCR goals:
- Arabic
- English
- mixed Arabic/English
- searchable PDF
- extracted text

Do not bundle huge OCR language models in MVP.

Future strategy:
- optional downloadable language packs

---

# 29. Repository Size Discipline

Do NOT commit:
- `node_modules`
- Rust build target
- installer binaries
- huge test binaries unnecessarily
- OCR model packs
- generated temporary files

Use GitHub Releases for binaries.

Use small representative fixtures in Git.

Generate larger fixtures programmatically when possible.

---

# 30. Git Discipline

If Git is available, create logical commits.

Examples:

- `chore: bootstrap tauri pdf toolkit`
- `feat: add bilingual rtl shell`
- `feat: implement pdf merge`
- `feat: implement pdf split`
- `feat: add page thumbnail rendering`
- `feat: support arabic watermark text`
- `test: add pdf operation fixtures`
- `ci: build windows release`

Never commit secrets.

---

# 31. Blocker Protocol

When something fails:

Do NOT immediately ask the user.

Instead:

1. inspect logs
2. reproduce
3. research local docs/source
4. try a reasonable alternative
5. simplify if necessary
6. continue independent tasks

Only mark as blocker if external.

Document true blockers in:
`BLOCKERS.md`

Valid examples:
- signing certificate
- Store credentials
- paid SDK key

Invalid examples:
- crate compile error
- PDF library API confusion
- Rust ownership issue
- React state bug
- CSS/RTL bug
- test failure

Those are your job to solve.

---

# 32. Definition of Done

MVP is complete only when:

## Application
- launches successfully
- Arabic UI works
- English UI works
- RTL works
- LTR works
- drag-and-drop works

## PDF Tools
- Merge works
- Split works
- Extract works
- Remove works
- Rotate works
- Reorder works
- Images to PDF works
- PDF to Images works
- Watermark works
- Arabic watermark works
- Page numbers work
- Protect works
- Unlock with valid password works

## Quality
- UI remains responsive
- originals are not overwritten by default
- errors are understandable
- temporary files are cleaned
- local processing is maintained

## Engineering
- lint passes
- typecheck passes
- frontend tests pass
- Rust formatting passes
- clippy passes
- Rust tests pass
- PDF integration tests pass
- production build succeeds
- Windows installer generated or release workflow is correctly configured if packaging is unavailable locally

## Documentation
- README complete
- privacy file complete
- third-party licenses documented
- CI configured

Do NOT claim MVP completion unless these conditions are satisfied.

---

# 33. Final Delivery Report

Only after implementation, report:

- completed tools
- engine/library choices
- licensing notes
- Arabic support status
- test fixtures used
- exact test commands
- test results
- build results
- artifact paths
- installer status
- known limitations
- future roadmap

Do not substitute this report for implementation.

---

# 34. START NOW

Immediately:

1. inspect the repository
2. initialize if empty
3. identify candidate PDF engines
4. choose the safest reliable stack
5. document the engine decision
6. complete Phase 0
7. implement Merge PDF end-to-end
8. test it with a real fixture
9. continue through every phase automatically
10. stop only when Definition of Done is satisfied or a genuine external blocker remains

Do not stop after planning.

Do not wait for permission between phases.

BUILD THE PRODUCT.

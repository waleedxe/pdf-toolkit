# Changelog

All notable changes to this project are documented here.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Project foundation: Tauri 2 + React + TypeScript + Vite + Tailwind CSS.
- Bilingual UI shell with English and Arabic, including LTR/RTL layout switching and
  persisted language preference.
- Shared application shell: sidebar tool navigation, tool cards, settings and privacy panels.
- Engine-agnostic Rust `pdf-core` crate with a first document loader and page counting
  (`lopdf`), plus unit tests for valid, malformed, and missing files.
- **Phase 1 PDF tools (backend + UI): Merge, Split, Extract Pages, Remove Pages, Rotate
  Pages.** Each runs entirely in the Rust core (never the UI thread), preserves the original
  file, and writes safe, non-overwriting output names.
- Shared page operations in `pdf-core`: page extraction/rebuild, page removal, contiguous
  range grouping, and `/Rotate` metadata manipulation, with integration tests.
- Frontend tool framework: typed Tauri bridge, native dialog helpers, drag-and-drop hook,
  shared job state machine (`useToolJob`), drop zone, result/open/reveal panel.
- Placeholder app icon generator (no external dependencies).
- CI workflow for frontend checks and Rust core tests; release workflow for the Windows
  NSIS installer.
- **Phase 2 (rendering):** `pdf-render` adapter using the pure-Rust **hayro** rasterizer
  (Apache-2.0 OR MIT). Lazy, cached page thumbnails (`render_thumbnail`, `pdf_info`) and the
  **Reorder Pages** tool with drag-and-drop.
- **Phase 3 (conversion):** **Images to PDF** (JPG/PNG/WebP, page size, orientation, fit,
  margin) and **PDF to Images** (PNG/JPG, DPI, quality, page range).
- **Phase 4 (generated content):** Arabic-aware text pipeline using **rustybuzz** shaping and
  the bundled **Amiri** font (OFL-1.1) embedded as a Type0/CIDFontType2 face. **Watermark PDF**
  (text + image, opacity, rotation, scale, position, page range) and **Page Numbers**
  (top/bottom, left/center/right, start number, size, range).
- **Phase 5 (security):** **Protect PDF** with AES-128 standard encryption and configurable
  permissions, and **Unlock PDF** using a known password only (never cracking/bypass).
- Split the application services into a Tauri-free `pdf-service` crate for portable testing,
  and added validation tests including an Arabic watermark render check.
- Hardening: all PDF loads now go through one loader that applies a 256 MiB per-stream
  decompression limit (defence against decompression-bomb inputs), plus a 100+ page
  scale test covering merge/split/extract/remove/rotate.

### Fixed

- Repaired files damaged by an interrupted write (zeroed or NUL-padded): rebuilt
  `src/lib/tauri.ts`, `src/lib/paths.ts`, `src/hooks/useFileDrop.ts`, and cleaned
  `src-tauri/src/lib.rs` / `src-tauri/src/pdf/mod.rs`.
- Restored the missing frontend `devDependencies` so lint, typecheck, tests, and builds run.

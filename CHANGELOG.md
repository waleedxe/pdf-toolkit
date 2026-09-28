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

### Fixed

- Repaired files damaged by an interrupted write (zeroed or NUL-padded): rebuilt
  `src/lib/tauri.ts`, `src/lib/paths.ts`, `src/hooks/useFileDrop.ts`, and cleaned
  `src-tauri/src/lib.rs` / `src-tauri/src/pdf/mod.rs`.
- Restored the missing frontend `devDependencies` so lint, typecheck, tests, and builds run.

# Project Memory — PDF Toolkit

> Canonical working memory for this repository. Read this first in any new session.
> Source of truth: `02-PDF-TOOLKIT-AUTONOMOUS-BUILD-v2.md` (the autonomous build spec).

## 1. Mission
Autonomously build a complete, tested, production-buildable **MVP** of a **local-first Windows PDF toolkit**.
Not a prototype, not mock screens, not fake processing. Every listed tool must work on real PDFs.

- Working name: **PDF Toolkit** (placeholder; no personal branding).
- Target OS: Windows 10/11. Distribution: `.exe` + NSIS installer.
- Posture: local-first, offline, privacy-focused, Arabic + English from day one, RTL-first.

## 2. Non-negotiable principles
1. Files stay on the device — no uploads, no cloud conversion, no remote OCR, no filename/document telemetry.
2. Never overwrite the original by default; always produce a new output (safe incremented names on collision).
3. Don't implement the PDF spec from scratch — wrap mature engines behind adapters.
4. Heavy work never runs on the React UI thread; use the shared job system.
5. Arabic is first-class: real text shaping + font embedding, not just translated UI.
6. Large PDFs must not freeze the UI (lazy thumbnails, caching, virtualization).

## 3. Mandatory stack
- Desktop: **Tauri 2** (NOT Electron)
- Frontend: **React + TypeScript + Vite**
- Backend: **Rust**
- State: **Zustand**
- i18n: **i18next + react-i18next**
- Styling: **Tailwind CSS** (or similarly lightweight)

## 4. Engine architecture (adapter rule)
```
UI → Tauri command → PDF service → engine adapter → selected library/binary
```
- Never scatter direct binary/library calls through the codebase.
- Candidate engines (decide + document in `BLOCKERS.md`/docs):
  - Rust PDF crates for manipulation (merge/split/rotate/extract)
  - PDFium or MuPDF for rendering/thumbnails (check license/distribution suitability)
  - qpdf for structural manipulation / security if appropriate
  - Ghostscript for advanced compression if licensing/distribution is acceptable
  - image crates for conversion; Arabic font + shaping lib for generated text
- Record every engine: crate/library, license, purpose, bundled? → `THIRD_PARTY_LICENSES.md`.

## 5. The 12 MVP tools (all must work)
1. Merge PDF — multi-input, drag-drop, reorder, remove, preserve pages.
2. Split PDF — every page / page ranges / selected ranges.
3. Extract Pages — ranges like `1`, `1,4,7`, `1-4,8,11-15` → new PDF.
4. Remove Pages — selection + range input → new file.
5. Rotate Pages — 90/180/270, all or selected.
6. Reorder Pages — thumbnail grid + drag-drop → save.
7. Images to PDF — JPG/JPEG/PNG/WebP(if stable); order, page size, fit, margin, orientation.
8. PDF to Images — PNG/JPG; DPI, quality, selected pages.
9. Watermark PDF — text + image; opacity, rotation, scale, position, selected pages. **Arabic must render correctly.**
10. Page Numbers — top/bottom, left/center/right, start number, font size, page range.
11. Protect PDF — set password, standard supported encryption.
12. Unlock PDF — valid user-supplied password only. No cracking/brute force/bypass.

## 6. Shared systems to build once (not per-tool)
- **Tool contract**: input files, options schema, validation, backend command, progress, output files, errors.
- **Job architecture**: idle → queued → running → completed/failed/cancelled; progress %, current page/file, cancel where practical.
- **Temp files**: app-specific temp dir, unique task folders, safe filenames, cleanup on success/failure, stale-job cleanup on startup.
- **Thumbnails/preview**: required for reorder, remove, rotate, extract; caching + lazy render + virtualization.
- **Error model**: actionable, understandable error states.
- **Output naming**: `document_merged.pdf`, `document_split_001.pdf`, `document_rotated.pdf`, `document_watermarked.pdf`, etc.

## 7. UX flow
Home (tool cards, categories) → Select Tool → Drop Files → Configure Options → Process → Progress → Success → Open File / Open Folder.
Drag-and-drop + file picker + progress + cancel + success/failure states required.

## 8. Arabic / i18n requirements
- EN + AR UI with language switcher; LTR + RTL; mirrored arrows/icons where appropriate; readable forms.
- Support Arabic filenames, folder names, Unicode paths.
- Generated text (watermark, page-number labels) needs: Arabic shaping, RTL ordering, correct font + embedding, no disconnected letters.
- Use a redistributable Arabic-capable font; document its license.
- Test mixed: Arabic+English, Arabic+numbers, English+Arabic.

## 9. Security
PDFs = untrusted input. Validate input types + path handling, avoid shell string concatenation, pass binary args safely, no arbitrary shell exec, sandbox/capabilities where possible, handle malformed PDFs, enforce resource limits, protect against path traversal, never execute embedded PDF content.

## 10. Suggested repo layout
```
src/{app,components/{common,file-drop,progress,page-grid,page-thumbnail},features/{home,settings,history},tools/{merge,split,extract,remove,rotate,reorder,images-to-pdf,pdf-to-images,watermark,page-numbers,protect,unlock},i18n/{en.json,ar.json},stores,hooks,types,utils}
src-tauri/src/{commands,jobs,filesystem,pdf/{core,render,text,security,engines,tools},errors,lib.rs}
assets/fonts/  tests/fixtures/  .github/workflows/
```

## 11. Development phases
- **Phase 0 — Foundation**: init Tauri2+React+TS, strict TS, lint/typecheck, Rust fmt/clippy, i18n EN/AR, RTL/LTR shell, native bridge, dev run + Windows prod build.
- **Phase 1 — PDF Core**: pick engines, then Merge, Split, Extract, Remove, Rotate (each working before next).
- **Phase 2 — Rendering/Page UX**: rendering, thumbnails, page selection, drag reorder, Reorder tool.
- **Phase 3 — Conversion**: Images→PDF, PDF→Images.
- **Phase 4 — Generated Content**: Arabic font strategy + shaping, Watermark, Page Numbers (validated visually).
- **Phase 5 — Security**: Protect, Unlock (known password).
- **Phase 6 — Polish**: settings, language persistence, recent tasks (optional), errors, cancel, a11y, perf, privacy messaging.
- **Phase 7 — Release**: Windows installer, CI, release workflow, README, THIRD_PARTY_LICENSES, CHANGELOG, privacy docs.

## 12. Engineering loop (after every major feature)
implement → format → lint → typecheck → rustfmt → clippy → tests → real-PDF fixture test → prod build if relevant → fix critical failures → continue. Never accumulate broken phases.

## 13. Testing fixtures (legally safe; generate programmatically when large)
one-page PDF · multi-page PDF · PDF with images · Arabic PDF · mixed Arabic/English PDF · password-protected PDF · malformed PDF · large PDF · JPG/PNG images.
Never rely only on mocks — run real operations.
Per-tool tests: valid succeeds, invalid fails safely, output exists/parses, correct page count, expected transformation, original unchanged. Plus merge order, split groups, rotate metadata, reorder order, protect requires password, unlock correct/incorrect password, Arabic watermark inspected.

## 14. Docs to produce
`README.md` (purpose, local-first privacy, tools, AR/EN support, build+install, architecture, dev, limitations, roadmap), `CHANGELOG.md`, `CONTRIBUTING.md`, `PRIVACY.md`, `THIRD_PARTY_LICENSES.md`, `BLOCKERS.md` (only if needed).

## 15. CI/CD
- PR: install frontend deps, lint, typecheck, tests, `cargo fmt --check`, clippy, `cargo test`.
- Release on version tag: Windows runner, build Tauri, produce installer, upload to GitHub Release.

## 16. Repository / git discipline
Don't commit `node_modules`, Rust `target`, installer binaries, huge fixtures, OCR packs, temp files — use GitHub Releases for binaries. Logical commits (e.g. `chore: bootstrap tauri pdf toolkit`, `feat: implement pdf merge`, ...). Never commit secrets.

## 17. Blocker protocol
Don't ask the user immediately. Inspect logs → reproduce → research → try alternative → simplify → continue other tasks. Only external blockers (signing cert, Store creds, paid SDK key, publish creds, final name/logo, undefrable legal decisions) count. Document in `BLOCKERS.md`.

## 18. Do NOT build (post-MVP)
OCR, searchable scanned PDF, PDF→Markdown, Word/Excel/PPT→PDF, PDF→Office, AI summarize/translate, ask-PDF, certificate signatures, cloud sync, accounts. Just keep architecture from preventing them. (OCR: prepare abstraction only; no huge language models in MVP — optional downloadable language packs later.)

## 19. Current environment notes (as of session 1)
- Repo initially contained **only** the spec file; **not a git repo**. Git was initialized on branch `main`.
- Linux/WSL environment (Windows path `/mnt/c/...`) — **filesystem is slow**, installs take a long time. Ubuntu 20.04.
- Rust was installed via rustup (cargo present); toolchain had to be reinstalled after an interrupted download.
- **No Tauri Linux system deps** (webkit2gtk 4.1 is unavailable on Ubuntu 20.04). Consequence: the `pdf-toolkit` Tauri app crate is **checked/built on the Windows CI runner**, not locally.
- Windows installer packaging does not happen locally; it is produced by the GitHub Actions release workflow. This is explicitly allowed by the spec.

## 19a. Architecture decision — split Rust workspace (deviation, intentional)
Per the spec the PDF modules live under `src-tauri/src/pdf/`. To keep the heavy PDF
logic testable on Linux/CI **without** Tauri GUI dependencies, the logic lives in a
standalone crate:

- `crates/pdf-core/` — engine-agnostic PDF library (all real manipulation logic, unit-tested everywhere). Currently uses `lopdf`.
- `src-tauri/src/pdf/mod.rs` — thin service layer that calls `pdf-core`; the only place in the app allowed to touch an engine.
- Root `Cargo.toml` is a workspace with members `src-tauri` and `crates/pdf-core`.

This satisfies the adapter rule (engines wrapped behind stable APIs) and the modular-architecture
principle, while making the engineering loop runnable locally.

## 19b. Tech choices
- Frontend: React 19 + TypeScript (strict, including `exactOptionalPropertyTypes`), Vite, Tailwind CSS v4 (`@tailwindcss/vite`), Zustand, i18next, `lucide-react`, Vitest + Testing Library.
- Rust: `lopdf` (MIT) for document manipulation; Tauri 2 + `tauri-plugin-dialog` + `tauri-plugin-opener`.
- Docs/CI written up-front: README, PRIVACY, CONTRIBUTING, THIRD_PARTY_LICENSES, CHANGELOG, CI + release workflows.

## 20. Status log
- **Session 1**:
  1. Read full spec; created this memory file.
  2. `git init` on `main`.
  3. Scaffolded frontend: Vite + React + TS strict, Tailwind v4, i18n (en/ar) with RTL switching + persisted language, app shell (sidebar, tool cards, settings/privacy), 12-tool catalog, Vitest tests.
  4. Scaffolded Rust: root workspace, `crates/pdf-core` (document loader + page count, error model, tests), `src-tauri` Tauri 2 app with `app_info` command, plugins, capabilities, NSIS config.
  5. Wrote docs + CI/release workflows + placeholder icon generator.
  6. **Pending:** finish npm/Rust toolchain installs, then run lint/typecheck/test/build and `cargo fmt/clippy/test`. Then Phase 1 (Merge end-to-end).

- **Session 2**:
  1. **Found on-disk corruption** from the interrupted Session 1 run (and **no git commits** existed):
     `src/lib/tauri.ts`, `src/lib/paths.ts`, `src/hooks/useFileDrop.ts` were 100% zero bytes;
     `src-tauri/src/lib.rs` and `src-tauri/src/pdf/mod.rs` had trailing NUL padding.
     Rebuilt the three TS files (typed Tauri bridge, cross-platform path helpers, Tauri-native
     drag-drop hook) and cleaned the two Rust files. Verified repo-wide: zero NUL bytes.
  2. Restored the **missing frontend `devDependencies`** (they were never declared, so the
     lockfile was production-only): vite 8, @vitejs/plugin-react, @tailwindcss/vite, tailwindcss,
     typescript, vitest, jsdom, testing-library, eslint + plugins, prettier, @types/*.
  3. Installed Rust toolchain (rustup, minimal + rustfmt/clippy) for the dev user.
  4. Completed the Tauri service layer: `filesystem` module (non-overwriting output names),
     `pdf` services (merge/extract/remove/rotate/split with contiguous-range grouping),
     command surface, and re-registered commands in `lib.rs`.
  5. **Phase 1 PDF core** in `crates/pdf-core/src/tools/pages.rs`: shared page rebuild,
     extract, remove, split, rotate (`/Rotate` metadata). 23 Rust tests pass; clippy clean.
  6. **Phase 1 UI**: shared framework (typed bridge, dialog helpers, drop zone, `useToolJob`,
     result panel) + Merge, Split, Extract, Remove, Rotate tools, en/ar translations.
  7. Fixed two pre-existing config issues: `exactOptionalPropertyTypes` in `vite.config.ts`,
     and Vite 8 (rolldown) no longer bundling `esbuild` (dropped `minify: "esbuild"`).
  8. **Gate green**: `format:check`, `lint`, `typecheck`, 12 frontend tests, `vite build`;
     `cargo fmt`, `clippy -D warnings`, 23 `pdf-core` tests.
  9. **Next:** Phase 2 — renderer + thumbnails + Reorder, then Phase 3+.

- **Session 3** (continued autonomously through all remaining phases):
  1. **Renderer decision:** chose **hayro** (pure Rust, Apache-2.0 OR MIT) over PDFium/MuPDF —
     no native library to ship, self-contained installer. Added crate `crates/pdf-render`.
  2. **Phase 2:** lazy + cached page thumbnails (`render_thumbnail` with in-memory cache,
     `pdf_info`), `PageThumbnail` with IntersectionObserver, and the **Reorder** tool with
     drag-and-drop.
  3. **Phase 3:** **Images to PDF** (JPG/PNG/WebP; page size/orientation/fit/margin) and
     **PDF to Images** (PNG/JPG, DPI, quality, range). Extracted the Tauri-free
     `crates/pdf-service` crate so all orchestration is unit-testable locally.
  4. **Phase 4:** Arabic text pipeline — **rustybuzz** shaping + **Amiri** font (OFL-1.1)
     embedded as Type0/CIDFontType2. **Watermark** (text + image) and **Page Numbers**.
     Validated by a render test proving the Arabic watermark produces visible glyphs.
  5. **Phase 5:** **Protect** (AES-128, V4 security handler, permissions) and **Unlock**
     (known password only). Added file `/ID` when missing so minimal PDFs can be encrypted.
  6. **Phase 6:** settings + persisted language, actionable errors, lazy rendering,
     privacy messaging, a11y labels (existing polish retained; recent-tasks left optional).
  7. **Phase 7:** updated README, THIRD_PARTY_LICENSES (all permissive), CHANGELOG, fixtures
     docs; CI now lints/tests all three engine crates; added `@tauri-apps/cli`.
  8. **All 12 MVP tools are implemented and wired end-to-end.** Local gates green
     (frontend lint/typecheck/tests/build; `cargo fmt`/`clippy -D warnings`/tests across
     `pdf-core`, `pdf-render`, `pdf-service`). The Tauri app crate remains checked on the
     Windows CI runner (no WebKitGTK on Ubuntu 20.04).

- **Session 4** (Windows validation performed *on the Linux box* by cross-compiling):
  1. Discovered the app crate had **never compiled**: `commands/mod.rs` used `pdf::…`
     without `use crate::pdf;`. Fixed.
  2. Set up a user-space Windows GNU toolchain (no root): `rustup target add
     x86_64-pc-windows-gnu`, then extracted `binutils`/`gcc`/`nsis` `.deb`s with
     `apt-get download` + `dpkg-deb -x`; added a `gcc` wrapper (`-B` + `--sysroot`) and a
     `makensis` wrapper forcing `NSISDIR`.
  3. Overlaid **NSIS 3.11** `Include`/`Stubs`/`Plugins` (Ubuntu 3.05's `Win/COM.nsh` lacks
     `ICustomDestinationList`, which Tauri's installer uses) and added the Tauri
     `ApplicationID.dll` plugin.
  4. Fixed two more real issues: Tauri plugin npm/crate version drift, and mobile
     `staticlib`/`cdylib` targets overflowing the GNU linker's export ordinals (desktop-only
     now: `crate-type = ["rlib"]`).
  5. **Result:** `cargo check -p pdf-toolkit --target x86_64-pc-windows-gnu` passes, and the
     GNU cross-build *links* an `.exe` + NSIS installer. **However, that exe does not run on
     Windows** — GNU `ld` mangles a Rust-generated import (a "GetTimeZoneInformationForYear"
     entry-point error at launch; the logs warned "corrupt .drectve"). So the GNU cross route
     validates *compilation* only, not the runtime artifact.
  6. **Conclusion:** the supported Windows build is **MSVC** — natively on Windows, or via the
     GitHub Actions `release.yml` (windows-latest). Do not ship the GNU cross-built installer.
  7. **Still open:** runtime launch and the official MSVC installer (Windows/CI); GitHub push.

- **Session 5 — DELIVERED (project closed)**:
  1. Created the **private** GitHub repo `waleedxe/pdf-toolkit` and pushed `main`.
  2. Fixed CI's flaky `pdf-render` test (parallel tests shared one temp fixture →
     per-test directory now) and corrected the release workflow's artifact paths (a Cargo
     workspace outputs under the workspace-root `target/`, not `src-tauri/target/`).
  3. Tagged **`v0.1.0`**; the **Windows (MSVC) release workflow passed** and produced:
     `PDF.Toolkit_0.1.0_x64-setup.exe` (NSIS installer) + `PDF-Toolkit-portable-windows-x64.zip`.
     The release was published; CI is green (frontend + Windows app check + Rust tests).
  4. Downloaded both artifacts to the user's Windows Desktop; the user **ran the app and
     confirmed all tools work** ("wow"). End-to-end verified on real Windows.
  5. Added `docs/PROJECT-AR.md` (Arabic guide: project, stack, architecture, how to build the
     EXE, and how to add features).
  6. **Remaining (user-side, optional):** revoke the temporary GitHub token; flip the repo to
     public when ready; optionally add code signing.

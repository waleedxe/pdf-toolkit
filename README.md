# PDF Toolkit

A fast, **local-first** desktop PDF toolkit for Windows. It offers the convenience of
popular online PDF websites while processing everything **on your device**.

- 🔒 **Private by design** — files never leave your computer. No uploads, no cloud
  conversion, no accounts.
- 🌍 **Arabic + English from day one** — first-class translations plus true right-to-left
  (RTL) layout, and Arabic-capable text rendering for generated PDF content.
- ⚡ **Fast and responsive** — heavy work runs in the background; large PDFs never freeze
  the interface.
- 🧩 **Modular tools** — one consistent tool contract and job system across all tools.

> **Status:** early development (Phase 0 — foundation). See [Roadmap](#roadmap).

## Supported tools (MVP target)

| Category | Tools |
| --- | --- |
| **Organize** | Merge PDF · Split PDF · Extract Pages · Remove Pages · Reorder Pages |
| **Convert** | Images to PDF · PDF to Images |
| **Edit** | Rotate Pages · Watermark PDF · Page Numbers |
| **Security** | Protect PDF · Unlock PDF (with a password you know) |

Unlock **never** implements password cracking, brute forcing, or bypass. It only removes
protection when you provide the correct password.

## Privacy

PDF Toolkit is built so that the statement *"Files are processed locally on your device"*
is literally true:

- no PDF uploads
- no cloud conversion
- no remote OCR
- no analytics containing file names or document contents
- no hidden third-party processing

See [PRIVACY.md](PRIVACY.md) for details.

## Arabic support

Arabic is a first-class citizen, not an afterthought:

- the entire UI is translated (English / Arabic) with a language switcher
- the layout switches between LTR and RTL, mirroring navigation and icons
- Arabic file names, folder names, and Unicode paths are supported
- generated text (watermarks, page-number labels) uses Arabic shaping, RTL ordering, and
  proper font embedding — no disconnected letters

Arabic-friendliness is validated with real fixtures (Arabic and mixed Arabic/English PDFs),
not just translated strings.

## Architecture

```text
UI (React + TypeScript)
  ↓ Tauri command
Tauri layer (src-tauri)
  ↓ service
PDF core (crates/pdf-core) — engine-agnostic
  ↓ adapter
PDF engine (lopdf, and a renderer for thumbnails)
```

- Every external engine is wrapped behind an adapter, so engines can be replaced later.
- The PDF core is a standalone Rust library, which makes it unit-testable on any platform
  without the GUI dependencies.

### Repository layout

```text
src/                      React + TypeScript frontend
  app/                    application shell
  components/             shared UI (common, file-drop, progress, page-grid, ...)
  features/               home, settings, history
  tools/                  one folder per PDF tool
  i18n/                   en.json, ar.json, direction helpers
  stores/                 Zustand stores
crates/pdf-core/          engine-agnostic Rust PDF library
src-tauri/                Tauri 2 application (Rust)
assets/fonts/             redistributable fonts (incl. Arabic)
tests/fixtures/           legally safe test fixtures
.github/workflows/        CI + release
```

## Build from source

### Prerequisites

- [Node.js](https://nodejs.org/) 20+ (22 recommended) and npm
- [Rust](https://rustup.rs/) stable
- [Tauri 2 prerequisites](https://tauri.app/start/prerequisites/) for your OS (on Windows:
  WebView2, which ships with Windows 10/11; build tools for MSVC)

### Commands

```bash
npm install            # install frontend dependencies
npm run dev            # run the frontend in the browser (Vite)
npm run tauri dev      # run the full desktop app
npm run typecheck      # TypeScript
npm run lint           # ESLint
npm run test           # frontend tests (Vitest)

cargo test -p pdf-core --all-targets   # Rust PDF core tests
cargo fmt --all -- --check             # Rust formatting
cargo clippy -p pdf-core --all-targets -- -D warnings
```

### Production build (Windows)

```bash
npm run tauri build
```

This produces an executable and an NSIS installer under
`src-tauri/target/release/bundle/`. Code signing is not configured yet; see
[BLOCKERS.md](BLOCKERS.md) if it exists.

## Install (end users)

Download the latest NSIS installer (`PDF Toolkit_x.y.z_x64-setup.exe`) from the
[Releases](../../releases) page and run it. No account, no internet connection required.

## Development

- Engineering loop: implement → format → lint → typecheck → `rustfmt` → `clippy` → tests →
  real-PDF fixture test → production build.
- Commits are logical and scoped (`feat: implement pdf merge`, `test: add pdf fixtures`).
- See [CONTRIBUTING.md](CONTRIBUTING.md).

## Limitations

- Windows 10/11 is the primary target.
- OCR, Office conversion, digital signatures, and cloud features are intentionally **out of
  scope** for the MVP.
- Code signing and auto-updates are planned but not yet configured.

## Roadmap

- **Phase 0** Foundation (Tauri 2 + React + TS, i18n, RTL shell) ← *current*
- **Phase 1** PDF core: Merge, Split, Extract, Remove, Rotate
- **Phase 2** Rendering, thumbnails, Reorder
- **Phase 3** Images to PDF, PDF to Images
- **Phase 4** Watermark (Arabic-validated), Page Numbers
- **Phase 5** Protect, Unlock
- **Phase 6** Polish: settings, history, errors, cancellation, accessibility, performance
- **Phase 7** Release: installer, CI, docs

Post-MVP ideas (OCR, PDF↔Office, AI features) are planned for later and the architecture is
designed not to prevent them.

## License

To be finalized before public release. See [THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md)
for third-party components.

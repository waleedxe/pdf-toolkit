# PDF Toolkit

A fast, **local-first** desktop PDF toolkit for Windows. It offers the convenience of
popular online PDF websites while processing everything **on your device**.

- 🔒 **Private by design** — files never leave your computer. No uploads, no cloud
  conversion, no accounts.
- 🌍 **Arabic + English from day one** — first-class translations, true right-to-left (RTL)
  layout, and genuine Arabic text support in generated PDF content (shaping + font embedding).
- ⚡ **Fast and responsive** — heavy work runs off the UI thread; page thumbnails render lazily
  so large PDFs never freeze the interface.
- 🧩 **Modular tools** — one consistent tool contract and engine-adapter architecture.

> **Status:** MVP complete. All twelve tools work on real documents.

## Download

Get the latest build from the [**Releases**](../../releases/latest) page:

- **Installer (`.exe`)** — recommended: run the setup wizard (Next → Next → Install) for a
  Start-menu shortcut and uninstaller.
- **Portable (`.zip`)** — extract it and run `pdf-toolkit.exe`; nothing is installed.

Builds are **unsigned**, so Windows SmartScreen may warn on first run: click **More info →
Run anyway**. No account is needed and no internet connection is required at runtime.

## Supported tools

| Category | Tools |
| --- | --- |
| **Organize** | Merge PDF · Split PDF · Extract Pages · Remove Pages · Reorder Pages |
| **Convert** | Images to PDF (JPG/PNG/WebP) · PDF to Images (PNG/JPG, DPI + quality) |
| **Edit** | Rotate Pages · Watermark PDF (text + image, Arabic-aware) · Page Numbers |
| **Security** | Protect PDF (AES-128 password) · Unlock PDF (with a password you know) |

Unlock **never** implements password cracking, brute forcing, or bypass. It only removes
protection when you provide the correct password.

### Notes on behaviour

- Originals are **never overwritten**; every tool writes a new file and safe incrementing
  names are used on collision.
- Page-oriented tools accept selections like `1`, `1,4,7`, and `1-4,8,11-15`.
- Reorder, Remove, Rotate, and Extract use lazily rendered page thumbnails.
- Output is produced by the Rust backend; the webview never receives whole documents.

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

- the entire UI is translated (English / Arabic) with a language switcher and persisted choice
- the layout switches between LTR and RTL, mirroring navigation and icons
- Arabic file names, folder names, and Unicode paths are supported
- generated text (watermarks, page-number labels) is shaped with **rustybuzz** (contextual
  forms, RTL ordering) and embedded as a **Type0/CIDFontType2** font using the bundled
  **Amiri** face — no disconnected letters, no dependency on system fonts
- mixed Arabic + English + digits is handled in one pass

Arabic rendering is validated by a test that rasterizes a real watermarked PDF and checks
that the glyphs actually appear.

## Architecture

```text
UI (React + TypeScript)
  ↓ Tauri command
src-tauri          — thin command surface + in-memory thumbnail cache
  ↓
crates/pdf-service — application services: output naming, tool orchestration
  ↓
crates/pdf-core    — manipulation + text (lopdf, rustybuzz, image)
crates/pdf-render  — rasterization (hayro)
```

- External engines are wrapped behind adapters, so a library can be replaced without touching
  the UI or command layer.
- `pdf-service` has no Tauri dependency, so the tool orchestration is fully unit-testable on
  any platform.
- The Tauri application crate is built/checked on the Windows CI runner (see Limitations).

### Repository layout

```text
src/                      React + TypeScript frontend
  app/ components/ features/ tools/ i18n/ stores/ hooks/ types/ lib/
crates/pdf-core/          engine-agnostic PDF library (manipulation, text, security)
crates/pdf-render/        PDF rasterization adapter (hayro)
crates/pdf-service/       application services (naming + orchestration)
src-tauri/                Tauri 2 application (Rust)
assets/fonts/             bundled Amiri font (OFL-1.1) + license
tests/fixtures/           legally safe test fixtures
.github/workflows/        CI + release
```

## Build from source

### Prerequisites

- [Node.js](https://nodejs.org/) 20+ (22 recommended) and npm
- [Rust](https://rustup.rs/) 1.92+ (stable)
- [Tauri 2 prerequisites](https://tauri.app/start/prerequisites/) for your OS (on Windows:
  WebView2, which ships with Windows 10/11; MSVC build tools)

### Commands

```bash
npm install            # install frontend dependencies
npm run dev            # run the frontend in the browser (Vite)
npm run tauri dev      # run the full desktop app
npm run typecheck      # TypeScript
npm run lint           # ESLint
npm run test           # frontend tests (Vitest)

cargo test -p pdf-core -p pdf-render -p pdf-service --all-targets
cargo fmt --all -- --check
cargo clippy -p pdf-core -p pdf-render -p pdf-service --all-targets -- -D warnings
```

### Production build (Windows)

```bash
npm run tauri build
```

This produces an executable and an NSIS installer under `src-tauri/target/release/bundle/`.

## Install (end users)

Two ways, both from the [Releases](../../releases) page (no account, no internet needed at
runtime):

- **Installer (recommended):** download `PDF Toolkit_x.y.z_x64-setup.exe` and run it — a normal
  Next → Next → Install wizard, with a Start-menu shortcut and uninstaller.
- **Portable:** download `PDF-Toolkit-portable-windows-x64.zip`, extract it, and run
  `pdf-toolkit.exe`. Nothing is installed.

The app is unsigned for now, so Windows SmartScreen may warn once: **More info → Run anyway**.
WebView2 is required and already ships with Windows 10/11.

## Limitations

- Windows 10/11 is the primary target.
- The Tauri application crate cannot be compiled on the Linux development machine used here
  (Ubuntu 20.04 lacks WebKitGTK 4.1); it is checked on the Windows CI runner.
- Code signing and auto-updates are not configured yet; installers are published as-is.
- OCR, Office conversion, and cloud features are intentionally **out of scope** for the MVP.

## Roadmap

- ✅ **MVP** — foundation, PDF core, rendering/thumbnails, conversion, generated content
  (Arabic-validated), security, polish, release tooling.
- Post-MVP (architecture keeps these possible): OCR with optional downloadable language packs,
  PDF↔Office conversion, and AI features.

## License

Released under the [MIT License](LICENSE). See [THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md)
for third-party components; all bundled engines use permissive licenses.

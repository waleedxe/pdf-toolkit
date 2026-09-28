# Third-Party Licenses

PDF Toolkit bundles or depends on the third-party components below. This file must be kept
up to date whenever an engine, crate, or asset is added.

> **Status:** foundation phase. This list will grow as PDF engines and fonts are added.

## Rust crates

| Component | License | Purpose | Bundled in build |
| --- | --- | --- | --- |
| [`lopdf`](https://crates.io/crates/lopdf) | MIT | PDF parsing and document manipulation (merge, split, extract, remove, rotate, reorder) | Yes (statically linked) |
| [`thiserror`](https://crates.io/crates/thiserror) | MIT OR Apache-2.0 | Ergonomic error types | Yes |
| [`serde`](https://crates.io/crates/serde) | MIT OR Apache-2.0 | Serialization for the Tauri bridge | Yes |
| [`tauri`](https://crates.io/crates/tauri) | MIT OR Apache-2.0 | Desktop application framework | Yes |
| [`tauri-plugin-dialog`](https://crates.io/crates/tauri-plugin-dialog) | MIT OR Apache-2.0 | Native file dialogs | Yes |
| [`tauri-plugin-opener`](https://crates.io/crates/tauri-plugin-opener) | MIT OR Apache-2.0 | Open files/folders in the OS | Yes |

## JavaScript packages

| Component | License | Purpose | Bundled in build |
| --- | --- | --- | --- |
| [`react`](https://www.npmjs.com/package/react) / `react-dom` | MIT | UI library | Yes (bundled JS) |
| [`zustand`](https://www.npmjs.com/package/zustand) | MIT | State management | Yes |
| [`i18next`](https://www.npmjs.com/package/i18next) / `react-i18next` | MIT | Internationalization | Yes |
| [`lucide-react`](https://www.npmjs.com/package/lucide-react) | ISC | Icons | Yes |
| [`@tauri-apps/api`](https://www.npmjs.com/package/@tauri-apps/api) and plugins | MIT OR Apache-2.0 | Frontend bindings to the Tauri backend | Yes |
| [`tailwindcss`](https://www.npmjs.com/package/tailwindcss) | MIT | Styling (build-time) | No (compiled CSS only) |
| [`vite`](https://www.npmjs.com/package/vite) | MIT | Build tool (development) | No |

## Fonts

| Component | License | Purpose | Bundled in build |
| --- | --- | --- | --- |
| _To be added_ | — | Arabic-capable font for generated PDF text | Planned |

When a redistributable Arabic font is added (for watermarks and page numbers), record its
name, license, and whether it is embedded in generated PDFs here.

## Rendering engine (planned)

A PDF rendering engine will be selected for thumbnails and PDF-to-images. Its license and
redistribution implications will be documented here before it is bundled. Permissively
licensed options are strongly preferred to keep public distribution unencumbered.

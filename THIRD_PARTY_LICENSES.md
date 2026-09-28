# Third-Party Licenses

PDF Toolkit bundles or depends on the third-party components below. This file is kept up to
date whenever an engine, crate, package, or asset is added.

All runtime components use permissive licenses (MIT / Apache-2.0 / BSD / ISC / OFL), so the
application can be distributed publicly without copyleft obligations. No GPL/AGPL engine is
bundled.

## Rust crates

| Component | License | Purpose | Bundled in build |
| --- | --- | --- | --- |
| [`lopdf`](https://crates.io/crates/lopdf) | MIT | PDF parsing, manipulation (merge, split, extract, remove, rotate, reorder), images-to-PDF, encryption (protect/unlock), content editing | Yes (statically linked) |
| [`hayro`](https://crates.io/crates/hayro) | Apache-2.0 OR MIT | Pure-Rust PDF rasterizer: thumbnails and PDF-to-images. No native libraries required | Yes (statically linked) |
| [`vello_cpu`](https://crates.io/crates/vello_cpu) | Apache-2.0 OR MIT | CPU renderer used by hayro | Yes (transitive) |
| [`image`](https://crates.io/crates/image) | MIT OR Apache-2.0 | Image decoding (JPG/PNG/WebP) and JPEG encoding | Yes |
| [`flate2`](https://crates.io/crates/flate2) | MIT OR Apache-2.0 | Flate/zlib compression for embedded image and font streams | Yes |
| [`rustybuzz`](https://crates.io/crates/rustybuzz) | MIT | OpenType text shaping (Arabic contextual forms, RTL ordering) | Yes |
| [`ttf-parser`](https://crates.io/crates/ttf-parser) | MIT OR Apache-2.0 | Reading font metrics for embedding | Yes |
| [`base64`](https://crates.io/crates/base64) | MIT OR Apache-2.0 | Encoding rendered thumbnails as data URLs for the webview | Yes |
| [`thiserror`](https://crates.io/crates/thiserror) | MIT OR Apache-2.0 | Ergonomic error types | Yes |
| [`serde`](https://crates.io/crates/serde) / `serde_json` | MIT OR Apache-2.0 | Serialization for the Tauri bridge | Yes |
| [`tauri`](https://crates.io/crates/tauri) | MIT OR Apache-2.0 | Desktop application framework | Yes |
| [`tauri-plugin-dialog`](https://crates.io/crates/tauri-plugin-dialog) | MIT OR Apache-2.0 | Native file/folder dialogs | Yes |
| [`tauri-plugin-opener`](https://crates.io/crates/tauri-plugin-opener) | MIT OR Apache-2.0 | Open files/folders in the OS | Yes |

## JavaScript packages

| Component | License | Purpose | Bundled in build |
| --- | --- | --- | --- |
| [`react`](https://www.npmjs.com/package/react) / `react-dom` | MIT | UI library | Yes (bundled JS) |
| [`zustand`](https://www.npmjs.com/package/zustand) | MIT | State management | Yes |
| [`i18next`](https://www.npmjs.com/package/i18next) / `react-i18next` | MIT | Internationalization | Yes |
| [`lucide-react`](https://www.npmjs.com/package/lucide-react) | ISC | Icons | Yes |
| [`@tauri-apps/api`](https://www.npmjs.com/package/@tauri-apps/api) and plugins | MIT OR Apache-2.0 | Frontend bindings to the Tauri backend | Yes |
| [`tailwindcss`](https://www.npmjs.com/package/tailwindcss) + `@tailwindcss/vite` | MIT | Styling (build-time) | No (compiled CSS only) |
| [`vite`](https://www.npmjs.com/package/vite) + `@vitejs/plugin-react` | MIT | Build tool (development) | No |
| [`typescript`](https://www.npmjs.com/package/typescript), `vitest`, `jsdom`, `@testing-library/*`, `eslint` + plugins, `prettier`, `@tauri-apps/cli` | MIT / Apache-2.0 / ISC | Development, testing, linting, bundling | No |

## Fonts

| Component | License | Purpose | Bundled in build |
| --- | --- | --- | --- |
| [Amiri Regular](https://github.com/aliftype/amiri) | SIL Open Font License 1.1 | Arabic-capable font used for watermarks and page numbers; the full font is embedded in generated PDFs as a Type0/CIDFontType2 (`Identity-H`) font | Yes |

The font license text is kept at [`assets/fonts/OFL.txt`](../assets/fonts/OFL.txt).

## Rendering engine note

Thumbnails and PDF-to-images use **hayro**, a pure-Rust PDF rasterizer (Apache-2.0 OR MIT).
It is compiled into the application, so no separate native library (and no per-platform
binary) needs to be shipped. This keeps the installer self-contained and avoids the
licensing/distribution complexity of PDFium or MuPDF.

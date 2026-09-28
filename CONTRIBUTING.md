# Contributing

Thanks for your interest in PDF Toolkit. This project is a local-first, privacy-focused
PDF utility for Windows with first-class Arabic and English support.

## Development setup

1. Install prerequisites: Node.js 20+ (22 recommended), Rust stable, and the
   [Tauri 2 prerequisites](https://tauri.app/start/prerequisites/) for your OS.
2. Install dependencies:

   ```bash
   npm install
   ```

3. Run the app:

   ```bash
   npm run tauri dev
   ```

## Engineering loop

Before opening a pull request, make sure all of the following pass:

```bash
npm run format:check
npm run lint
npm run typecheck
npm run test

cargo fmt --all -- --check
cargo clippy -p pdf-core --all-targets -- -D warnings
cargo test -p pdf-core --all-targets
```

Then, for changes that affect real output, run the relevant operation against a real PDF
fixture and inspect the result. Do not rely on mocked processing alone.

## Guidelines

- Keep the UI responsive: never block the React thread with heavy processing.
- Never overwrite the user's original file by default; always produce a new output.
- Keep external engines behind adapters in `crates/pdf-core`.
- Preserve the local-first guarantee: no uploads, no cloud conversion, no telemetry that
  contains file names or contents.
- Arabic is first-class. Test RTL layout and Arabic text rendering when relevant.

## Commit messages

Use scoped, logical commits, for example:

- `chore: bootstrap tauri pdf toolkit`
- `feat: implement pdf merge`
- `test: add pdf operation fixtures`
- `ci: build windows release`

## Reporting issues

Include what you expected, what happened, and the smallest PDF or image that reproduces the
problem when possible. Never attach confidential documents.

# rust-implement-linux-wasm

A tiny Rust kernel wired to a browser host via WebAssembly.

## Getting started

Prerequisites:
- Rust stable (`rustup`)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- `wasm-pack`: `cargo install wasm-pack`
- Node.js 18+ with npm

Run the web app:

```bash
cd web
npm install
npm run dev
```

Build the web app (includes wasm build):

```bash
cd web
npm run build
```

Build wasm only:

```bash
cd web
npm run build:wasm
```

Rust workspace checks:

```bash
cargo fmt
cargo clippy --workspace --all-targets
cargo test --workspace
```

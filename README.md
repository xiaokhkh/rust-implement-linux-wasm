# rust-implement-linux-wasm

A tiny Rust kernel wired to a browser host via WebAssembly.

## Architecture at a Glance

```
┌──────────────────────────────────────┐
│              Web Browser              │
│   (JS Runtime · DOM · Web APIs · IO)  │
└───────────────▲──────────────────────┘
                │
┌───────────────┴──────────────────────┐
│        WebAssembly Runtime            │
│   (Deterministic execution + memory)  │
│        “Idealized CPU + RAM”          │
└───────────────▲──────────────────────┘
                │
┌───────────────┴──────────────────────┐
│              Host ABI                 │
│   console · time · fs · net (future)  │
│   Platform / device abstraction       │
└───────────────▲──────────────────────┘
                │
┌───────────────┴──────────────────────┐
│              Kernel                   │
│   boot · syscalls · tasks · VFS       │
│   shell · rules · abstractions        │
└──────────────────────────────────────┘
```

## What This Means (10 Lines)

EN

The browser is the real host system.

WebAssembly provides a safe, deterministic execution sandbox.

WASM is treated as an idealized CPU + memory, not an OS.

The Host ABI replaces hardware, firmware, and device drivers.

The kernel cannot access the outside world directly.

All IO and resources go through rules defined by the kernel.

The kernel defines semantics, not hardware control.

Processes, syscalls, and files are abstractions you build.

This mirrors Linux design under extreme constraints.

If you understand this kernel, you understand why Linux exists.

中文

浏览器是真正的宿主系统

WebAssembly 提供安全、确定性的执行环境

WASM 被视为“理想化的 CPU + 内存”

Host ABI 替代了硬件、固件和设备驱动

Kernel 不能直接访问外部世界

所有 IO 和资源必须经过内核定义的规则

Kernel 定义的是“语义”，而不是硬件

进程、系统调用、文件都是你构建的抽象

这是在极端受限环境下复现 Linux 设计

理解这个内核，就理解 Linux 为什么这样设计 Day 1

## Steps

- Step 02: Syscall boundary - `docs/steps/02-syscall.md`

## Getting started

Prerequisites:
- Rust stable (`rustup`)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- `wasm-pack`: `cargo install wasm-pack`
- Node.js 18+ with pnpm

Run the web app:

```bash
cd web
pnpm i
pnpm dev
```

Build the web app (includes wasm build):

```bash
cd web
pnpm run build
```

Build wasm only:

```bash
cd web
pnpm run build:wasm
```

Rust workspace checks:

```bash
cargo fmt
cargo clippy --workspace --all-targets
cargo test --workspace
```

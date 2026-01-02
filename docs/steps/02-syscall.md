# Step 02 - Syscall Boundary

## Why syscalls

User code must not touch the host or browser APIs directly. A syscall boundary
keeps userland pure and forces all IO through kernel rules. That separation is
what makes an OS an OS: the kernel owns resources and enforces policy.

## TODO

- Add a syscall ABI with write/exit numbers and error codes.
- Route IO through a kernel syscall dispatcher.
- Add a tiny userland program that prints via syscalls.
- Verify userland has no host/wasm imports.

## Acceptance

1) Build the web demo:

```bash
cd web
pnpm run build:wasm
pnpm dev
```

2) In the browser terminal:

```
hello
```

Expected output (line breaks ok):

```
> hello
hello from userland
>
```

3) Run workspace checks:

```bash
cargo test -p userland
cargo clippy --workspace --all-targets
```

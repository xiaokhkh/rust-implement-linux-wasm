//! Minimal kernel entry point for Day 1.

use abi::HostAbi;

/// Kernel main entry point invoked by a host environment.
pub fn kmain<H: HostAbi>(host: &mut H) {
    host.console_write("Kernel booting...\n");
    host.console_write("Welcome to rust-implement-linux-wasm\n");
    host.console_write("> ");
}

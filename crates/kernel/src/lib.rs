//! Minimal kernel entry point for Day 1.

use abi::HostAbi;

/// Kernel main entry point invoked by a host environment.
pub fn kmain<H: HostAbi>(host: &mut H) {
    host.console_write("Kernel booting...\n");
    host.console_write("Welcome to rust-implement-linux-wasm\n");
    host.console_write("Type 'help' for commands.\n");
    host.console_write("> ");
}

/// Handle a single line of user input.
pub fn handle_input<H: HostAbi>(host: &mut H, line: &str) {
    let line = line.trim();
    if line.is_empty() {
        host.console_write("> ");
        return;
    }

    if line == "help" {
        host.console_write("Commands: help, echo <text>\n> ");
        return;
    }

    if let Some(rest) = line.strip_prefix("echo ") {
        host.console_write(rest);
        host.console_write("\n> ");
        return;
    }

    host.console_write("Unknown command. Type 'help'.\n> ");
}

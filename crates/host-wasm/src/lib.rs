use abi::HostAbi;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = hostConsoleWrite)]
    fn host_console_write(s: &str);
}

struct WasmHost;

impl HostAbi for WasmHost {
    fn console_write(&mut self, s: &str) {
        host_console_write(s);
    }
}

/// Boot the kernel from the browser host.
#[wasm_bindgen]
pub fn boot() {
    let mut host = WasmHost;
    kernel::kmain(&mut host);
}

/// Forward a single line of input to the kernel.
#[wasm_bindgen]
pub fn handle_input(line: &str) {
    let mut host = WasmHost;
    kernel::handle_input(&mut host, line);
}

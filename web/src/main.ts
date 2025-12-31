import init, { boot, handle_input } from "./wasm/host_wasm.js";
import { createTerminal } from "./terminal";

const terminalEl = document.getElementById("terminal");
if (!terminalEl) {
  throw new Error("Missing terminal container");
}

const terminal = createTerminal(terminalEl);
let wasmReady = false;

(window as typeof window & { hostConsoleWrite?: (s: string) => void }).hostConsoleWrite = (
  s: string,
) => {
  terminal.write(s);
};

async function start() {
  await init();
  boot();
  wasmReady = true;
  terminal.focus();
}

start();

terminal.onLine((line) => {
  if (!wasmReady) {
    return;
  }
  handle_input(line);
});

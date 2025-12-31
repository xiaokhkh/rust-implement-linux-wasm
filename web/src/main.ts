import init, { boot } from "./wasm/host_wasm.js";
import { createTerminal } from "./terminal";

const terminalEl = document.getElementById("terminal");
if (!terminalEl) {
  throw new Error("Missing terminal container");
}

const terminal = createTerminal(terminalEl);

(window as typeof window & { hostConsoleWrite?: (s: string) => void }).hostConsoleWrite = (
  s: string,
) => {
  terminal.write(s);
};

async function start() {
  await init();
  boot();
}

start();

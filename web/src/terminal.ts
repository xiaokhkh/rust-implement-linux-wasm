export type TerminalWriter = {
  write: (chunk: string) => void;
  onLine: (handler: (line: string) => void) => void;
  focus: () => void;
};

export function createTerminal(container: HTMLElement): TerminalWriter {
  const output = document.createElement("pre");
  output.className = "terminal-output";
  output.textContent = "";
  container.innerHTML = "";
  container.appendChild(output);

  let buffer = "";
  let inputBuffer = "";
  let lineHandler: ((line: string) => void) | null = null;

  container.tabIndex = 0;
  container.setAttribute("role", "textbox");
  container.setAttribute("aria-label", "Terminal");

  const render = () => {
    output.textContent = buffer + inputBuffer;
    container.scrollTop = container.scrollHeight;
  };

  const commitLine = () => {
    const line = inputBuffer;
    inputBuffer = "";
    buffer += `${line}\n`;
    render();
    lineHandler?.(line);
  };

  container.addEventListener("keydown", (event) => {
    if (event.key === "Enter") {
      event.preventDefault();
      commitLine();
      return;
    }

    if (event.key === "Backspace") {
      event.preventDefault();
      inputBuffer = inputBuffer.slice(0, -1);
      render();
      return;
    }

    if (event.key.length === 1 && !event.ctrlKey && !event.metaKey && !event.altKey) {
      event.preventDefault();
      inputBuffer += event.key;
      render();
    }
  });

  container.addEventListener("click", () => {
    container.focus();
  });

  return {
    write(chunk: string) {
      buffer += chunk;
      render();
    },
    onLine(handler: (line: string) => void) {
      lineHandler = handler;
    },
    focus() {
      container.focus();
    },
  };
}

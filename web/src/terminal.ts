export type TerminalWriter = {
  write: (chunk: string) => void;
};

export function createTerminal(container: HTMLElement): TerminalWriter {
  const output = document.createElement("pre");
  output.className = "terminal-output";
  output.textContent = "";
  container.innerHTML = "";
  container.appendChild(output);

  return {
    write(chunk: string) {
      output.textContent += chunk;
      container.scrollTop = container.scrollHeight;
    },
  };
}

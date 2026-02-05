const { Command } = window.__TAURI__.shell;

async function run() {
  const command = Command.sidecar('binaries/sidecar', ['hello', 'Tauri']);
  const output = await command.execute();
  document.querySelector('h1').textContent = output.stdout;
}

run();
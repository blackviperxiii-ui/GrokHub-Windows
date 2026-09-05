# GrokHub for Windows

Native Rust cabin (egui). Not Electron. Not Tauri.

**v2.9.1** — Grok Build **1.0.21**. History is `grok sessions` 1:1. `/update` overlays the GUI and updates `grok`. MCP tools can ask for a form or URL. Per-user installer.

## Install

1. Download **GrokHub-Setup-2.9.1.exe** from [Releases](https://github.com/blackviperxiii-ui/GrokHub-Windows/releases/latest).
2. Run it (no admin). Installs to `%LOCALAPPDATA%\Programs\GrokHub`.
3. Launch GrokHub. Close hides to the tray; Quit from the tray.

If the installer could not vendor `grok.exe`, install Grok Build **alpha**:

```powershell
$env:GROK_CHANNEL='alpha'; irm https://x.ai/cli/install.ps1 | iex
```

Then `grok login`.

| Repo | Platform |
|------|----------|
| **[GrokHub-Windows](https://github.com/blackviperxiii-ui/GrokHub-Windows)** (this) | Windows x86_64 |
| [GrokHub](https://github.com/blackviperxiii-ui/GrokHub) | Arch Linux / CachyOS |

## Build from source

```bash
cargo test --workspace
cargo run -p grokhub-app
```

## License

MIT

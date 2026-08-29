# GrokHub for Windows

Native Rust cabin (egui). Not Electron. Not Tauri.

**v2.7.0** — Grok Build 1.0.13, History 1:1 with `grok sessions`, desktop filesystem/shell. Per-user installer.

## Install

1. Download **GrokHub-Setup-2.7.0.exe** from [Releases](https://github.com/blackviperxiii-ui/GrokHub-Windows/releases/latest).
2. Run it (no admin). Installs to `%LOCALAPPDATA%\Programs\GrokHub`.
3. Launch GrokHub. Close hides to the tray; Quit from the tray.

If the installer could not vendor `grok.exe`, install Grok Build:

```powershell
irm https://x.ai/cli/install.ps1 | iex
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

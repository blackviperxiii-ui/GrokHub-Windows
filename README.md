# GrokHub for Windows

Native Rust cabin (egui). Not Electron. Not Tauri.

**v2.6.43** — per-user installer. Grok Build (`grok.exe`) is the agent and computer-use.

## Install

1. Download **GrokHub-Setup-2.6.43.exe** from [Releases](https://github.com/blackviperxiii-ui/GrokHub-Windows/releases/latest).
2. Run it (no admin). Installs to `%LOCALAPPDATA%\Programs\GrokHub`.
3. Launch GrokHub. First launch installs Grok Build CLI if it is missing (no extra terminal). Close hides to the tray; Quit from the tray.

The cabin is a GUI app. There is no console window behind it. Closing the cabin window hides it; use **Quit** on the tray icon to exit.

If Grok Build still is not present after first launch, Settings → Host → **Install**, or:

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

Windows release pack (fails if `grok.exe` cannot be downloaded):

```powershell
pwsh -File scripts/make-windows-release.ps1
```

Offline pack without vendoring Grok Build:

```powershell
pwsh -File scripts/make-windows-release.ps1 -SkipGrok
```

## License

MIT

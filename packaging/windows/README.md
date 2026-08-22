# GrokHub Windows packaging

Per-user install (no admin): `%LOCALAPPDATA%\Programs\GrokHub`

## Prerequisites

- Rust toolchain with Windows target (`x86_64-pc-windows-msvc` or gnu)
- [Inno Setup 6](https://jrsoftware.org/isinfo.php) — `ISCC.exe` at:
  - `%ProgramFiles%\Inno Setup 6\ISCC.exe`
  - `%LOCALAPPDATA%\Programs\Inno Setup 6\ISCC.exe`

## Build

```powershell
pwsh -File scripts/make-windows-release.ps1
```

Stages `target/release/grokhub.exe` + `grokhub-hub.exe`, optionally downloads Grok CLI into the stage, then runs ISCC.

## Outputs (`dist-release/`)

| Artifact | Name |
|----------|------|
| Inno installer | `GrokHub-Setup-<version>.exe` |
| Portable zip | `grokhub-windows-v<version>.zip` |

Missing `grokhub.exe` / `grokhub-hub.exe` is fatal. Grok download failure is a warning only (`skipifsourcedoesntexist` on those files).

## Lock test (no Windows build)

```powershell
pwsh -File scripts/make-windows-release.tests.ps1
```

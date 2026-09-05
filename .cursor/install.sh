#!/usr/bin/env bash
# Cloud Agent bootstrap for GrokHub. Idempotent and non-interactive.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

SUDO=""
if [[ "$(id -u)" -ne 0 ]]; then
  SUDO="sudo"
fi

# Native GUI build deps for the eframe/GTK cabin + hub (mirrors .github/workflows/ci.yml).
if command -v apt-get >/dev/null 2>&1; then
  $SUDO apt-get update
  $SUDO apt-get install -y --no-install-recommends \
    pkg-config libgtk-3-dev libxkbcommon-dev libxkbcommon-x11-dev \
    libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
    libgl1-mesa-dev libwayland-dev libasound2-dev
fi

# Warm the workspace build so agents start with everything compiled.
cargo build --workspace --locked

# First-install Grok Build CLI alpha. Channel must be passed to bash, not curl.
# scripts/install-grok-cli.sh is overlay-safe and does not fail the cabin setup.
if [[ -x "$ROOT/scripts/install-grok-cli.sh" ]]; then
  PREFIX="${PREFIX:-$HOME/.local}" bash "$ROOT/scripts/install-grok-cli.sh" \
    || echo "grok: install-grok-cli.sh continued"
fi

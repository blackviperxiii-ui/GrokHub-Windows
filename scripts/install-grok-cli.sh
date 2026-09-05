#!/usr/bin/env bash
# Install xAI Grok Build CLI alpha (https://x.ai/cli) alongside grokhub.
# Official installer writes ~/.grok/bin/grok. This also links PREFIX/bin/grok.
# GROK_CHANNEL must be passed to bash (not curl) or the installer defaults to stable.
# Overlay-safe: never fails the cabin install.
set -u

PREFIX="${PREFIX:-$HOME/.local}"
GROK_INSTALL_URL="${GROK_INSTALL_URL:-https://x.ai/cli/install.sh}"
GROK_CHANNEL="${GROK_CHANNEL:-alpha}"

if [[ -z "${GROK_BIN_DIR:-}" ]]; then
  case "$PREFIX" in
    /usr|/usr/local) export GROK_BIN_DIR="$PREFIX/bin" ;;
  esac
fi

grok_present() {
  command -v grok >/dev/null 2>&1 \
    || [[ -x "${GROK_BIN_DIR:-}/grok" ]] \
    || [[ -x "$PREFIX/bin/grok" ]] \
    || [[ -x "$HOME/.grok/bin/grok" ]]
}

grok_bin() {
  if command -v grok >/dev/null 2>&1; then
    command -v grok
  elif [[ -x "${GROK_BIN_DIR:-}/grok" ]]; then
    echo "${GROK_BIN_DIR}/grok"
  elif [[ -x "$PREFIX/bin/grok" ]]; then
    echo "$PREFIX/bin/grok"
  elif [[ -x "$HOME/.grok/bin/grok" ]]; then
    echo "$HOME/.grok/bin/grok"
  fi
}

link_into_prefix() {
  local src=""
  if [[ -x "${GROK_BIN_DIR:-}/grok" ]]; then
    src="${GROK_BIN_DIR}/grok"
  elif [[ -x "$HOME/.grok/bin/grok" ]]; then
    src="$HOME/.grok/bin/grok"
  elif command -v grok >/dev/null 2>&1; then
    src="$(command -v grok)"
  fi
  [[ -n "$src" ]] || return 0
  mkdir -p "$PREFIX/bin" || return 0
  if [[ "$src" != "$PREFIX/bin/grok" && ! -e "$PREFIX/bin/grok" ]]; then
    ln -sf "$src" "$PREFIX/bin/grok" 2>/dev/null \
      || install -Dm755 "$src" "$PREFIX/bin/grok" \
      || true
  fi
  local agent=""
  if [[ -x "${GROK_BIN_DIR:-}/agent" ]]; then
    agent="${GROK_BIN_DIR}/agent"
  elif [[ -x "$HOME/.grok/bin/agent" ]]; then
    agent="$HOME/.grok/bin/agent"
  fi
  if [[ -n "$agent" && "$agent" != "$PREFIX/bin/agent" && ! -e "$PREFIX/bin/agent" ]]; then
    ln -sf "$agent" "$PREFIX/bin/agent" 2>/dev/null || true
  fi
}

run_official_alpha() {
  if ! command -v curl >/dev/null 2>&1 && ! command -v wget >/dev/null 2>&1; then
    echo "grok: curl or wget required to install Grok Build CLI alpha from https://x.ai/cli"
    return 0
  fi
  echo "grok: installing Grok Build CLI alpha from https://x.ai/cli (GROK_CHANNEL=${GROK_CHANNEL})"
  # Channel must reach bash. `VAR= curl | bash` leaves the installer on stable.
  if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$GROK_INSTALL_URL" | GROK_CHANNEL="$GROK_CHANNEL" bash \
      || echo "grok: official installer failed — cabin install continues"
  else
    wget -qO- "$GROK_INSTALL_URL" | GROK_CHANNEL="$GROK_CHANNEL" bash \
      || echo "grok: official installer failed — cabin install continues"
  fi
}

if grok_present; then
  link_into_prefix
  echo "grok: Grok Build CLI present — updating ${GROK_CHANNEL} channel"
  bin="$(grok_bin)"
  if [[ -n "$bin" ]] && "$bin" update --"${GROK_CHANNEL}" >/dev/null 2>&1; then
    echo "grok: updated Grok Build CLI (${GROK_CHANNEL})"
  else
    run_official_alpha
  fi
  link_into_prefix
  if grok_present; then
    echo "grok: Grok Build CLI ready (${GROK_CHANNEL})"
  fi
  exit 0
fi

run_official_alpha
link_into_prefix
if grok_present; then
  echo "grok: installed Grok Build CLI (${GROK_CHANNEL})"
else
  echo "grok: missing — run: curl -fsSL https://x.ai/cli/install.sh | GROK_CHANNEL=alpha bash"
fi

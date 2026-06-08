#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN_DIR="$HOME/.local/bin"
APP_BIN="$BIN_DIR/cliptube-ai"
DESKTOP_DIR="$HOME/.local/share/applications"
ICON_DIR="$HOME/.local/share/icons/hicolor/scalable/apps"

mkdir -p "$BIN_DIR"
mkdir -p "$DESKTOP_DIR"
mkdir -p "$ICON_DIR"

if [[ ! -x "$ROOT_DIR/target/release/cliptube-ai" ]]; then
  echo "No encontré target/release/cliptube-ai. Compilando release primero..."
  (cd "$ROOT_DIR" && cargo build --release)
fi

install -m 0755 "$ROOT_DIR/target/release/cliptube-ai" "$APP_BIN"
sed "s|@CLIPTUBE_EXEC@|$APP_BIN|g" "$ROOT_DIR/packaging/io.github.cliptubeai.ClipTubeAI.desktop" \
  > "$DESKTOP_DIR/io.github.cliptubeai.ClipTubeAI.desktop"
install -m 0644 "$ROOT_DIR/assets/icon.svg" "$ICON_DIR/cliptube-ai.svg"

if command -v update-desktop-database >/dev/null 2>&1; then
  update-desktop-database "$DESKTOP_DIR" >/dev/null 2>&1 || true
fi

echo "Binario instalado en: $APP_BIN"
echo "Desktop file instalado en: $DESKTOP_DIR/io.github.cliptubeai.ClipTubeAI.desktop"
echo "Icono instalado en: $ICON_DIR/cliptube-ai.svg"

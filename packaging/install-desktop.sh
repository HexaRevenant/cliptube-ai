#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
mkdir -p "$HOME/.local/share/applications"
mkdir -p "$HOME/.local/share/icons/hicolor/scalable/apps"
install -m 0644 "$ROOT_DIR/packaging/io.github.cliptubeai.ClipTubeAI.desktop" "$HOME/.local/share/applications/io.github.cliptubeai.ClipTubeAI.desktop"
install -m 0644 "$ROOT_DIR/assets/icon.svg" "$HOME/.local/share/icons/hicolor/scalable/apps/cliptube-ai.svg"
echo "Instalado en: $HOME/.local/share/applications/io.github.cliptubeai.ClipTubeAI.desktop"
echo "Icono instalado en: $HOME/.local/share/icons/hicolor/scalable/apps/cliptube-ai.svg"
echo "Si cambias la ruta del proyecto, actualiza Exec en ese archivo."

#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
mkdir -p "$HOME/.local/share/applications"
install -m 0644 "$ROOT_DIR/packaging/io.github.cliptubeai.ClipTubeAI.desktop" "$HOME/.local/share/applications/io.github.cliptubeai.ClipTubeAI.desktop"
echo "Instalado en: $HOME/.local/share/applications/io.github.cliptubeai.ClipTubeAI.desktop"
echo "Si cambias la ruta del proyecto, actualiza Exec/Icon en ese archivo."

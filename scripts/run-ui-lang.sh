#!/usr/bin/env bash
set -euo pipefail

LANG_CODE="${1:-en}"
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

pkill -f './target/release/cliptube-ai' >/dev/null 2>&1 || true
CLIPTUBE_UI_LANGUAGE="$LANG_CODE" ./target/release/cliptube-ai

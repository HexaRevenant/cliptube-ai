#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
APP_NAME="ClipTube AI"
DIST_DIR="${ROOT_DIR}/dist/macos"
ARCH_SUFFIX="${1:-}"
OUTPUT_STEM="cliptube-ai-macos"

if [[ -n "${ARCH_SUFFIX}" ]]; then
  APP_DIR="${DIST_DIR}/${APP_NAME} (${ARCH_SUFFIX}).app"
  DMG_PATH="${DIST_DIR}/${OUTPUT_STEM}-${ARCH_SUFFIX}.dmg"
else
  APP_DIR="${DIST_DIR}/${APP_NAME}.app"
  DMG_PATH="${DIST_DIR}/${OUTPUT_STEM}.dmg"
fi

STAGE_DIR="${DIST_DIR}/dmg-stage"

if [[ ! -d "${APP_DIR}" ]]; then
  echo "Missing app bundle: ${APP_DIR}" >&2
  exit 1
fi

rm -rf "${STAGE_DIR}" "${DMG_PATH}"
mkdir -p "${STAGE_DIR}"
cp -R "${APP_DIR}" "${STAGE_DIR}/${APP_NAME}.app"
ln -s /Applications "${STAGE_DIR}/Applications"

hdiutil create \
  -volname "${APP_NAME}" \
  -srcfolder "${STAGE_DIR}" \
  -ov -format UDZO \
  "${DMG_PATH}"

rm -rf "${STAGE_DIR}"

echo "Created: ${DMG_PATH}"

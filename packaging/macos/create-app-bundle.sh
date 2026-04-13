#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
APP_NAME="ClipTube AI"
BUNDLE_ID="io.github.cliptubeai.ClipTubeAI"
BIN_NAME="cliptube-ai"
VERSION="${1:-0.1.0}"
DIST_DIR="${ROOT_DIR}/dist/macos"
ARCH_SUFFIX="${2:-}"
BIN_PATH="${3:-${ROOT_DIR}/target/release/${BIN_NAME}}"
OUTPUT_STEM="cliptube-ai-macos"

if [[ -n "${ARCH_SUFFIX}" ]]; then
  APP_DIR="${DIST_DIR}/${APP_NAME} (${ARCH_SUFFIX}).app"
  ZIP_NAME="${OUTPUT_STEM}-${ARCH_SUFFIX}-app.zip"
else
  APP_DIR="${DIST_DIR}/${APP_NAME}.app"
  ZIP_NAME="${OUTPUT_STEM}-app.zip"
fi

CONTENTS_DIR="${APP_DIR}/Contents"
MACOS_DIR="${CONTENTS_DIR}/MacOS"
RESOURCES_DIR="${CONTENTS_DIR}/Resources"

rm -rf "${APP_DIR}"
mkdir -p "${MACOS_DIR}" "${RESOURCES_DIR}"

cp "${BIN_PATH}" "${MACOS_DIR}/${BIN_NAME}"
chmod +x "${MACOS_DIR}/${BIN_NAME}"
cp "${ROOT_DIR}/assets/icon.icns" "${RESOURCES_DIR}/icon.icns"

cat > "${CONTENTS_DIR}/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDevelopmentRegion</key>
  <string>en</string>
  <key>CFBundleDisplayName</key>
  <string>${APP_NAME}</string>
  <key>CFBundleExecutable</key>
  <string>${BIN_NAME}</string>
  <key>CFBundleIconFile</key>
  <string>icon</string>
  <key>CFBundleIconName</key>
  <string>icon</string>
  <key>CFBundleIdentifier</key>
  <string>${BUNDLE_ID}</string>
  <key>CFBundleInfoDictionaryVersion</key>
  <string>6.0</string>
  <key>CFBundleName</key>
  <string>${APP_NAME}</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>${VERSION}</string>
  <key>CFBundleVersion</key>
  <string>${VERSION}</string>
  <key>LSMinimumSystemVersion</key>
  <string>11.0</string>
  <key>NSPrincipalClass</key>
  <string>NSApplication</string>
  <key>NSHighResolutionCapable</key>
  <true/>
</dict>
</plist>
PLIST

mkdir -p "${DIST_DIR}"
rm -f "${DIST_DIR}/${ZIP_NAME}"
/usr/bin/ditto -c -k --sequesterRsrc --keepParent "${APP_DIR}" "${DIST_DIR}/${ZIP_NAME}"

echo "Created: ${APP_DIR}"
echo "Created: ${DIST_DIR}/${ZIP_NAME}"

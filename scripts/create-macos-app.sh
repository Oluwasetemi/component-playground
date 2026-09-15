#!/usr/bin/env bash
set -euo pipefail

profile="${1:-debug}"
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
target_dir="$repo_root/target/$profile"
binary="$target_dir/component-playground-desktop"
app_dir="$target_dir/Component Playground.app"
icon_png="$repo_root/crates/desktop/assets/app-icon.png"
iconset="$target_dir/AppIcon.iconset"

if [[ ! -x "$binary" ]]; then
  echo "missing executable: $binary" >&2
  exit 1
fi

rm -rf "$app_dir" "$iconset"
mkdir -p "$app_dir/Contents/MacOS" "$app_dir/Contents/Resources" "$iconset"
cp "$binary" "$app_dir/Contents/MacOS/Component Playground"
chmod +x "$app_dir/Contents/MacOS/Component Playground"

for size in 16 32 128 256 512; do
  sips -z "$size" "$size" "$icon_png" --out "$iconset/icon_${size}x${size}.png" >/dev/null
  double=$((size * 2))
  sips -z "$double" "$double" "$icon_png" --out "$iconset/icon_${size}x${size}@2x.png" >/dev/null
done
iconutil -c icns "$iconset" -o "$app_dir/Contents/Resources/AppIcon.icns"

cat > "$app_dir/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleExecutable</key>
  <string>Component Playground</string>
  <key>CFBundleIconFile</key>
  <string>AppIcon</string>
  <key>CFBundleIdentifier</key>
  <string>com.component-playground.app</string>
  <key>CFBundleName</key>
  <string>Component Playground</string>
  <key>CFBundleDisplayName</key>
  <string>Component Playground</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>0.1.0</string>
  <key>CFBundleVersion</key>
  <string>1</string>
  <key>LSMinimumSystemVersion</key>
  <string>11.0</string>
  <key>NSHighResolutionCapable</key>
  <true/>
</dict>
</plist>
PLIST

echo "$app_dir"

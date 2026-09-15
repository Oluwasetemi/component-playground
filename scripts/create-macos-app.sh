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
  inner=$((size * 70 / 100))
  tmp="$iconset/icon_${size}x${size}.inner.png"
  sips -z "$inner" "$inner" "$icon_png" --out "$tmp" >/dev/null
  scripts/pad-png.py "$tmp" "$iconset/icon_${size}x${size}.png" "$size" "$size"

  double=$((size * 2))
  inner_double=$((double * 70 / 100))
  tmp_double="$iconset/icon_${size}x${size}@2x.inner.png"
  sips -z "$inner_double" "$inner_double" "$icon_png" --out "$tmp_double" >/dev/null
  scripts/pad-png.py "$tmp_double" "$iconset/icon_${size}x${size}@2x.png" "$double" "$double"
done
rm -f "$iconset"/*.inner.png
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

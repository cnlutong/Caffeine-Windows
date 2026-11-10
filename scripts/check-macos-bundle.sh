#!/usr/bin/env bash
set -euo pipefail

app=${1:?Pass the path to a Caffeine .app bundle}
binary="$app/Contents/MacOS/caffeine"
info="$app/Contents/Info.plist"
test "$(lipo -archs "$binary")" = arm64
codesign --verify --deep --strict "$app"
plutil -lint "$app/Contents/Resources/PrivacyInfo.xcprivacy"

# Match the keys even when the compiler combines string literals.
keys=$(strings -a "$binary" | grep -Eo 'drawsBackground|fullScreenEnabled|developerExtrasEnabled|allowsPictureInPictureMediaPlayback|_inspector' | sort -u || true)
if [[ -n "$keys" ]]; then
  printf 'Private WebKit keys found in the executable:\n%s\n' "$keys" >&2
  exit 1
fi

if [[ ${2:-} = --sandbox ]]; then
  test "$(/usr/libexec/PlistBuddy -c 'Print :LSMinimumSystemVersion' "$info")" = 13.0
  test "$(xcrun vtool -show-build "$binary" | awk '$1 == "minos" {print $2}')" = 13.0
  entitlements=$(codesign -d --entitlements :- "$app" 2>/dev/null)
  test "$(printf '%s\n' "$entitlements" | plutil -extract 'com\.apple\.security\.app-sandbox' raw -o - -)" = true
  test "$(printf '%s\n' "$entitlements" | plutil -extract 'com\.apple\.security\.network\.client' raw -o - -)" = true
fi

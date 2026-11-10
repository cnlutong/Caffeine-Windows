#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."
mode=${1:-}
if [[ $# != 1 || ( "$mode" != --local && "$mode" != --signed ) ]]; then
  echo 'Usage: npm run build:macos-store -- --local|--signed' >&2
  exit 2
fi
if [[ $(uname -s) != Darwin ]]; then
  echo 'The Mac App Store build requires macOS.' >&2
  exit 2
fi

export CARGO_TARGET_DIR="$(pwd)/src-tauri/target/macos-app-store"
version=$(node -p 'require("./package.json").version')
identifier=$(node -p 'require("./src-tauri/tauri.conf.json").identifier')
output="$(pwd)/artifacts/macos-app-store/$version"
entitlements="$(pwd)/src-tauri/Entitlements.appstore.plist"
mkdir -p "$output"
stage=$(mktemp -d "$output/.build.XXXXXX")
trap 'rm -rf "$stage"' EXIT

identity=-
if [[ "$mode" = --signed ]]; then
  identity=${MACOS_STORE_APP_IDENTITY:?Set the Apple Distribution signing identity}
  installer=${MACOS_STORE_INSTALLER_IDENTITY:?Set the Mac Installer Distribution signing identity}
  profile=${MACOS_STORE_PROFILE:?Set the path to the Mac App Store Connect provisioning profile}
  [[ "$identity" != - && "$installer" != - && -f "$profile" ]]

  security cms -D -i "$profile" -o "$stage/profile.plist"
  team=$(/usr/libexec/PlistBuddy -c 'Print :TeamIdentifier:0' "$stage/profile.plist")
  prefix=$(/usr/libexec/PlistBuddy -c 'Print :ApplicationIdentifierPrefix:0' "$stage/profile.plist")
  app_identifier=$(/usr/libexec/PlistBuddy -c 'Print :Entitlements:com.apple.application-identifier' "$stage/profile.plist")
  if [[ "$app_identifier" != "$prefix.$identifier" ]]; then
    echo "The provisioning profile must match $identifier." >&2
    exit 1
  fi

  cp "$entitlements" "$stage/entitlements.plist"
  entitlements="$stage/entitlements.plist"
  /usr/libexec/PlistBuddy -c "Add :com.apple.application-identifier string $app_identifier" "$entitlements"
  /usr/libexec/PlistBuddy -c "Add :com.apple.developer.team-identifier string $team" "$entitlements"
fi

npm run tauri -- build --target aarch64-apple-darwin --bundles app --no-sign --ci \
  --config src-tauri/tauri.appstore.conf.json -- --locked
app="$stage/Caffeine.app"
ditto "$CARGO_TARGET_DIR/aarch64-apple-darwin/release/bundle/macos/Caffeine.app" "$app"
test "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$app/Contents/Info.plist")" = "$identifier"
test "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleShortVersionString' "$app/Contents/Info.plist")" = "$version"

signing_time=--timestamp=none
if [[ "$mode" = --signed ]]; then
  cp "$profile" "$app/Contents/embedded.provisionprofile"
  if [[ -n ${MACOS_STORE_BUILD_NUMBER:-} ]]; then
    if [[ ! "$MACOS_STORE_BUILD_NUMBER" =~ ^[1-9][0-9]{0,3}(\.[0-9]{1,2}){0,2}$ ]]; then
      echo 'Set MACOS_STORE_BUILD_NUMBER to a valid CFBundleVersion.' >&2
      exit 2
    fi
    /usr/libexec/PlistBuddy -c "Set :CFBundleVersion $MACOS_STORE_BUILD_NUMBER" "$app/Contents/Info.plist"
  fi
  signing_time=--timestamp
fi
codesign --force --sign "$identity" "$signing_time" --options runtime --entitlements "$entitlements" "$app"
bash scripts/check-macos-bundle.sh "$app" --sandbox

if [[ "$mode" = --local ]]; then
  "$app/Contents/MacOS/caffeine" --self-test
  rm -rf "$output/sandbox/Caffeine.app"
  mkdir -p "$output/sandbox"
  mv "$app" "$output/sandbox/Caffeine.app"
  printf 'Local sandbox build: %s\n' "$output/sandbox/Caffeine.app"
else
  signature=$(codesign -dvv "$app" 2>&1)
  test "$(printf '%s\n' "$signature" | sed -n 's/^TeamIdentifier=//p')" = "$team"
  if ! grep -Eq '^Authority=(Apple Distribution|3rd Party Mac Developer Application):' <<< "$signature"; then
    echo 'The app must use a Mac App Store distribution signing identity.' >&2
    exit 1
  fi
  package="Caffeine_${version}_macos_appstore_arm64.pkg"
  xcrun productbuild --sign "$installer" --component "$app" /Applications "$stage/$package"
  pkgutil --check-signature "$stage/$package"
  mv -f "$stage/$package" "$output/$package"
  (cd "$output" && shasum -a 256 "$package" > "$package.sha256")
  printf 'App Store Connect package: %s\n' "$output/$package"
fi

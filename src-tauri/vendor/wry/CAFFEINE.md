# Wry 0.57.1

The vendored crate comes from the official [`wry` 0.57.1 release](https://crates.io/crates/wry/0.57.1). Its registry archive SHA-256 is `759c50ddad0ef9913bb5496cb9d509a50433a3b78cb0238d61105526589cf0e9`.

Caffeine’s macOS patch is in `src/wkwebview/mod.rs`. It removes the private WKWebView background, older fullscreen, picture-in-picture, and inspector calls from the macOS build. It uses the public setters for keyboard tab focus and background throttling. macOS development can inspect the webview through Safari Web Inspector on supported systems.

When updating Wry, compare this file with the same upstream release and run `scripts/check-macos-bundle.sh` against both release builds. The upstream MIT and Apache-2.0 licenses are included.

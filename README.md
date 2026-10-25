# Caffeine 2.0

English | [简体中文](README.zh-CN.md)

A quiet keep-awake utility built with **Tauri 2 + React + TypeScript + Rust**.

Version 2 targets **Windows x86_64** and **macOS ARM64 (Apple Silicon)** only. A compact panel controls native power requests owned by Rust. No server is required.

## Version 2 usage

- Launch into the system tray / menu bar. Choose **Settings** to open the compact panel.
- **Allow sleep** follows your usual power settings.
- **Keep system awake** prevents idle system sleep while allowing display sleep.
- **Keep screen on** prevents idle system and display sleep.
- On Windows, left-click toggles Off / the last active mode and right-click opens the menu. On macOS, click opens the menu.
- Closing the panel keeps Caffeine running. Choose **Quit** in the panel or tray to exit. Launching again opens the existing instance's panel.
- Language follows the system, with English and Chinese overrides. Language and last active mode are saved; each fresh process starts Off.

Only idle sleep is inhibited. Explicit sleep, lid closure, low battery, thermal conditions, screen locking and managed policy remain under OS control. Keeping a display awake does not turn an already sleeping display on.

## Version 2 development

Install Node.js 24, stable Rust and the platform's [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/). Windows needs the Visual Studio C++ build tools and Windows SDK; macOS needs Xcode Command Line Tools.

```sh
npm ci
npm run tauri -- dev -- --settings
```

`npm run dev` opens a read-only browser preview. It cannot control system power.

```sh
npm run build
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
npm test
```

Build on the corresponding platform:

```sh
# Windows x86_64: per-user NSIS installer
npm run tauri -- build --target x86_64-pc-windows-msvc --bundles nsis

# macOS ARM64: application bundle and disk image
npm run tauri -- build --target aarch64-apple-darwin --bundles app,dmg
```

Outputs live in `src-tauri/target/<target>/release/bundle/`. Run the executable with `--self-test` to briefly create, switch and release real power requests without starting the tray or changing preferences.

The Windows UI needs WebView2. The installer downloads its bootstrapper if absent, requiring network connectivity. Public macOS distribution needs Developer ID signing and Apple notarization. Use trusted code signing for Windows publisher trust.

`.github/workflows/release-v2.yml` tests and builds on both native operating systems. Manual runs produce downloadable artifacts; a `v2.*` tag creates a **draft release** with checksums only after both targets succeed. Windows builds are unsigned; macOS builds use a local ad-hoc signature and are not notarized. Complete signing, notarization and real idle-sleep validation before publishing.

See the [2.0 architecture](docs/architecture-v2.md). The Windows 1.x implementation and existing MSIX instructions are preserved below. `source/` and `packaging/Build-Msix.ps1` remain the 1.x implementation.

---

# Caffeine for Windows (1.x)

English | [简体中文](README.zh-CN.md)

A lightweight Windows tray utility that prevents your computer from sleeping when it is idle.

## Download

Download `Caffeine.exe` from the [latest release](https://github.com/cnlutong/Caffeine-Windows/releases/latest). No installation or administrator privileges are required.

### Windows Smart App Control blocks the app

The default build does not add a code signature. When Windows 11 Smart App Control cannot establish an app's reputation, it blocks unsigned apps; this message alone does not mean a virus was detected. It does not offer per-app exceptions. See [Microsoft's FAQ](https://support.microsoft.com/en-us/windows/security/threat-malware-protection/smart-app-control-frequently-asked-questions).

Distribute either an EXE with a trusted code signature or an MSIX signed and distributed by the Microsoft Store, as described below. Repackaging or using a self-signed certificate does not establish publisher trust.

## Usage

1. Double-click `Caffeine.exe`; the app appears in the notification area.
2. Left-click the icon to toggle between Off and the last active mode.
3. Right-click the icon to select a mode or exit:
   - **Off**
   - **Prevent sleep**: allows the display to turn off as usual.
   - **Prevent sleep and keep display on**

A gray icon means Off, orange means sleep prevention is active, and green means the display is also kept on. The tooltip shows the current state.

The interface automatically uses Chinese when the Windows display language is Chinese; otherwise it uses English.

## Notes

- Self-contained single-file app for 64-bit Windows 10/11.
- Uses the native Windows `SetThreadExecutionState` API and restores the normal power policy on exit.
- Does not block sleep explicitly requested by the user, the power button, closing a laptop lid, or the screen saver.

## Build from source

```powershell
dotnet publish .\source\Caffeine.csproj -c Release -o .\publish
```

Run the built-in self-test:

```powershell
.\publish\Caffeine.exe --self-test
if ($LASTEXITCODE -ne 0) { throw 'Self-test failed' }
```

### Microsoft Store upload package

Requires PowerShell 7, the .NET 10 SDK, and `makeappx.exe` from the Windows SDK. With `makeappx.exe` on `PATH`, run:

```powershell
.\packaging\Build-Msix.ps1
```

Alternatively, pass `-MakeAppxPath 'C:\path\makeappx.exe'`. The script uses the Store identity in `packaging/AppxManifest.xml`, derives the four-part package version from the project version, builds the app, validates the manifest, unpacks the package, and runs the packaged executable's self-test. It only replaces the output package after those checks pass.

Upload `artifacts/msix/Caffeine-Windows_1.1.0.0_x64.msix` under **Caffeine-Windows → Submission → Packages** in Partner Center. The filename follows the project version. The adjacent `.sha256` file is for verification and does not need uploading.

The package is intentionally unsigned for Store submission, not direct distribution as a trusted installer. The [Microsoft Store signs it after certification](https://learn.microsoft.com/en-us/windows/apps/publish/publish-your-app/msix/app-package-requirements). These build checks do not replace Windows App Certification Kit checks, installation/uninstallation and tray interaction testing, or Store certification.

Suggested `runFullTrust` explanation and certification notes:

> Caffeine-Windows is a WinForms desktop tray utility. It uses NotifyIcon for its tray menu and SetThreadExecutionState to prevent idle sleep only when enabled by the user. It runs as the current user without administrator privileges. On launch, look for the coffee-cup icon in the notification area (including hidden icons); no main window opens. Right-click the icon to choose a mode or Exit. It does not collect or transmit user data.

### Standalone EXE release signing

`dotnet publish` does not add an Authenticode signature. [Smart App Control requires](https://learn.microsoft.com/en-us/windows/apps/develop/smart-app-control/code-signing-for-smart-app-control) an RSA code signing certificate from a trusted provider. Self-signed certificates and .NET strong names are not substitutes.

After obtaining a certificate, sign in an environment with access to its private key. These commands assume the certificate is in the current user's `My` certificate store and the Windows SDK's `signtool.exe` is on `PATH`. Replace both placeholders with your certificate thumbprint and your provider's RFC 3161 timestamp URL. For cloud signing, use the service's signing command instead.

```powershell
signtool sign /sha1 "YOUR_CERTIFICATE_THUMBPRINT" /fd SHA256 /tr "YOUR_RFC3161_TIMESTAMP_URL" /td SHA256 .\publish\Caffeine.exe
if ($LASTEXITCODE -ne 0) { throw 'Signing failed' }
signtool verify /pa /all /tw .\publish\Caffeine.exe
if ($LASTEXITCODE -ne 0) { throw 'Signature verification failed' }
```

Sign after the final build, before creating archives or checksums. Distribute the exact file that was signed and verified; rebuilds require signing again. Before release, also test startup and every mode on Windows 11 with Smart App Control enabled. Signature verification alone does not replace that compatibility test.

## Inspiration

Inspired by [Caffeine for Ubuntu](https://wiki.ubuntuusers.de/Caffeine/), which uses a tray icon to temporarily suppress power-saving behavior. This project is an independent Windows implementation and is not affiliated with the original project.

## License

[MIT](LICENSE)

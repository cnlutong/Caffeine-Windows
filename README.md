# Caffeine 2.0

English | [简体中文](README.zh-CN.md)

A quiet keep-awake utility for **Windows and macOS**, built with **Tauri 2 + React + TypeScript + Rust**.

## Download 2.0

| Platform       | Requirements                         | Download v2.0.0                                                                                                                           |
| -------------- | ------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------- |
| Windows x86_64 | 64-bit Windows 10 / 11, Intel or AMD | [Windows installer (.exe)](https://github.com/cnlutong/Caffeine-Windows/releases/download/v2.0.0/Caffeine_2.0.0_windows_x86_64_setup.exe) |
| macOS ARM64    | macOS 11 or later, Apple Silicon     | [Mac disk image (.dmg)](https://github.com/cnlutong/Caffeine-Windows/releases/download/v2.0.0/Caffeine_2.0.0_macos_arm64.dmg)             |

[Release notes](https://github.com/cnlutong/Caffeine-Windows/releases/tag/v2.0.0) · [SHA-256 checksums](https://github.com/cnlutong/Caffeine-Windows/releases/download/v2.0.0/SHA256SUMS.txt) · [All releases](https://github.com/cnlutong/Caffeine-Windows/releases)

Only these two architectures are distributed in 2.0. There are no Windows ARM64, Windows 32-bit or Intel Mac packages.

## What's new in 2.0

- **Native Windows and Mac support** with a shared, compact settings panel built with React. Rust manages the platform's power requests; no server is required.
- **Three clear coffee-cup states**: outline when Off, solid with steam when keeping the system awake, and solid with a sun when keeping the screen on. Mac menu bar icons adapt to light and dark appearances.
- **Matching tray controls on both platforms**: left-click toggles Off / the last active mode; right-click opens the menu.
- **English and Chinese**, saved language and last active mode, and single-instance behavior. Closing the settings panel keeps the tray utility running; quitting releases its power requests.

## Installation

### Windows x86_64

1. Download `Caffeine_2.0.0_windows_x86_64_setup.exe` from the table above and run it.
2. Follow the installer to install for the current user. The interface requires **Microsoft Edge WebView2**; if it is missing, the installer downloads its bootstrapper, so an internet connection is needed for that step.
3. Launch **Caffeine** from the Start menu. Look for the coffee cup in the notification area, including the hidden-icons menu. Right-click it and choose **Settings…** to open the panel.

When upgrading from 1.x, exit the old tray app before starting 2.0.

The v2.0.0 Windows installer is **unsigned**. Windows may display a publisher or reputation warning. [Smart App Control](https://support.microsoft.com/en-us/windows/security/threat-malware-protection/smart-app-control-frequently-asked-questions) may block it and does not support exceptions for individual apps.

### macOS ARM64 / Apple Silicon

1. Download and open `Caffeine_2.0.0_macos_arm64.dmg`.
2. Drag **Caffeine.app** into **Applications**, then eject the disk image.
3. Open **Caffeine** from Applications. The coffee cup appears in the menu bar; no Dock icon or settings window opens at startup. Right-click the cup and choose **Settings…** to open the panel.

The v2.0.0 Mac app is **ad-hoc signed and not notarized**. If macOS blocks it because the developer cannot be verified, first confirm the download source and checksum. If you choose to open it, try launching it once, then use **System Settings → Privacy & Security → Open Anyway** and confirm. On macOS 11 / 12, this is under **System Preferences → Security & Privacy → General**. See [Apple's opening instructions](https://support.apple.com/en-us/102445).

### Verify a download

Download `SHA256SUMS.txt` alongside the installer and compare the matching filename's checksum with the output below. A mismatch means the file should not be installed.

Windows PowerShell:

```powershell
Get-FileHash .\Caffeine_2.0.0_windows_x86_64_setup.exe -Algorithm SHA256
```

macOS Terminal:

```sh
shasum -a 256 Caffeine_2.0.0_macos_arm64.dmg
```

## Usage

| Mode              | Behavior                                                 | Tray icon         |
| ----------------- | -------------------------------------------------------- | ----------------- |
| Allow sleep       | Follow your usual power settings                         | Outline cup       |
| Keep system awake | Prevent idle system sleep; allow the display to turn off | Solid cup + steam |
| Keep screen on    | Prevent idle system and display sleep                    | Solid cup + sun   |

- On both Windows and macOS, left-click toggles Off / the last active mode and right-click opens the menu.
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

The Windows UI needs WebView2. The installer downloads its bootstrapper if absent, requiring network connectivity. For publisher trust, configure trusted code signing on Windows and Developer ID signing with Apple notarization on macOS. The v2.0.0 signing status is documented in the installation instructions above.

`.github/workflows/release-v2.yml` tests and builds on both native operating systems, including architecture checks and native power API self-tests. Manual runs produce downloadable artifacts; a `v2.*` tag creates a **draft release** with checksums only after both targets succeed. Existing releases and their uploaded assets are preserved; publication is a separate manual step.

See the [2.0 architecture](docs/architecture-v2.md). `source/` and `packaging/Build-Msix.ps1` belong to the legacy Windows 1.x implementation; their usage and MSIX instructions are preserved below.

---

<details>
<summary>Legacy Windows 1.x: usage, source builds and MSIX packaging</summary>

The following instructions apply to the Windows-only 1.x app.

## Download

Download `Caffeine.exe` from the [v1.1.0 release](https://github.com/cnlutong/Caffeine-Windows/releases/tag/v1.1.0). No installation or administrator privileges are required.

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

</details>

## Inspiration

Inspired by [Caffeine for Ubuntu](https://wiki.ubuntuusers.de/Caffeine/), which uses a tray icon to temporarily suppress power-saving behavior. This project is an independent implementation and is not affiliated with the original project.

## License

[MIT](LICENSE)

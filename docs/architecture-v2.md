# Caffeine 2.0

Targets are Windows x86_64 (`x86_64-pc-windows-msvc`) and macOS ARM64 (`aarch64-apple-darwin`). Other architectures are rejected by the Rust platform module.

## Ownership

React renders the compact settings panel. Rust owns the current mode, last active mode, language, native power request and tray. Both tray actions and frontend commands call the same controller. State events carry a monotonically increasing revision, so a delayed fetch cannot overwrite a newer tray event.

No window is created at startup. The native tray opens the settings WebView on demand. Closing the WebView destroys it; the tray and power request remain alive. Only an explicit Quit exits the application. A second launch opens the existing instance's panel. Use `--settings` to open the panel on the first launch.

On macOS, LaunchServices reopen events open Settings. A second executable launch checks an OS-managed file lock in the application support directory and sends a payload-free distributed notification to the existing process. macOS places the lock and preferences in the application container when sandboxed. Windows uses the Tauri single-instance plugin.

## Power requests

On Windows, `PowerCreateRequest` creates a process-owned request handle. System mode sets `PowerRequestSystemRequired`; display mode sets both that request and `PowerRequestDisplayRequired`. Each successful increment is paired with `PowerClearRequest`; the handle is always closed. A replacement request is fully activated before the old request is dropped. This avoids depending on the thread affinity of the 1.x `SetThreadExecutionState` implementation.

On macOS, `IOPMAssertionCreateWithName` creates a `PreventUserIdleSystemSleep` or `PreventUserIdleDisplaySleep` assertion at `kIOPMAssertionLevelOn`. A display assertion also prevents idle system sleep. The new assertion is created before the previous one is released. A failed transition leaves the previous mode selected. RAII cleanup and the application exit handler release owned requests.

Assertions inhibit idle sleep only. Explicit sleep, lid closure, thermal/battery conditions, locking and managed device policy remain under OS control. Keeping the display awake does not turn an already sleeping display on.

## Preferences

`settings.json` is stored in the platform's per-user application configuration directory. Only language and last active mode are saved. Every fresh application process starts Off. Files are written to a sibling temporary file, flushed and renamed; failed saves leave the current in-memory state active and show a warning. Invalid files fall back to defaults with a warning.

## Frontend/native boundary

Commands: `get_state`, `set_mode`, `set_language`, `quit_app`. `Mode` accepts `off`, `system`, `display`; `Language` accepts `auto`, `en`, `zh`. Rust deserialization validates input at the IPC boundary. The frontend has no shell, filesystem or network plugin permissions. It subscribes before loading the initial snapshot and applies only equal or newer revisions. A regular browser shows an explicitly labelled, read-only UI preview, never simulated wake prevention.

## Builds and distribution

`npm ci`, `npm run build`, `cargo test --manifest-path src-tauri/Cargo.toml` and `npm run tauri -- build --target <target>` build the application. Windows installers use NSIS and install for the current user. Windows requires WebView2; the default installer downloads its bootstrapper when needed, so a missing runtime needs connectivity. macOS bundles are `.app` / `.dmg` and run as an accessory app without a Dock icon.

The GitHub Actions workflow builds and tests each native target on its own OS. It also checks an ad-hoc sandboxed Mac App Store build against the same version. Both jobs must pass before a `v2.*` tag produces a draft release with SHA-256 checksums. Windows builds are unsigned and the GitHub macOS build is ad-hoc signed without notarization. The separately signed Mac App Store `.pkg` follows the [submission guide](mac-app-store.md). The existing .NET code and MSIX scripts remain the 1.x implementation.

## Validation

Unit tests exercise failed-mode rollback, toggle restoration, duplicate requests, safe restart, corrupt preferences and replacement saves. `caffeine --self-test` briefly creates, switches and releases actual native requests, exits with a failure status on API errors and does not start the tray or change saved preferences.

Before publishing, validate native tray clicks, settings synchronization, window close/reopen, process termination cleanup, real display/system idle timeouts, manual sleep and wake, lock, external display, AC and battery behavior. Windows and macOS must be exercised separately; compiling a target does not establish those runtime behaviors.

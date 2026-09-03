# Caffeine for Windows

English | [简体中文](README.zh-CN.md)

A lightweight Windows tray utility that prevents your computer from sleeping when it is idle.

## Download

Download `Caffeine.exe` from the [latest release](https://github.com/cnlutong/Caffeine-Windows/releases/latest). No installation or administrator privileges are required.

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

## Inspiration

Inspired by [Caffeine for Ubuntu](https://wiki.ubuntuusers.de/Caffeine/), which uses a tray icon to temporarily suppress power-saving behavior. This project is an independent Windows implementation and is not affiliated with the original project.

## License

[MIT](LICENSE)

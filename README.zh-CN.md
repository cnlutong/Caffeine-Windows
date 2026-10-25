# Caffeine 2.0

[English](README.md) | 简体中文

基于 **Tauri 2 + React + TypeScript + Rust** 的轻量防休眠工具。

2.0 仅面向 **Windows x86_64** 和 **macOS ARM64（Apple Silicon）**。界面为紧凑面板，电源状态由 Rust 管理；无需联网服务。

## 2.0 使用

- 启动后显示托盘 / 菜单栏图标；菜单中选择「设置」打开面板。
- **允许休眠**：遵循系统原有电源策略。
- **仅防休眠**：阻止电脑因空闲自动休眠，允许屏幕关闭。
- **屏幕常亮**：同时阻止电脑与屏幕因空闲自动休眠。
- Windows 和 Mac 均为左键单击托盘图标，在关闭与上次启用的模式间切换；右键打开菜单。
- 关闭面板后继续运行；从面板或托盘选择「退出」才结束程序。再次启动会打开已有实例的面板。
- 默认语言跟随系统，也可选择中文或英文。记住语言与上次模式，每次重新启动仍从关闭状态开始。

仅抑制空闲休眠；手动睡眠、合盖、低电量、过热、安全锁屏和设备管理策略仍由系统决定。屏幕常亮模式不会主动唤醒已经熄灭的显示器。

## 2.0 开发与构建

需要 Node.js 24、稳定版 Rust 和对应平台的 [Tauri 系统依赖](https://v2.tauri.app/start/prerequisites/)。Windows 需要 Visual Studio C++ 构建工具与 Windows SDK；macOS 需要 Xcode Command Line Tools。

```sh
npm ci
npm run tauri -- dev -- --settings
```

`npm run dev` 只启动浏览器中的只读界面预览，不能控制系统电源。

```sh
npm run build
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
npm test
```

在相应平台构建发布包：

```sh
# Windows x86_64，生成当前用户安装的 NSIS EXE
npm run tauri -- build --target x86_64-pc-windows-msvc --bundles nsis

# macOS ARM64，生成 APP 与 DMG
npm run tauri -- build --target aarch64-apple-darwin --bundles app,dmg
```

产物在 `src-tauri/target/<目标架构>/release/bundle/`。可对可执行文件运行 `--self-test`：它会短暂申请、切换并释放真实电源请求，不启动托盘，也不修改偏好设置。

Windows 界面依赖 WebView2。安装包会在缺失时下载运行时引导程序，需要网络。macOS 正式分发需要 Developer ID 签名和 Apple 公证；Windows 需要受信任签名以建立发布者信任。

`.github/workflows/release-v2.yml` 在两个原生系统上分别测试和打包。手动运行可下载构建产物；推送 `v2.*` 标签时，两端均成功后创建含校验和的**草稿发布**。Windows 默认未签名，Mac 仅带本地 ad-hoc 签名且未公证，发布前仍需签名、公证与实机空闲休眠验证。

详细设计见 [2.0 架构](docs/architecture-v2.md)，本机工具链见 [外置盘开发环境](docs/local-development.zh-CN.md)。下方保留 1.x 的 Windows 实现与现有 MSIX 打包说明，`source/` 和 `packaging/Build-Msix.ps1` 仍属于 1.x。

---

# Caffeine for Windows（1.x）

[English](README.md) | 简体中文

一个轻量的 Windows 托盘工具，用来阻止电脑因空闲而自动休眠。

## 下载

从[最新版本](https://github.com/cnlutong/Caffeine-Windows/releases/latest)下载 `Caffeine.exe`。程序免安装且无需管理员权限。

### Windows 智能应用控制拦截

本项目的默认构建不添加代码签名。Windows 11 的 Smart App Control（智能应用控制）在无法确认程序信誉时，会拦截未签名的程序；这条提示本身不代表已检测到病毒。它不支持为单个应用添加放行例外，参见[微软常见问题](https://support.microsoft.com/en-us/windows/security/threat-malware-protection/smart-app-control-frequently-asked-questions)。

面向用户分发可选择带有受信任代码签名的 EXE，或由 Microsoft Store 签名并分发的 MSIX，具体见下方构建说明。仅重新打包或使用自签名证书不能解决发布者信任问题。

## 使用

1. 双击 `Caffeine.exe`；应用会出现在任务栏通知区域。
2. 左键单击图标：在“关闭”和上次使用的模式之间切换。
3. 右键单击图标：选择以下模式或退出。
   - **关闭**
   - **仅防止电脑休眠**：允许系统按原计划关闭屏幕。
   - **防止休眠并保持屏幕开启**

灰色图标表示关闭，橙色表示仅防休眠，绿色表示同时保持屏幕开启。鼠标悬停也会显示当前状态。

Windows 显示语言为中文时，程序界面自动使用中文；其他语言环境使用英文。

## 说明

- 单文件、免安装、无需管理员权限，适用于 64 位 Windows 10/11。
- 使用 Windows 原生 `SetThreadExecutionState` API；退出后系统电源策略自动恢复。
- 不会阻止用户主动点击“睡眠”、按电源键或合上笔记本上盖，也不会禁用屏幕保护程序。

## 从源码构建

```powershell
dotnet publish .\source\Caffeine.csproj -c Release -o .\publish
```

运行内置自检：

```powershell
.\publish\Caffeine.exe --self-test
if ($LASTEXITCODE -ne 0) { throw 'Self-test failed' }
```

### Microsoft Store 上传包

需要 PowerShell 7、.NET 10 SDK 和 Windows SDK 的 `makeappx.exe`。在 `makeappx.exe` 已加入 `PATH` 的环境中运行：

```powershell
.\packaging\Build-Msix.ps1
```

也可以通过 `-MakeAppxPath 'C:\路径\makeappx.exe'` 指定工具位置。脚本使用 `packaging/AppxManifest.xml` 中的商店身份，从项目版本生成四段包版本，自动构建、校验清单、解包检查并运行包内程序的自检；只有检查通过后才更新输出包。

将 `artifacts/msix/Caffeine-Windows_1.1.0.0_x64.msix` 上传到 Partner Center 中 **Caffeine-Windows → 产品提交 → 包**。版本号随项目版本变化，旁边的 `.sha256` 文件用于校验，不需要上传。

该包故意不做本地签名，用于商店提交，不能作为已受信任的安装包直接分发。审核通过后由 [Microsoft Store 签名](https://learn.microsoft.com/en-us/windows/apps/publish/publish-your-app/msix/app-package-requirements)。本地打包检查不替代 Windows App Certification Kit 检测、安装/卸载和托盘交互测试，也不代表已通过商店审核。

在提交选项的 `runFullTrust` 权限说明及审核备注中，可填写：

> Caffeine-Windows is a WinForms desktop tray utility. It uses NotifyIcon for its tray menu and SetThreadExecutionState to prevent idle sleep only when enabled by the user. It runs as the current user without administrator privileges. On launch, look for the coffee-cup icon in the notification area (including hidden icons); no main window opens. Right-click the icon to choose a mode or Exit. It does not collect or transmit user data.

### 独立 EXE 发布签名

`dotnet publish` 不会自动添加 Authenticode 签名。[Smart App Control 要求](https://learn.microsoft.com/en-us/windows/apps/develop/smart-app-control/code-signing-for-smart-app-control)签名证书由受信任机构签发，且使用 RSA；自签名证书和 .NET 强名称签名不能替代它。

取得证书后，在能访问该证书私钥的环境中签名。下面的命令适用于证书已安装在当前用户的 `My` 证书存储区，并已将 Windows SDK 的 `signtool.exe` 加入 `PATH` 的情况。将两个占位值替换为证书指纹和证书提供商给出的 RFC 3161 时间戳地址；云签名服务使用其提供的签名命令。

```powershell
signtool sign /sha1 "YOUR_CERTIFICATE_THUMBPRINT" /fd SHA256 /tr "YOUR_RFC3161_TIMESTAMP_URL" /td SHA256 .\publish\Caffeine.exe
if ($LASTEXITCODE -ne 0) { throw 'Signing failed' }
signtool verify /pa /all /tw .\publish\Caffeine.exe
if ($LASTEXITCODE -ne 0) { throw 'Signature verification failed' }
```

签名必须在最后一次构建后、制作压缩包和计算校验和前完成。发布签名并验证过的同一份文件；每次重新构建后都需要重新签名。发布前还需在开启 Smart App Control 的 Windows 11 上验证启动和所有模式；签名验证通过不等于完成了这项兼容性测试。

## 灵感来源

灵感来自 [Ubuntu Caffeine](https://wiki.ubuntuusers.de/Caffeine/)：它通过托盘图标临时抑制系统节能行为。本项目是独立的 Windows 实现，与原项目没有隶属关系。

## 许可证

[MIT](LICENSE)

# Caffeine 2.0

[English](README.md) | 简体中文

适用于 **Windows 和 macOS** 的轻量防休眠工具，基于 **Tauri 2 + React + TypeScript + Rust**。

## 下载 2.0

| 平台           | 系统要求                                   | v2.0.0 下载                                                                                                                             |
| -------------- | ------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------- |
| Windows x86_64 | 64 位 Windows 10 / 11，Intel 或 AMD 处理器 | [Windows 安装包（.exe）](https://github.com/cnlutong/Caffeine-Windows/releases/download/v2.0.0/Caffeine_2.0.0_windows_x86_64_setup.exe) |
| macOS ARM64    | macOS 11 或更新版本，Apple Silicon 芯片    | [Mac 安装镜像（.dmg）](https://github.com/cnlutong/Caffeine-Windows/releases/download/v2.0.0/Caffeine_2.0.0_macos_arm64.dmg)            |

[版本说明](https://github.com/cnlutong/Caffeine-Windows/releases/tag/v2.0.0) · [SHA-256 校验和](https://github.com/cnlutong/Caffeine-Windows/releases/download/v2.0.0/SHA256SUMS.txt) · [全部版本](https://github.com/cnlutong/Caffeine-Windows/releases)

2.0 仅提供上述两种架构，暂不提供 Windows ARM64、32 位 Windows 或 Intel Mac 安装包。

## 2.0 新功能

- **支持 Windows 和 Mac**：共用 React 紧凑设置面板，由 Rust 调用系统原生电源接口，无需联网服务。
- **三种咖啡杯状态图标**：关闭时为空心杯，仅防休眠为实心杯加蒸汽，屏幕常亮为实心杯加太阳；Mac 菜单栏图标适配浅色和深色外观。
- **统一的左右键操作**：两端均为左键开启或关闭，并恢复上次启用的模式；右键呼出菜单。
- **中英文界面与偏好记忆**：保存语言和上次模式，支持单实例运行；关闭设置面板后继续运行，退出时释放电源请求。

## 安装说明

### Windows x86_64

1. 从上表下载并运行 `Caffeine_2.0.0_windows_x86_64_setup.exe`。
2. 按向导完成当前用户安装。界面依赖 **Microsoft Edge WebView2**；如果系统尚未安装，安装包会下载运行时引导程序，此步骤需要联网。
3. 从开始菜单启动 **Caffeine**，在任务栏通知区域找到咖啡杯图标，必要时展开隐藏图标；右键选择「设置…」打开面板。

从 1.x 升级时，请先退出旧版托盘程序，再启动 2.0。

v2.0.0 Windows 安装包**未签名**，系统可能提示发布者未知或信誉不足。[Smart App Control（智能应用控制）](https://support.microsoft.com/en-us/windows/security/threat-malware-protection/smart-app-control-frequently-asked-questions)可能拦截它，且不支持为单个应用添加放行例外。

### macOS ARM64 / Apple Silicon

1. 下载并打开 `Caffeine_2.0.0_macos_arm64.dmg`。
2. 将 **Caffeine.app** 拖入「**应用程序**」，然后推出安装镜像。
3. 从「应用程序」打开 **Caffeine**。启动后咖啡杯出现在菜单栏，不显示 Dock 图标，也不会自动打开设置窗口；右键单击咖啡杯，选择「设置…」打开面板。

v2.0.0 Mac 应用仅有 **ad-hoc 签名，尚未经过 Apple 公证**。如果系统因无法验证开发者而阻止打开，请先确认下载来源与校验和。若决定继续打开，先尝试启动一次，再到「**系统设置 → 隐私与安全性 → 仍要打开**」中确认；macOS 11 / 12 的对应入口是「**系统偏好设置 → 安全性与隐私 → 通用**」。详见 [Apple 官方打开说明](https://support.apple.com/zh-cn/102445)。

### 校验下载文件

同时下载 `SHA256SUMS.txt`，将其中对应文件名的校验和与下方命令输出对比；不一致时请勿安装。

Windows PowerShell：

```powershell
Get-FileHash .\Caffeine_2.0.0_windows_x86_64_setup.exe -Algorithm SHA256
```

macOS 终端：

```sh
shasum -a 256 Caffeine_2.0.0_macos_arm64.dmg
```

## 使用方式

| 模式     | 行为                                 | 托盘图标      |
| -------- | ------------------------------------ | ------------- |
| 允许休眠 | 遵循系统原有电源策略                 | 空心杯        |
| 仅防休眠 | 阻止电脑因空闲自动休眠，允许屏幕关闭 | 实心杯 + 蒸汽 |
| 屏幕常亮 | 同时阻止电脑与屏幕因空闲自动休眠     | 实心杯 + 太阳 |

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

Windows 界面依赖 WebView2，安装包会在缺失时下载运行时引导程序，需要网络。建立发布者信任需要配置 Windows 受信任代码签名，以及 macOS Developer ID 签名和 Apple 公证；v2.0.0 的实际签名状态见上方安装说明。

`.github/workflows/release-v2.yml` 在两个原生系统上分别测试和打包，包含架构检查与原生电源接口自检。手动运行可下载构建产物；推送 `v2.*` 标签时，两端均成功后创建含校验和的**草稿发布**。已有发布及其附件会被保留，正式发布是单独的手动步骤。

详细设计见 [2.0 架构](docs/architecture-v2.md)，本机工具链见 [外置盘开发环境](docs/local-development.zh-CN.md)。`source/` 和 `packaging/Build-Msix.ps1` 属于 Windows 1.x 实现，其使用与 MSIX 打包说明保留在下方。

---

<details>
<summary>旧版 Windows 1.x：使用、源码构建与 MSIX 打包</summary>

以下说明仅适用于 Windows 专用的 1.x 程序。

## 下载

从 [v1.1.0 发布页](https://github.com/cnlutong/Caffeine-Windows/releases/tag/v1.1.0)下载 `Caffeine.exe`。程序免安装且无需管理员权限。

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

</details>

## 灵感来源

灵感来自 [Ubuntu Caffeine](https://wiki.ubuntuusers.de/Caffeine/)：它通过托盘图标临时抑制系统节能行为。本项目为独立实现，与原项目没有隶属关系。

## 许可证

[MIT](LICENSE)

# Mac App Store 提交手册

GitHub DMG 和 Mac App Store 包从同一提交构建，共用 `com.cnlutong.caffeine`、产品版本和界面。GitHub DMG 支持 Apple Silicon 上的 macOS 11 起。商店包要求 Apple Silicon 和 macOS 13 起，并开启 App Sandbox。商店包在自己的容器内保存偏好，首次从 DMG 版切换到商店版时使用默认偏好。

## 构建与验证

按[本机开发环境](local-development.zh-CN.md)设置工具链，然后在项目根目录构建沙盒包。

```sh
npm ci
npm run check:release
npm run build:macos-store -- --local
```

本地沙盒 `.app` 在 `artifacts/macos-app-store/<version>/sandbox/`。脚本验证架构、沙盒签名、隐私清单和已知 WebKit 私有调用，并运行原生电源自检。本地构建用于验证。正式 `.pkg` 走下方签名流程。

同一提交的 GitHub 版使用原发布命令。推送版本标签后，原工作流从同一提交生成 Windows EXE 和 Mac DMG 的 GitHub 草稿，并验证商店沙盒构建。等 App Store 审核通过后，可手动发布两边的同版本。 [App Store 手动发布](https://developer.apple.com/help/app-store-connect/manage-your-apps-availability/select-an-app-store-version-release-option/)

```sh
npm run tauri -- build --target aarch64-apple-darwin --bundles app,dmg --ci -- --locked
```

## 商店页面

首个商店版本为 `2.0.1`，SKU 可用 `caffeine-macos`，主语言为 English (U.S.)，版权字段按仓库许可证填写 `2026 cnlutong`。账号持有人在 App Store Connect 确认定价、发行地区、年龄分级和审核联系人。 [Apple 元数据字段](https://developer.apple.com/help/app-store-connect/reference/app-information/platform-version-information/)

| 字段 | English (U.S.) | 简体中文 |
| --- | --- | --- |
| 建议显示名 | Caffeine: Menu Bar Awake | Caffeine：菜单栏防休眠 |
| 副标题 | Keep your Mac awake | 一键防休眠和屏幕常亮 |
| 类别 | Utilities | 工具 |
| 关键词 | sleep,awake,display,screen,menubar,presentation,idle,power | 防休眠,屏幕常亮,菜单栏,演示,电源,空闲 |

显示名由 App Store Connect 确认可用性。隐私政策 URL 为 `https://github.com/cnlutong/Caffeine-Windows/blob/main/docs/privacy.md`。支持 URL 为 `https://github.com/cnlutong/Caffeine-Windows/blob/main/docs/support.md`。将这些文件合并至默认分支后，从未登录浏览器验证两个 URL。应用隐私问卷可选择 **Data Not Collected**，应用内偏好保存在本机。

Mac 截图来自沙盒版实机界面，每种语言各两张，均为无透明通道的 1280 × 800 PNG。按 `system.png`、`display.png` 的顺序上传到对应语言。 [English (U.S.) 截图](app-store/screenshots/en-US/) · [简体中文截图](app-store/screenshots/zh-Hans/) · [Apple 截图规格](https://developer.apple.com/help/app-store-connect/reference/app-information/screenshot-specifications/)

### English description

Keep your Mac awake during a presentation, a long download, or a task that needs more time. Caffeine gives you two modes from a coffee cup in the menu bar.

- **Keep system awake** prevents idle system sleep and lets the display turn off.
- **Keep screen on** prevents both the display and the Mac from going to sleep because of inactivity.

Click the cup to turn your last used mode on or off. Right-click or Control-click for modes, Settings, and Quit. Caffeine starts with sleep prevention off and follows your usual power settings again when you switch it off or quit. The interface is available in English and Simplified Chinese.

Caffeine controls idle sleep. Manual sleep, closing the lid, and security locking remain under macOS control.

### 简体中文描述

演示、下载大文件或等待长任务完成时，可以从 Mac 菜单栏的咖啡杯快速开启防休眠。

- **仅防休眠**会阻止电脑因空闲自动休眠，同时允许屏幕关闭。
- **屏幕常亮**会同时阻止电脑和屏幕因空闲自动休眠。

单击咖啡杯可开启上次使用的模式，或关闭防休眠。右键或按住 Control 单击可以选择模式、打开设置或退出。每次启动时默认允许休眠。关闭模式或退出后，Mac 按原有电源策略运行。应用支持简体中文和英文。

手动休眠、合盖和安全锁屏仍由 macOS 控制。

### What’s New in 2.0.1

Reopening Caffeine brings up Settings in the existing instance. The settings panel now includes a Privacy Policy link.

再次打开 Caffeine 会显示已有实例的设置窗口。设置面板新增「隐私政策」入口。

## App Review notes

> Caffeine is a menu bar utility. Launch it and look for the coffee cup in the macOS menu bar. Right-click or Control-click the cup and choose Settings to see all controls. Choose Keep system awake, then Keep screen on, then Allow sleep. The coffee cup changes with each mode. Left-click toggles the last active mode. Close the Settings window and reopen Caffeine from Applications to bring Settings back in the same session. Quit via Settings or the menu bar item to release the power request.
>
> The app uses public IOKit assertions for idle system and display sleep. It starts with prevention off, does not change power settings, and has no login, analytics, or background service outside the app process. The Privacy Policy link opens the default browser on request.
>
> The outgoing network entitlement allows WKWebView to render the bundled settings page under App Sandbox. The app loads its interface and communicates with its native backend through local WebKit schemes. Its content security policy restricts connections to local IPC.

## 正式签名与上传

1. 在 Apple Developer 注册 `com.cnlutong.caffeine`，在 App Store Connect 创建 macOS 记录。准备该 App ID 的 **Mac App Store Connect** 描述文件，在本机钥匙串安装带私钥的 **Apple Distribution** 和 **Mac Installer Distribution** 证书。
2. 指定本机签名身份和描述文件路径。描述文件与私钥保存在仓库之外。

   ```sh
   export MACOS_STORE_APP_IDENTITY='Apple Distribution: Your Name (TEAMID)'
   export MACOS_STORE_INSTALLER_IDENTITY='3rd Party Mac Developer Installer: Your Name (TEAMID)'
   export MACOS_STORE_PROFILE='/absolute/path/to/Caffeine.provisionprofile'
   npm run build:macos-store -- --signed
   ```

   同一商店版本重新上传时，通过 `MACOS_STORE_BUILD_NUMBER` 设置递增的 `CFBundleVersion`。产品版本继续使用两渠道共用的版本号。

3. 用 Transporter 验证并上传 `artifacts/macos-app-store/<version>/Caffeine_<version>_macos_appstore_arm64.pkg`。在 App Store Connect 填入页面文案、截图、审核联系人和隐私问卷，选择已处理的构建。提交前在 TestFlight 实机复查初次启动、重复打开、偏好持久化、实际空闲超时和退出清理。 [Apple 签名证书](https://developer.apple.com/help/account/certificates/certificates-overview/) · [Apple 上传构建](https://developer.apple.com/help/app-store-connect/manage-builds/upload-builds/)

# 本机外置盘开发环境

本机项目位于 `/Volumes/dev1/Projects/caffeine`。Rust、交叉工具链和缓存也安装在外置盘；没有修改全局 shell 配置。

执行开发命令前，确认 `/Volumes/dev1` 已挂载且可写，然后在当前终端设置：

```sh
test -w /Volumes/dev1/Projects/caffeine || return 1
export CARGO_HOME=/Volumes/dev1/Developer/rust/cargo
export RUSTUP_HOME=/Volumes/dev1/Developer/rust/rustup
export PATH="$CARGO_HOME/bin:$PATH"
export npm_config_cache=/Volumes/dev1/Developer/cache/npm
export TMPDIR=/Volumes/dev1/codex/tmp
```

启动桌面应用并打开设置：

```sh
npm run tauri -- dev -- --settings
```

Mac 正常构建：

```sh
npm run tauri -- build --target aarch64-apple-darwin --bundles app,dmg --ci
```

本机也准备了 Windows MSVC 交叉构建环境。它不能替代 Windows 实机测试，正式发布仍应使用工作流中的 Windows 原生构建。

```sh
export XWIN_CACHE_DIR=/Volumes/dev1/Developer/cache/xwin
export XWIN_ARCH=x86_64
export NSIS_PATH=/Volumes/dev1/Developer/makensis/3.13/share/nsis
export PATH="/Volumes/dev1/Developer/bin:/Volumes/dev1/Developer/llvm-mingw-20261020-ucrt-macos-universal/bin:/Volumes/dev1/Developer/cargo-xwin-venv/bin:$PATH"
npm run tauri -- build --runner cargo-xwin --target x86_64-pc-windows-msvc --bundles nsis --ci
```

`/Volumes/dev1/Developer/bin/makensis` 通过 NSIS 官方支持的 `NSISDIR` 环境变量指向外置盘的 NSIS 数据目录。LLVM 与 NSIS 下载包均已核对发布方摘要。

启用原生桌面集成测试（普通构建不包含测试命令）：

```sh
CAFFEINE_TEST_CONFIG_DIR=/Volumes/dev1/codex/tmp/caffeine-v2/smoke \
  cargo run --manifest-path src-tauri/Cargo.toml --features desktop-smoke -- --smoke-test
```

此测试需要 `npm run dev` 正在运行。它会真实切换系统电源请求，并通过 macOS `pmset` 检查请求类型、窗口关闭重开后的状态和清理结果；偏好文件写入隔离的外置盘测试目录。

离线发布界面可通过以下命令测试，使用先前 `npm run build` 生成的资源：

```sh
CAFFEINE_TEST_CONFIG_DIR=/Volumes/dev1/codex/tmp/caffeine-v2/smoke-release \
  cargo run --manifest-path src-tauri/Cargo.toml --release \
  --target aarch64-apple-darwin --features desktop-smoke,tauri/custom-protocol -- --smoke-test
```

测试功能只用于验证。最终分发包通过普通 `tauri build` 重新构建，不能携带 `desktop-smoke` 特性。

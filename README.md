# Caffeine for Windows

一个简洁的 Windows 托盘工具，用来阻止电脑因空闲而自动休眠。

## 使用

1. 双击 `Caffeine.exe`；应用会出现在任务栏通知区域。
2. 左键单击图标：在“关闭”和上次使用的模式之间切换。
3. 右键单击图标：选择以下模式或退出。
   - **关闭**
   - **仅防止电脑休眠**：允许系统按原计划关闭屏幕。
   - **防止休眠并保持屏幕开启**

灰色图标表示关闭，橙色表示仅防休眠，绿色表示同时保持屏幕开启。鼠标悬停也会显示当前状态。

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

## 许可证

[MIT](LICENSE)

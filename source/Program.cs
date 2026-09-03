using System.ComponentModel;
using System.Drawing.Drawing2D;
using System.Globalization;
using System.Runtime.InteropServices;

namespace Caffeine.Windows;

internal enum AwakeMode
{
    Off,
    System,
    Display
}

[Flags]
internal enum ExecutionState : uint
{
    SystemRequired = 0x00000001,
    DisplayRequired = 0x00000002,
    Continuous = 0x80000000
}

internal static class Program
{
    [STAThread]
    private static int Main(string[] args)
    {
        if (args.Contains("--self-test", StringComparer.OrdinalIgnoreCase))
            return SelfTest();

        ApplicationConfiguration.Initialize();

        using var mutex = new Mutex(true, @"Local\Caffeine.Windows.43E867D2", out var firstInstance);
        if (!firstInstance)
        {
            MessageBox.Show(UiText.AlreadyRunning, "Caffeine",
                MessageBoxButtons.OK, MessageBoxIcon.Information);
            return 0;
        }

        using var context = new TrayApplicationContext();
        Application.Run(context);
        return 0;
    }

    private static int SelfTest()
    {
        if (PowerRequest.FlagsFor(AwakeMode.Off) != ExecutionState.Continuous ||
            PowerRequest.FlagsFor(AwakeMode.System) !=
                (ExecutionState.Continuous | ExecutionState.SystemRequired) ||
            PowerRequest.FlagsFor(AwakeMode.Display) !=
                (ExecutionState.Continuous | ExecutionState.SystemRequired | ExecutionState.DisplayRequired))
            return 1;

        if (!UiText.UsesChinese(CultureInfo.GetCultureInfo("zh-CN")) ||
            UiText.UsesChinese(CultureInfo.GetCultureInfo("en-US")))
            return 1;

        try
        {
            PowerRequest.Apply(AwakeMode.System);
            PowerRequest.Apply(AwakeMode.Display);
            PowerRequest.Apply(AwakeMode.Off);
            return 0;
        }
        finally
        {
            PowerRequest.TryClear();
        }
    }
}

internal sealed class TrayApplicationContext : ApplicationContext
{
    private readonly ContextMenuStrip _menu = new() { ShowImageMargin = false };
    private readonly ToolStripMenuItem _offItem = new(UiText.Off);
    private readonly ToolStripMenuItem _systemItem = new(UiText.PreventSleep);
    private readonly ToolStripMenuItem _displayItem = new(UiText.KeepDisplayOn);
    private readonly NotifyIcon _tray;
    private Icon _icon;
    private AwakeMode _mode = AwakeMode.Off;
    private AwakeMode _lastActiveMode = AwakeMode.System;

    public TrayApplicationContext()
    {
        _offItem.Click += (_, _) => SetMode(AwakeMode.Off);
        _systemItem.Click += (_, _) => SetMode(AwakeMode.System);
        _displayItem.Click += (_, _) => SetMode(AwakeMode.Display);

        var exitItem = new ToolStripMenuItem(UiText.Exit);
        exitItem.Click += (_, _) => ExitThread();
        _menu.Items.AddRange([
            _offItem,
            _systemItem,
            _displayItem,
            new ToolStripSeparator(),
            exitItem
        ]);

        _icon = CreateIcon(AwakeMode.Off);
        _tray = new NotifyIcon
        {
            ContextMenuStrip = _menu,
            Icon = _icon,
            Text = UiText.TrayOff,
            Visible = true
        };
        _tray.MouseClick += (_, eventArgs) =>
        {
            if (eventArgs.Button == MouseButtons.Left)
                SetMode(_mode == AwakeMode.Off ? _lastActiveMode : AwakeMode.Off);
        };

        UpdateUi();
    }

    private void SetMode(AwakeMode mode)
    {
        if (mode == _mode)
            return;

        try
        {
            PowerRequest.Apply(mode);
        }
        catch (Exception exception)
        {
            MessageBox.Show($"{UiText.ChangePowerFailed}\n\n{exception.Message}", "Caffeine",
                MessageBoxButtons.OK, MessageBoxIcon.Error);
            return;
        }

        _mode = mode;
        if (mode != AwakeMode.Off)
            _lastActiveMode = mode;

        UpdateUi();
    }

    private void UpdateUi()
    {
        _offItem.Checked = _mode == AwakeMode.Off;
        _systemItem.Checked = _mode == AwakeMode.System;
        _displayItem.Checked = _mode == AwakeMode.Display;

        _tray.Text = _mode switch
        {
            AwakeMode.System => UiText.TrayPreventSleep,
            AwakeMode.Display => UiText.TrayKeepDisplayOn,
            _ => UiText.TrayOff
        };

        var oldIcon = _icon;
        _icon = CreateIcon(_mode);
        _tray.Icon = _icon;
        oldIcon.Dispose();
    }

    protected override void ExitThreadCore()
    {
        if (_mode != AwakeMode.Off)
            PowerRequest.TryClear();

        _tray.Visible = false;
        base.ExitThreadCore();
    }

    protected override void Dispose(bool disposing)
    {
        if (disposing)
        {
            _tray.Dispose();
            _menu.Dispose();
            _icon.Dispose();
        }

        base.Dispose(disposing);
    }

    private static Icon CreateIcon(AwakeMode mode)
    {
        using var bitmap = new Bitmap(32, 32);
        using var graphics = Graphics.FromImage(bitmap);
        graphics.SmoothingMode = SmoothingMode.AntiAlias;
        graphics.Clear(Color.Transparent);

        var color = mode switch
        {
            AwakeMode.System => Color.FromArgb(245, 158, 11),
            AwakeMode.Display => Color.FromArgb(20, 184, 166),
            _ => Color.FromArgb(148, 163, 184)
        };

        using var cup = new GraphicsPath();
        cup.AddLine(6, 12, 23, 12);
        cup.AddLine(23, 12, 22, 21);
        cup.AddBezier(22, 21, 22, 24, 20, 25, 17, 25);
        cup.AddLine(17, 25, 12, 25);
        cup.AddBezier(12, 25, 9, 25, 7, 24, 7, 21);
        cup.AddLine(7, 21, 6, 12);
        cup.CloseFigure();

        using var outline = new Pen(Color.FromArgb(45, 55, 72), 2.2f)
        {
            LineJoin = LineJoin.Round,
            StartCap = LineCap.Round,
            EndCap = LineCap.Round
        };
        using var paint = new SolidBrush(mode == AwakeMode.Off ? Color.Transparent : color);
        using var accent = new Pen(color, 2.2f)
        {
            StartCap = LineCap.Round,
            EndCap = LineCap.Round
        };

        graphics.FillPath(paint, cup);
        graphics.DrawPath(mode == AwakeMode.Off ? accent : outline, cup);
        graphics.DrawArc(mode == AwakeMode.Off ? accent : outline, 20, 14, 8, 8, -90, 180);
        graphics.DrawLine(accent, 5, 28, 25, 28);

        if (mode != AwakeMode.Off)
        {
            graphics.DrawBezier(accent, 11, 9, 8, 7, 14, 5, 11, 3);
            graphics.DrawBezier(accent, 18, 9, 15, 7, 21, 5, 18, 3);
        }

        if (mode == AwakeMode.Display)
        {
            graphics.FillEllipse(Brushes.White, 25, 4, 4, 4);
            graphics.DrawEllipse(outline, 25, 4, 4, 4);
        }

        var handle = bitmap.GetHicon();
        using var borrowed = Icon.FromHandle(handle);
        var icon = (Icon)borrowed.Clone();
        NativeMethods.DestroyIcon(handle);
        return icon;
    }
}

internal static class UiText
{
    // ponytail: inline strings cover two languages; move to .resx when adding a third.
    private static bool Chinese => UsesChinese(CultureInfo.CurrentUICulture);

    internal static string AlreadyRunning => Chinese
        ? "Caffeine 已在运行，请查看任务栏通知区域。"
        : "Caffeine is already running. Check the notification area.";
    internal static string Off => Chinese ? "关闭" : "Off";
    internal static string PreventSleep => Chinese ? "仅防止电脑休眠" : "Prevent sleep";
    internal static string KeepDisplayOn => Chinese ? "防止休眠并保持屏幕开启" : "Prevent sleep and keep display on";
    internal static string Exit => Chinese ? "退出" : "Exit";
    internal static string TrayOff => Chinese ? "Caffeine：已关闭" : "Caffeine: off";
    internal static string TrayPreventSleep => Chinese
        ? "Caffeine：正在防止电脑休眠"
        : "Caffeine: preventing sleep";
    internal static string TrayKeepDisplayOn => Chinese
        ? "Caffeine：正在防止休眠并保持屏幕开启"
        : "Caffeine: preventing sleep and keeping display on";
    internal static string ChangePowerFailed => Chinese ? "无法更改电源状态。" : "Unable to change the power state.";
    internal static string NativeCallFailed => Chinese
        ? "SetThreadExecutionState 调用失败。"
        : "SetThreadExecutionState failed.";

    internal static bool UsesChinese(CultureInfo culture) => culture.TwoLetterISOLanguageName == "zh";
}

internal static class PowerRequest
{
    internal static ExecutionState FlagsFor(AwakeMode mode) => mode switch
    {
        AwakeMode.System => ExecutionState.Continuous | ExecutionState.SystemRequired,
        AwakeMode.Display => ExecutionState.Continuous | ExecutionState.SystemRequired |
                             ExecutionState.DisplayRequired,
        _ => ExecutionState.Continuous
    };

    internal static void Apply(AwakeMode mode)
    {
        if (NativeMethods.SetThreadExecutionState(FlagsFor(mode)) != 0)
            return;

        var error = Marshal.GetLastWin32Error();
        throw error == 0
            ? new InvalidOperationException(UiText.NativeCallFailed)
            : new Win32Exception(error);
    }

    internal static void TryClear() => NativeMethods.SetThreadExecutionState(ExecutionState.Continuous);
}

internal static partial class NativeMethods
{
    [DllImport("kernel32.dll", SetLastError = true)]
    internal static extern ExecutionState SetThreadExecutionState(ExecutionState executionState);

    [DllImport("user32.dll")]
    [return: MarshalAs(UnmanagedType.Bool)]
    internal static extern bool DestroyIcon(nint iconHandle);
}

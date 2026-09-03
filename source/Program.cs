using System.ComponentModel;
using System.Drawing.Drawing2D;
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
            MessageBox.Show("Caffeine 已在运行，请查看任务栏通知区域。", "Caffeine",
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
    private readonly ToolStripMenuItem _offItem = new("关闭");
    private readonly ToolStripMenuItem _systemItem = new("仅防止电脑休眠");
    private readonly ToolStripMenuItem _displayItem = new("防止休眠并保持屏幕开启");
    private readonly NotifyIcon _tray;
    private Icon _icon;
    private AwakeMode _mode = AwakeMode.Off;
    private AwakeMode _lastActiveMode = AwakeMode.System;

    public TrayApplicationContext()
    {
        _offItem.Click += (_, _) => SetMode(AwakeMode.Off);
        _systemItem.Click += (_, _) => SetMode(AwakeMode.System);
        _displayItem.Click += (_, _) => SetMode(AwakeMode.Display);

        var exitItem = new ToolStripMenuItem("退出");
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
            Text = "Caffeine：已关闭",
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
            MessageBox.Show($"无法更改电源状态。\n\n{exception.Message}", "Caffeine",
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
            AwakeMode.System => "Caffeine：正在防止电脑休眠",
            AwakeMode.Display => "Caffeine：正在防止休眠并保持屏幕开启",
            _ => "Caffeine：已关闭"
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
            ? new InvalidOperationException("SetThreadExecutionState 调用失败。")
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

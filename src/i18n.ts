import type { AwakeMode, Locale } from "./model";

export const messages = {
  en: {
    status: { off: "Off", system: "System awake", display: "Screen awake" },
    modes: {
      off: "Allow sleep",
      system: "Keep system awake",
      display: "Keep screen on",
    },
    help: {
      off: "Follow your usual power settings.",
      system: "Prevent idle system sleep. The display can turn off.",
      display: "Prevent idle sleep for both the system and display.",
    },
    language: "Language",
    automatic: "System",
    modeLabel: "Awake mode",
    running: "Keeps running when closed",
    details:
      "Only prevents idle sleep. Manual sleep, closing the lid and security locking remain under system control.",
    quit: "Quit",
    retry: "Retry",
    failed: "Could not apply the change.",
    unavailable: "Open the desktop app to control power.",
    preview: "Interface preview · power controls unavailable",
    loading: "Connecting…",
    reconnect: "Connection lost. Reopen the panel or retry.",
    saveWarning: "Your preference could not be saved.",
  },
  zh: {
    status: { off: "已关闭", system: "正在防止休眠", display: "屏幕保持常亮" },
    modes: { off: "允许休眠", system: "仅防休眠", display: "屏幕常亮" },
    help: {
      off: "遵循系统原有电源设置。",
      system: "阻止因空闲自动休眠，允许屏幕关闭。",
      display: "同时阻止电脑和屏幕因空闲自动休眠。",
    },
    language: "语言",
    automatic: "跟随系统",
    modeLabel: "唤醒模式",
    running: "关闭窗口后继续运行",
    details: "仅阻止空闲休眠。手动休眠、合盖和安全锁屏仍由系统控制。",
    quit: "退出",
    retry: "重试",
    failed: "无法应用更改。",
    unavailable: "请在桌面应用中控制电源。",
    preview: "界面预览 · 无法控制系统电源",
    loading: "正在连接…",
    reconnect: "连接已断开，请重新打开面板或重试。",
    saveWarning: "偏好设置未能保存。",
  },
} satisfies Record<
  Locale,
  {
    status: Record<AwakeMode, string>;
    modes: Record<AwakeMode, string>;
    help: Record<AwakeMode, string>;
  } & Record<string, unknown>
>;

export function systemLocale(): Locale {
  return navigator.language.toLowerCase().startsWith("zh") ? "zh" : "en";
}

export type AwakeMode = "off" | "system" | "display";
export type Language = "auto" | "en" | "zh";
export type Locale = "en" | "zh";

export interface AppSnapshot {
  mode: AwakeMode;
  lastActiveMode: AwakeMode;
  language: Language;
  locale: Locale;
  platform: "macos" | "windows";
  version: string;
  revision: number;
  warning: string | null;
  error: string | null;
}

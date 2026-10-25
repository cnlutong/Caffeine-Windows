import { invoke, isTauri } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { AppSnapshot, AwakeMode, Language } from "./model";

export const desktop = isTauri();

export const getState = () => invoke<AppSnapshot>("get_state");
export const setMode = (mode: AwakeMode) =>
  invoke<AppSnapshot>("set_mode", { mode });
export const setLanguage = (language: Language) =>
  invoke<AppSnapshot>("set_language", { language });
export const quit = () => invoke<void>("quit_app");
export const onStateChanged = (callback: (state: AppSnapshot) => void) =>
  listen<AppSnapshot>("state-changed", (event) => callback(event.payload));

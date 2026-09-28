import { create } from "zustand";
import { persist } from "zustand/middleware";

import type { SupportedLanguage } from "../i18n";

interface SettingsState {
  language: SupportedLanguage;
  setLanguage: (language: SupportedLanguage) => void;
}

export const SETTINGS_STORAGE_KEY = "pdf-toolkit:settings";

export const useSettingsStore = create<SettingsState>()(
  persist(
    (set) => ({
      language: "en",
      setLanguage: (language) => set({ language }),
    }),
    {
      name: SETTINGS_STORAGE_KEY,
    },
  ),
);

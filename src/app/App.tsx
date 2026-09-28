import { useEffect } from "react";

import { AppShell } from "./AppShell";
import { changeLanguage } from "../i18n/changeLanguage";
import { applyDocumentDirection } from "../i18n/direction";
import { useSettingsStore } from "../stores/settingsStore";

export default function App() {
  const language = useSettingsStore((state) => state.language);

  useEffect(() => {
    applyDocumentDirection(language);
    void changeLanguage(language);
  }, [language]);

  return <AppShell />;
}

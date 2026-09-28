import { useTranslation } from "react-i18next";

import { Sidebar } from "../components/common/Sidebar";
import { LanguageSwitcher } from "../components/common/LanguageSwitcher";
import { HomeView } from "../features/home/HomeView";
import { SettingsView } from "../features/settings/SettingsView";
import { ToolView } from "../tools/ToolView";
import { useNavigationStore } from "../stores/navigationStore";

function useViewTitle(): string {
  const { t } = useTranslation();
  const view = useNavigationStore((state) => state.view);

  switch (view.type) {
    case "home":
      return t("home.title");
    case "settings":
      return t("settings.title");
    case "tool":
      return t(`tools.${view.toolId}.title`);
  }
}

export function AppShell() {
  const view = useNavigationStore((state) => state.view);
  const title = useViewTitle();

  return (
    <div className="flex h-full w-full overflow-hidden">
      <Sidebar />

      <div className="flex min-w-0 flex-1 flex-col">
        <header className="flex items-center justify-between gap-4 border-b border-slate-200 bg-white px-6 py-3">
          <h1 className="truncate text-lg font-semibold text-slate-800">{title}</h1>
          <LanguageSwitcher />
        </header>

        <main className="flex-1 overflow-y-auto px-6 py-6">
          {view.type === "home" && <HomeView />}
          {view.type === "settings" && <SettingsView />}
          {view.type === "tool" && <ToolView toolId={view.toolId} />}
        </main>
      </div>
    </div>
  );
}

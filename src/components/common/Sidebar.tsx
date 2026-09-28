import { useTranslation } from "react-i18next";
import { FileText, Home, Settings } from "lucide-react";

import { useNavigationStore } from "../../stores/navigationStore";
import { TOOL_CATEGORIES, getToolsByCategory } from "../../types/tools";

export function Sidebar() {
  const { t } = useTranslation();
  const view = useNavigationStore((state) => state.view);
  const goHome = useNavigationStore((state) => state.goHome);
  const goSettings = useNavigationStore((state) => state.goSettings);
  const openTool = useNavigationStore((state) => state.openTool);

  const navButtonClass = (active: boolean) =>
    [
      "flex w-full items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors",
      active
        ? "bg-indigo-50 text-indigo-700"
        : "text-slate-600 hover:bg-slate-100 hover:text-slate-900",
    ].join(" ");

  const toolButtonClass = (active: boolean) =>
    [
      "flex w-full items-center gap-2 rounded-lg px-3 py-1.5 text-sm transition-colors",
      active
        ? "bg-indigo-50 text-indigo-700"
        : "text-slate-600 hover:bg-slate-100 hover:text-slate-900",
    ].join(" ");

  return (
    <aside className="flex w-64 shrink-0 flex-col border-e border-slate-200 bg-white">
      <div className="flex items-center gap-2 border-b border-slate-200 px-4 py-4">
        <span className="flex h-9 w-9 items-center justify-center rounded-lg bg-indigo-600 text-white">
          <FileText className="h-5 w-5" aria-hidden="true" />
        </span>
        <span className="truncate font-semibold text-slate-800">{t("app.name")}</span>
      </div>

      <nav className="min-h-0 flex-1 overflow-y-auto px-3 py-4">
        <div className="space-y-1">
          <button type="button" onClick={goHome} className={navButtonClass(view.type === "home")}>
            <Home className="h-4 w-4" aria-hidden="true" />
            {t("nav.home")}
          </button>
          <button
            type="button"
            onClick={goSettings}
            className={navButtonClass(view.type === "settings")}
          >
            <Settings className="h-4 w-4" aria-hidden="true" />
            {t("nav.settings")}
          </button>
        </div>

        <div className="mt-6 space-y-4">
          {TOOL_CATEGORIES.map((category) => (
            <section key={category}>
              <h2 className="px-3 text-xs font-semibold uppercase tracking-wide text-slate-400">
                {t(`categories.${category}`)}
              </h2>
              <div className="mt-1 space-y-0.5">
                {getToolsByCategory(category).map((tool) => {
                  const Icon = tool.icon;
                  const active = view.type === "tool" && view.toolId === tool.id;
                  return (
                    <button
                      key={tool.id}
                      type="button"
                      onClick={() => openTool(tool.id)}
                      className={toolButtonClass(active)}
                    >
                      <Icon className="h-4 w-4 shrink-0" aria-hidden="true" />
                      <span className="truncate">{t(`tools.${tool.id}.title`)}</span>
                    </button>
                  );
                })}
              </div>
            </section>
          ))}
        </div>
      </nav>

      <div className="border-t border-slate-200 px-4 py-3">
        <p className="text-xs text-slate-400">{t("home.privacy")}</p>
      </div>
    </aside>
  );
}

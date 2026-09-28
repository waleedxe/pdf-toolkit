import { useTranslation } from "react-i18next";
import { ShieldCheck } from "lucide-react";

import { TOOL_CATEGORIES, getToolsByCategory } from "../../types/tools";
import { ToolCard } from "../../components/common/ToolCard";

export function HomeView() {
  const { t } = useTranslation();

  return (
    <div className="mx-auto max-w-5xl">
      <div className="mb-6 flex items-start gap-3 rounded-xl border border-emerald-200 bg-emerald-50 p-4">
        <ShieldCheck className="mt-0.5 h-5 w-5 shrink-0 text-emerald-600" aria-hidden="true" />
        <div>
          <p className="font-medium text-emerald-800">{t("home.subtitle")}</p>
          <p className="text-sm text-emerald-700">{t("home.privacy")}</p>
        </div>
      </div>

      {TOOL_CATEGORIES.map((category) => (
        <section key={category} className="mb-8">
          <h2 className="mb-3 text-sm font-semibold uppercase tracking-wide text-slate-400">
            {t(`categories.${category}`)}
          </h2>
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
            {getToolsByCategory(category).map((tool) => (
              <ToolCard key={tool.id} tool={tool} />
            ))}
          </div>
        </section>
      ))}
    </div>
  );
}

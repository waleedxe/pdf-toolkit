import { useTranslation } from "react-i18next";

import type { ToolMeta } from "../../types/tools";
import { useNavigationStore } from "../../stores/navigationStore";

interface ToolCardProps {
  tool: ToolMeta;
}

export function ToolCard({ tool }: ToolCardProps) {
  const { t } = useTranslation();
  const openTool = useNavigationStore((state) => state.openTool);
  const Icon = tool.icon;

  return (
    <button
      type="button"
      onClick={() => openTool(tool.id)}
      className="group flex flex-col items-start gap-3 rounded-xl border border-slate-200 bg-white p-4 text-start shadow-sm transition-all hover:-translate-y-0.5 hover:border-indigo-300 hover:shadow-md focus:outline-none focus-visible:ring-2 focus-visible:ring-indigo-500"
    >
      <span className="flex h-10 w-10 items-center justify-center rounded-lg bg-indigo-50 text-indigo-600 transition-colors group-hover:bg-indigo-600 group-hover:text-white">
        <Icon className="h-5 w-5" aria-hidden="true" />
      </span>
      <span className="font-medium text-slate-800">{t(`tools.${tool.id}.title`)}</span>
      <span className="text-sm leading-relaxed text-slate-500">
        {t(`tools.${tool.id}.description`)}
      </span>
    </button>
  );
}

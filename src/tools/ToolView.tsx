import type { ComponentType } from "react";
import { useTranslation } from "react-i18next";
import { ArrowLeft, ArrowRight, Construction } from "lucide-react";

import { getToolMeta, type ToolId } from "../types/tools";
import { useNavigationStore } from "../stores/navigationStore";
import { MergeTool } from "./merge/MergeTool";
import { SplitTool } from "./split/SplitTool";
import { ExtractTool } from "./extract/ExtractTool";
import { RemoveTool } from "./remove/RemoveTool";
import { RotateTool } from "./rotate/RotateTool";

/** Tools that are implemented so far (Phase 1). */
const IMPLEMENTED: Partial<Record<ToolId, ComponentType>> = {
  merge: MergeTool,
  split: SplitTool,
  extract: ExtractTool,
  remove: RemoveTool,
  rotate: RotateTool,
};

interface ToolViewProps {
  toolId: ToolId;
}

/**
 * Shared tool page shell.
 *
 * Tools that are still being built fall back to a placeholder so the catalog
 * is always navigable.
 */
export function ToolView({ toolId }: ToolViewProps) {
  const { t } = useTranslation();
  const goHome = useNavigationStore((state) => state.goHome);
  const meta = getToolMeta(toolId);
  const Icon = meta.icon;
  const Tool = IMPLEMENTED[toolId];

  return (
    <div className="mx-auto max-w-3xl">
      <button
        type="button"
        onClick={goHome}
        className="mb-4 inline-flex items-center gap-2 text-sm font-medium text-slate-500 hover:text-slate-800"
      >
        {/* Mirror the back arrow for RTL layouts. */}
        <ArrowLeft className="h-4 w-4 rtl:hidden" aria-hidden="true" />
        <ArrowRight className="hidden h-4 w-4 rtl:block" aria-hidden="true" />
        {t("tool.back")}
      </button>

      <div className="mb-6 flex items-center gap-4 rounded-xl border border-slate-200 bg-white p-6">
        <span className="flex h-12 w-12 items-center justify-center rounded-lg bg-indigo-50 text-indigo-600">
          <Icon className="h-6 w-6" aria-hidden="true" />
        </span>
        <div>
          <h2 className="text-lg font-semibold text-slate-800">{t(`tools.${toolId}.title`)}</h2>
          <p className="text-sm text-slate-500">{t(`tools.${toolId}.description`)}</p>
        </div>
      </div>

      {Tool ? (
        <Tool />
      ) : (
        <div className="flex items-start gap-3 rounded-xl border border-dashed border-slate-300 bg-white p-6 text-slate-500">
          <Construction className="mt-0.5 h-5 w-5 shrink-0 text-amber-500" aria-hidden="true" />
          <p className="text-sm">{t("tool.comingSoon")}</p>
        </div>
      )}
    </div>
  );
}

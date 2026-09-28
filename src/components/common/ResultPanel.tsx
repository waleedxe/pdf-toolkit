import { FolderOpen, RotateCcw, SquareArrowOutUpRight } from "lucide-react";
import { useTranslation } from "react-i18next";

import { openFile, revealPath } from "../../lib/dialogs";
import { baseName } from "../../lib/paths";

interface ResultPanelProps {
  /** Absolute paths of the files the tool produced. */
  outputs: string[];
  /** Clear the result and let the user run the tool again. */
  onReset: () => void;
}

/** Success state shared by every tool: open, reveal, or start over. */
export function ResultPanel({ outputs, onReset }: ResultPanelProps) {
  const { t } = useTranslation();
  const first = outputs[0];

  const iconClass = "h-4 w-4";
  const buttonClass =
    "inline-flex items-center gap-2 rounded-lg px-3 py-2 text-sm font-medium transition-colors";

  return (
    <div className="rounded-xl border border-emerald-200 bg-emerald-50 p-5">
      <p className="font-medium text-emerald-800">
        {outputs.length === 1
          ? t("toolUi.successOne")
          : t("toolUi.successMany", { count: outputs.length })}
      </p>

      <ul className="mt-2 space-y-1">
        {outputs.map((path) => (
          <li key={path} className="truncate text-sm text-emerald-700" title={path}>
            {baseName(path)}
          </li>
        ))}
      </ul>

      <div className="mt-4 flex flex-wrap gap-2">
        {first && (
          <button
            type="button"
            onClick={() => void openFile(first)}
            className={`${buttonClass} bg-emerald-600 text-white hover:bg-emerald-700`}
          >
            <SquareArrowOutUpRight className={iconClass} aria-hidden="true" />
            {t("toolUi.openFile")}
          </button>
        )}
        {first && (
          <button
            type="button"
            onClick={() => void revealPath(first)}
            className={`${buttonClass} border border-emerald-300 bg-white text-emerald-800 hover:bg-emerald-100`}
          >
            <FolderOpen className={iconClass} aria-hidden="true" />
            {t("toolUi.openFolder")}
          </button>
        )}
        <button
          type="button"
          onClick={onReset}
          className={`${buttonClass} text-emerald-800 hover:bg-emerald-100`}
        >
          <RotateCcw className={iconClass} aria-hidden="true" />
          {t("toolUi.again")}
        </button>
      </div>
    </div>
  );
}

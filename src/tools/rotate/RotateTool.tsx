import { useState } from "react";
import { useTranslation } from "react-i18next";

import { FileDropZone } from "../../components/file-drop/FileDropZone";
import { ResultPanel } from "../../components/common/ResultPanel";
import {
  ErrorBanner,
  OutputPicker,
  PrimaryButton,
  SelectedFileRow,
  ToolSection,
} from "../../components/common/ToolControls";
import { PDF_EXTENSIONS } from "../../lib/dialogs";
import { invokeCommand } from "../../lib/tauri";
import { usePdfTool, type PdfOperationResult } from "../usePdfTool";

const ANGLES = [90, 180, 270] as const;
type Angle = (typeof ANGLES)[number];

export function RotateTool() {
  const { t } = useTranslation();
  const { input, output, job, selectInput, clearInput, chooseOutput } = usePdfTool();
  const [rotation, setRotation] = useState<Angle>(90);
  const [pages, setPages] = useState("");
  const [allPages, setAllPages] = useState(true);

  const canRun = input !== null && output !== null && (allPages || pages.trim().length > 0);

  const run = (): void => {
    if (!input || !output) return;
    void job.run(() =>
      invokeCommand<PdfOperationResult>("rotate_pages", {
        request: {
          input,
          pages: allPages ? null : pages,
          rotation,
          output,
        },
      }),
    );
  };

  return (
    <div className="mx-auto max-w-3xl space-y-6">
      <ToolSection title={t("toolUi.input")}>
        {input ? (
          <SelectedFileRow path={input} onClear={clearInput} />
        ) : (
          <FileDropZone
            onFiles={selectInput}
            accept={PDF_EXTENSIONS}
            multiple={false}
            hint={t("tools.rotate.description")}
          />
        )}
      </ToolSection>

      <ToolSection title={t("toolUi.rotation")}>
        <div className="flex flex-wrap gap-2">
          {ANGLES.map((angle) => (
            <button
              key={angle}
              type="button"
              onClick={() => setRotation(angle)}
              aria-pressed={rotation === angle}
              className={[
                "rounded-lg border px-4 py-2 text-sm font-medium transition-colors",
                rotation === angle
                  ? "border-indigo-300 bg-indigo-50 text-indigo-700"
                  : "border-slate-300 bg-white text-slate-700 hover:bg-slate-50",
              ].join(" ")}
            >
              {t("toolUi.angle", { angle })}
            </button>
          ))}
        </div>

        <label className="mt-4 flex items-center gap-2 text-sm text-slate-700">
          <input
            type="checkbox"
            checked={allPages}
            onChange={(event) => setAllPages(event.target.checked)}
            className="h-4 w-4 rounded border-slate-300"
          />
          {t("toolUi.allPages")}
        </label>

        {!allPages && (
          <input
            value={pages}
            onChange={(event) => setPages(event.target.value)}
            placeholder={t("toolUi.pagesPlaceholder")}
            className="mt-3 w-full rounded-lg border border-slate-300 px-3 py-2 text-sm text-slate-800 focus:border-indigo-400 focus:outline-none"
          />
        )}
      </ToolSection>

      <ToolSection title={t("toolUi.output")}>
        <OutputPicker path={output} onChoose={() => void chooseOutput("rotated")} />
      </ToolSection>

      {job.error && <ErrorBanner message={job.error} />}
      {job.result && <ResultPanel outputs={[job.result.output]} onReset={job.reset} />}

      <PrimaryButton
        onClick={run}
        disabled={!canRun || job.status === "success"}
        running={job.status === "running"}
        label={t("tools.rotate.title")}
      />
    </div>
  );
}

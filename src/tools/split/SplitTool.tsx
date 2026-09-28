import { useState } from "react";
import { useTranslation } from "react-i18next";

import { FileDropZone } from "../../components/file-drop/FileDropZone";
import { ResultPanel } from "../../components/common/ResultPanel";
import {
  ErrorBanner,
  PrimaryButton,
  SelectedFileRow,
  ToolSection,
} from "../../components/common/ToolControls";
import { PDF_EXTENSIONS, pickDirectory } from "../../lib/dialogs";
import { fileNameWithoutExtension } from "../../lib/paths";
import { invokeCommand } from "../../lib/tauri";
import { useToolJob } from "../../hooks/useToolJob";

type SplitMode = "every-page" | "ranges";

interface SplitResult {
  outputs: string[];
  page_count: number;
}

export function SplitTool() {
  const { t } = useTranslation();
  const [input, setInput] = useState<string | null>(null);
  const [mode, setMode] = useState<SplitMode>("every-page");
  const [pages, setPages] = useState("");
  const [outputDir, setOutputDir] = useState<string | null>(null);
  const [baseName, setBaseName] = useState("");
  const job = useToolJob<SplitResult>();

  const selectInput = (paths: string[]): void => {
    const first = paths[0];
    if (!first) return;
    setInput(first);
    setBaseName(`${fileNameWithoutExtension(first)}_split`);
    job.reset();
  };

  const clearInput = (): void => {
    setInput(null);
    setOutputDir(null);
    job.reset();
  };

  const chooseDirectory = async (): Promise<void> => {
    const chosen = await pickDirectory();
    if (chosen) setOutputDir(chosen);
  };

  const canRun =
    input !== null &&
    outputDir !== null &&
    baseName.trim().length > 0 &&
    (mode === "every-page" || pages.trim().length > 0);

  const run = (): void => {
    if (!input || !outputDir) return;
    void job.run(() =>
      invokeCommand<SplitResult>("split_pdf", {
        request: {
          input,
          mode,
          pages: mode === "ranges" ? pages : null,
          output_dir: outputDir,
          base_name: baseName,
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
            hint={t("tools.split.description")}
          />
        )}
      </ToolSection>

      <ToolSection title={t("toolUi.splitMode")}>
        <div className="flex flex-wrap gap-2">
          {(["every-page", "ranges"] as const).map((value) => (
            <button
              key={value}
              type="button"
              onClick={() => setMode(value)}
              aria-pressed={mode === value}
              className={[
                "rounded-lg border px-4 py-2 text-sm font-medium transition-colors",
                mode === value
                  ? "border-indigo-300 bg-indigo-50 text-indigo-700"
                  : "border-slate-300 bg-white text-slate-700 hover:bg-slate-50",
              ].join(" ")}
            >
              {value === "every-page" ? t("toolUi.modeEveryPage") : t("toolUi.modeRanges")}
            </button>
          ))}
        </div>

        {mode === "ranges" && (
          <input
            value={pages}
            onChange={(event) => setPages(event.target.value)}
            placeholder={t("toolUi.pagesPlaceholder")}
            className="mt-4 w-full rounded-lg border border-slate-300 px-3 py-2 text-sm text-slate-800 focus:border-indigo-400 focus:outline-none"
          />
        )}
      </ToolSection>

      <ToolSection title={t("toolUi.output")}>
        <div className="space-y-3">
          <button
            type="button"
            onClick={() => void chooseDirectory()}
            className="inline-flex items-center gap-2 rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm font-medium text-slate-700 hover:bg-slate-50"
          >
            {t("toolUi.chooseFolder")}
          </button>
          {outputDir && (
            <p className="truncate text-sm text-slate-600" title={outputDir}>
              {outputDir}
            </p>
          )}
          <input
            value={baseName}
            onChange={(event) => setBaseName(event.target.value)}
            placeholder={t("toolUi.baseName")}
            className="w-full rounded-lg border border-slate-300 px-3 py-2 text-sm text-slate-800 focus:border-indigo-400 focus:outline-none"
          />
        </div>
      </ToolSection>

      {job.error && <ErrorBanner message={job.error} />}
      {job.result && <ResultPanel outputs={job.result.outputs} onReset={job.reset} />}

      <PrimaryButton
        onClick={run}
        disabled={!canRun || job.status === "success"}
        running={job.status === "running"}
        label={t("tools.split.title")}
      />
    </div>
  );
}

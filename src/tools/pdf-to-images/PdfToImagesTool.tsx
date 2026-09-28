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

const FORMATS = ["png", "jpg"] as const;
type Format = (typeof FORMATS)[number];

interface ImagesResult {
  outputs: string[];
  page_count: number;
}

export function PdfToImagesTool() {
  const { t } = useTranslation();
  const [input, setInput] = useState<string | null>(null);
  const [pages, setPages] = useState("");
  const [allPages, setAllPages] = useState(true);
  const [dpi, setDpi] = useState(150);
  const [format, setFormat] = useState<Format>("png");
  const [quality, setQuality] = useState(85);
  const [outputDir, setOutputDir] = useState<string | null>(null);
  const [baseName, setBaseName] = useState("");
  const job = useToolJob<ImagesResult>();

  const selectInput = (paths: string[]): void => {
    const first = paths[0];
    if (!first) return;
    setInput(first);
    setBaseName(`${fileNameWithoutExtension(first)}_pages`);
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
    (allPages || pages.trim().length > 0);

  const run = (): void => {
    if (!input || !outputDir) return;
    void job.run(() =>
      invokeCommand<ImagesResult>("pdf_to_images", {
        request: {
          input,
          pages: allPages ? null : pages,
          dpi,
          format,
          quality,
          output_dir: outputDir,
          base_name: baseName,
        },
      }),
    );
  };

  const fieldClass =
    "w-full rounded-lg border border-slate-300 px-3 py-2 text-sm text-slate-800 focus:border-indigo-400 focus:outline-none";

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
            hint={t("tools.pdf-to-images.description")}
          />
        )}
      </ToolSection>

      <ToolSection title={t("toolUi.options")}>
        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
          <label className="text-sm text-slate-700">
            {t("toolUi.imageFormat")}
            <select
              value={format}
              onChange={(event) => setFormat(event.target.value as Format)}
              className={`mt-1 ${fieldClass}`}
            >
              {FORMATS.map((value) => (
                <option key={value} value={value}>
                  {value.toUpperCase()}
                </option>
              ))}
            </select>
          </label>

          <label className="text-sm text-slate-700">
            {t("toolUi.dpi")}
            <input
              type="number"
              min={36}
              max={600}
              value={dpi}
              onChange={(event) => setDpi(Number(event.target.value))}
              className={`mt-1 ${fieldClass}`}
            />
          </label>

          {format === "jpg" && (
            <label className="text-sm text-slate-700">
              {t("toolUi.quality")} ({quality})
              <input
                type="range"
                min={10}
                max={100}
                value={quality}
                onChange={(event) => setQuality(Number(event.target.value))}
                className="mt-2 w-full"
              />
            </label>
          )}
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
            className={`mt-3 ${fieldClass}`}
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
            className={fieldClass}
          />
        </div>
      </ToolSection>

      {job.error && <ErrorBanner message={job.error} />}
      {job.result && <ResultPanel outputs={job.result.outputs} onReset={job.reset} />}

      <PrimaryButton
        onClick={run}
        disabled={!canRun || job.status === "success"}
        running={job.status === "running"}
        label={t("tools.pdf-to-images.title")}
      />
    </div>
  );
}

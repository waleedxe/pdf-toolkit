import { useState } from "react";
import { ArrowDown, ArrowUp, ImagePlus, X } from "lucide-react";
import { useTranslation } from "react-i18next";

import { FileDropZone } from "../../components/file-drop/FileDropZone";
import { ResultPanel } from "../../components/common/ResultPanel";
import {
  ErrorBanner,
  OutputPicker,
  PrimaryButton,
  ToolSection,
} from "../../components/common/ToolControls";
import { IMAGE_EXTENSIONS, pickFiles, pickOutputFile } from "../../lib/dialogs";
import { baseName, dirName, joinPath } from "../../lib/paths";
import { invokeCommand } from "../../lib/tauri";
import { useToolJob } from "../../hooks/useToolJob";

const PAGE_SIZES = ["a4", "letter", "fit"] as const;
const ORIENTATIONS = ["auto", "portrait", "landscape"] as const;
const FITS = ["contain", "cover", "stretch"] as const;

type PageSize = (typeof PAGE_SIZES)[number];
type Orientation = (typeof ORIENTATIONS)[number];
type Fit = (typeof FITS)[number];

interface OperationResult {
  output: string;
  page_count: number;
}

export function ImagesToPdfTool() {
  const { t } = useTranslation();
  const [images, setImages] = useState<string[]>([]);
  const [output, setOutput] = useState<string | null>(null);
  const [pageSize, setPageSize] = useState<PageSize>("a4");
  const [orientation, setOrientation] = useState<Orientation>("auto");
  const [fit, setFit] = useState<Fit>("contain");
  const [margin, setMargin] = useState(0);
  const job = useToolJob<OperationResult>();

  const addImages = (paths: string[]): void => {
    setImages((current) => {
      const merged = [...current];
      for (const path of paths) if (!merged.includes(path)) merged.push(path);
      return merged;
    });
    job.reset();
  };

  const removeAt = (index: number): void => {
    setImages((current) => current.filter((_, i) => i !== index));
    job.reset();
  };

  const move = (index: number, offset: number): void => {
    setImages((current) => {
      const target = index + offset;
      if (target < 0 || target >= current.length) return current;
      const next = [...current];
      const [item] = next.splice(index, 1);
      if (item === undefined) return current;
      next.splice(target, 0, item);
      return next;
    });
  };

  const chooseOutput = async (): Promise<void> => {
    const first = images[0];
    const suggested = first ? joinPath(dirName(first), "images.pdf") : "images.pdf";
    const chosen = await pickOutputFile(suggested);
    if (chosen) setOutput(chosen);
  };

  const canRun = images.length > 0 && output !== null;

  const run = (): void => {
    if (!output) return;
    void job.run(() =>
      invokeCommand<OperationResult>("images_to_pdf", {
        request: { images, output, page_size: pageSize, orientation, fit, margin },
      }),
    );
  };

  const fieldClass =
    "w-full rounded-lg border border-slate-300 px-3 py-2 text-sm text-slate-800 focus:border-indigo-400 focus:outline-none";

  return (
    <div className="mx-auto max-w-3xl space-y-6">
      <ToolSection title={t("toolUi.input")}>
        {images.length === 0 ? (
          <FileDropZone
            onFiles={addImages}
            accept={IMAGE_EXTENSIONS}
            multiple
            hint={t("tools.images-to-pdf.description")}
          />
        ) : (
          <div className="space-y-2">
            <ul className="space-y-1">
              {images.map((path, index) => (
                <li
                  key={path}
                  className="flex items-center gap-2 rounded-lg border border-slate-200 bg-white px-3 py-2"
                >
                  <ImagePlus className="h-4 w-4 shrink-0 text-slate-300" aria-hidden="true" />
                  <span className="min-w-0 flex-1 truncate text-sm text-slate-700" title={path}>
                    {baseName(path)}
                  </span>
                  <button
                    type="button"
                    onClick={() => move(index, -1)}
                    disabled={index === 0}
                    aria-label={t("toolUi.moveUp")}
                    className="text-slate-400 hover:text-slate-700 disabled:opacity-30"
                  >
                    <ArrowUp className="h-4 w-4" aria-hidden="true" />
                  </button>
                  <button
                    type="button"
                    onClick={() => move(index, 1)}
                    disabled={index === images.length - 1}
                    aria-label={t("toolUi.moveDown")}
                    className="text-slate-400 hover:text-slate-700 disabled:opacity-30"
                  >
                    <ArrowDown className="h-4 w-4" aria-hidden="true" />
                  </button>
                  <button
                    type="button"
                    onClick={() => removeAt(index)}
                    aria-label={t("toolUi.remove")}
                    className="text-slate-400 hover:text-rose-600"
                  >
                    <X className="h-4 w-4" aria-hidden="true" />
                  </button>
                </li>
              ))}
            </ul>
            <button
              type="button"
              onClick={() => {
                void (async () => {
                  const paths = await pickFiles(IMAGE_EXTENSIONS, true);
                  if (paths.length > 0) addImages(paths);
                })();
              }}
              className="text-sm font-medium text-indigo-600 hover:text-indigo-800"
            >
              {t("toolUi.addMore")}
            </button>
          </div>
        )}
      </ToolSection>

      <ToolSection title={t("toolUi.options")}>
        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
          <label className="text-sm text-slate-700">
            {t("toolUi.pageSize")}
            <select
              value={pageSize}
              onChange={(event) => setPageSize(event.target.value as PageSize)}
              className={`mt-1 ${fieldClass}`}
            >
              {PAGE_SIZES.map((value) => (
                <option key={value} value={value}>
                  {t(`toolUi.pageSizeOption.${value}`)}
                </option>
              ))}
            </select>
          </label>

          <label className="text-sm text-slate-700">
            {t("toolUi.orientation")}
            <select
              value={orientation}
              onChange={(event) => setOrientation(event.target.value as Orientation)}
              className={`mt-1 ${fieldClass}`}
            >
              {ORIENTATIONS.map((value) => (
                <option key={value} value={value}>
                  {t(`toolUi.orientationOption.${value}`)}
                </option>
              ))}
            </select>
          </label>

          <label className="text-sm text-slate-700">
            {t("toolUi.fitMode")}
            <select
              value={fit}
              onChange={(event) => setFit(event.target.value as Fit)}
              className={`mt-1 ${fieldClass}`}
            >
              {FITS.map((value) => (
                <option key={value} value={value}>
                  {t(`toolUi.fitOption.${value}`)}
                </option>
              ))}
            </select>
          </label>

          <label className="text-sm text-slate-700">
            {t("toolUi.margin")}
            <input
              type="number"
              min={0}
              max={200}
              value={margin}
              onChange={(event) => setMargin(Number(event.target.value))}
              className={`mt-1 ${fieldClass}`}
            />
          </label>
        </div>
      </ToolSection>

      <ToolSection title={t("toolUi.output")}>
        <OutputPicker path={output} onChoose={() => void chooseOutput()} />
      </ToolSection>

      {job.error && <ErrorBanner message={job.error} />}
      {job.result && <ResultPanel outputs={[job.result.output]} onReset={job.reset} />}

      <PrimaryButton
        onClick={run}
        disabled={!canRun || job.status === "success"}
        running={job.status === "running"}
        label={t("tools.images-to-pdf.title")}
      />
    </div>
  );
}

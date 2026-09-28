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
import { usePdfTool } from "../usePdfTool";

const VERTICAL = [
  { value: "bottom", key: "positionBottom" },
  { value: "top", key: "positionTop" },
] as const;

const HORIZONTAL = [
  { value: "left", key: "alignLeft" },
  { value: "center", key: "alignCenter" },
  { value: "right", key: "alignRight" },
] as const;

export function PageNumbersTool() {
  const { t } = useTranslation();
  const { input, output, job, selectInput, clearInput, chooseOutput } = usePdfTool();
  const [vertical, setVertical] = useState<string>("bottom");
  const [horizontal, setHorizontal] = useState<string>("center");
  const [start, setStart] = useState(1);
  const [fontSize, setFontSize] = useState(14);
  const [margin, setMargin] = useState(36);
  const [color, setColor] = useState(0);
  const [pages, setPages] = useState("");
  const [allPages, setAllPages] = useState(true);

  const canRun = input !== null && output !== null && (allPages || pages.trim().length > 0);

  const run = (): void => {
    if (!input || !output) return;
    void job.run(() =>
      invokeCommand<{ output: string; page_count: number }>("add_page_numbers", {
        request: {
          input,
          pages: allPages ? null : pages,
          vertical,
          horizontal,
          start,
          font_size: fontSize,
          margin,
          color,
          output,
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
            hint={t("tools.page-numbers.description")}
          />
        )}
      </ToolSection>

      <ToolSection title={t("toolUi.options")}>
        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
          <label className="text-sm text-slate-700">
            {t("toolUi.vertical")}
            <select
              value={vertical}
              onChange={(event) => setVertical(event.target.value)}
              className={`mt-1 ${fieldClass}`}
            >
              {VERTICAL.map((item) => (
                <option key={item.value} value={item.value}>
                  {t(`toolUi.${item.key}`)}
                </option>
              ))}
            </select>
          </label>

          <label className="text-sm text-slate-700">
            {t("toolUi.horizontal")}
            <select
              value={horizontal}
              onChange={(event) => setHorizontal(event.target.value)}
              className={`mt-1 ${fieldClass}`}
            >
              {HORIZONTAL.map((item) => (
                <option key={item.value} value={item.value}>
                  {t(`toolUi.${item.key}`)}
                </option>
              ))}
            </select>
          </label>

          <label className="text-sm text-slate-700">
            {t("toolUi.startNumber")}
            <input
              type="number"
              min={0}
              value={start}
              onChange={(event) => setStart(Number(event.target.value))}
              className={`mt-1 ${fieldClass}`}
            />
          </label>

          <label className="text-sm text-slate-700">
            {t("toolUi.fontSize")}
            <input
              type="number"
              min={6}
              max={72}
              value={fontSize}
              onChange={(event) => setFontSize(Number(event.target.value))}
              className={`mt-1 ${fieldClass}`}
            />
          </label>

          <label className="text-sm text-slate-700">
            {t("toolUi.marginLabel")}
            <input
              type="number"
              min={0}
              max={200}
              value={margin}
              onChange={(event) => setMargin(Number(event.target.value))}
              className={`mt-1 ${fieldClass}`}
            />
          </label>

          <label className="text-sm text-slate-700">
            {t("toolUi.colorLabel")}: {color.toFixed(1)}
            <input
              type="range"
              min={0}
              max={1}
              step={0.1}
              value={color}
              onChange={(event) => setColor(Number(event.target.value))}
              className="mt-2 w-full"
            />
          </label>
        </div>
      </ToolSection>

      <ToolSection title={t("toolUi.pages")}>
        <label className="flex items-center gap-2 text-sm text-slate-700">
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
        <OutputPicker path={output} onChoose={() => void chooseOutput("numbered")} />
      </ToolSection>

      {job.error && <ErrorBanner message={job.error} />}
      {job.result && <ResultPanel outputs={[job.result.output]} onReset={job.reset} />}

      <PrimaryButton
        onClick={run}
        disabled={!canRun || job.status === "success"}
        running={job.status === "running"}
        label={t("tools.page-numbers.title")}
      />
    </div>
  );
}

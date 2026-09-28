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
import { IMAGE_EXTENSIONS, PDF_EXTENSIONS, pickFiles } from "../../lib/dialogs";
import { baseName } from "../../lib/paths";
import { invokeCommand } from "../../lib/tauri";
import { usePdfTool } from "../usePdfTool";

type Kind = "text" | "image";

const POSITIONS = [
  { value: "center", key: "center" },
  { value: "top-left", key: "topLeft" },
  { value: "top-right", key: "topRight" },
  { value: "bottom-left", key: "bottomLeft" },
  { value: "bottom-right", key: "bottomRight" },
] as const;

export function WatermarkTool() {
  const { t } = useTranslation();
  const { input, output, job, selectInput, clearInput, chooseOutput } = usePdfTool();
  const [kind, setKind] = useState<Kind>("text");
  const [text, setText] = useState("مسودة");
  const [image, setImage] = useState<string | null>(null);
  const [fontSize, setFontSize] = useState(48);
  const [opacity, setOpacity] = useState(0.3);
  const [rotation, setRotation] = useState(45);
  const [color, setColor] = useState(0.5);
  const [scale, setScale] = useState(0.5);
  const [position, setPosition] = useState<string>("center");
  const [pages, setPages] = useState("");
  const [allPages, setAllPages] = useState(true);

  const chooseImage = async (): Promise<void> => {
    const paths = await pickFiles(IMAGE_EXTENSIONS, false);
    if (paths[0]) setImage(paths[0]);
  };

  const canRun =
    input !== null &&
    output !== null &&
    (allPages || pages.trim().length > 0) &&
    (kind === "text" ? text.trim().length > 0 : image !== null);

  const run = (): void => {
    if (!input || !output) return;
    void job.run(() =>
      invokeCommand<{ output: string; page_count: number }>("watermark_pdf", {
        request: {
          input,
          pages: allPages ? null : pages,
          kind,
          text: kind === "text" ? text : null,
          image: kind === "image" ? image : null,
          font_size: fontSize,
          opacity,
          rotation,
          color,
          scale,
          position,
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
            hint={t("tools.watermark.description")}
          />
        )}
      </ToolSection>

      <ToolSection title={t("toolUi.watermarkType")}>
        <div className="flex flex-wrap gap-2">
          {(["text", "image"] as const).map((value) => (
            <button
              key={value}
              type="button"
              onClick={() => setKind(value)}
              aria-pressed={kind === value}
              className={[
                "rounded-lg border px-4 py-2 text-sm font-medium transition-colors",
                kind === value
                  ? "border-indigo-300 bg-indigo-50 text-indigo-700"
                  : "border-slate-300 bg-white text-slate-700 hover:bg-slate-50",
              ].join(" ")}
            >
              {value === "text" ? t("toolUi.watermarkText") : t("toolUi.watermarkImage")}
            </button>
          ))}
        </div>

        {kind === "text" ? (
          <div className="mt-4 space-y-3">
            <input
              value={text}
              onChange={(event) => setText(event.target.value)}
              placeholder={t("toolUi.watermarkText")}
              className={fieldClass}
            />
            <label className="block text-sm text-slate-700">
              {t("toolUi.fontSize")}: {fontSize}
              <input
                type="range"
                min={8}
                max={200}
                value={fontSize}
                onChange={(event) => setFontSize(Number(event.target.value))}
                className="mt-2 w-full"
              />
            </label>
            <label className="block text-sm text-slate-700">
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
        ) : (
          <div className="mt-4 space-y-3">
            {image ? (
              <SelectedFileRow path={image} onClear={() => setImage(null)} />
            ) : (
              <button type="button" onClick={() => void chooseImage()} className={fieldClass}>
                {t("toolUi.chooseImage")}
              </button>
            )}
            <label className="block text-sm text-slate-700">
              {t("toolUi.scaleLabel")}: {scale.toFixed(2)}
              <input
                type="range"
                min={0.05}
                max={1}
                step={0.05}
                value={scale}
                onChange={(event) => setScale(Number(event.target.value))}
                className="mt-2 w-full"
              />
            </label>
            {image && <p className="truncate text-xs text-slate-500">{baseName(image)}</p>}
          </div>
        )}

        <label className="mt-4 block text-sm text-slate-700">
          {t("toolUi.opacity")}: {Math.round(opacity * 100)}%
          <input
            type="range"
            min={0}
            max={1}
            step={0.05}
            value={opacity}
            onChange={(event) => setOpacity(Number(event.target.value))}
            className="mt-2 w-full"
          />
        </label>

        <label className="mt-4 block text-sm text-slate-700">
          {t("toolUi.rotation")}: {rotation}°
          <input
            type="range"
            min={-90}
            max={90}
            value={rotation}
            onChange={(event) => setRotation(Number(event.target.value))}
            className="mt-2 w-full"
          />
        </label>

        <label className="mt-4 block text-sm text-slate-700">
          {t("toolUi.positionLabel")}
          <select
            value={position}
            onChange={(event) => setPosition(event.target.value)}
            className={`mt-1 ${fieldClass}`}
          >
            {POSITIONS.map((item) => (
              <option key={item.value} value={item.value}>
                {t(`toolUi.positionOption.${item.key}`)}
              </option>
            ))}
          </select>
        </label>
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
        <OutputPicker path={output} onChoose={() => void chooseOutput("watermarked")} />
      </ToolSection>

      {job.error && <ErrorBanner message={job.error} />}
      {job.result && <ResultPanel outputs={[job.result.output]} onReset={job.reset} />}

      <PrimaryButton
        onClick={run}
        disabled={!canRun || job.status === "success"}
        running={job.status === "running"}
        label={t("tools.watermark.title")}
      />
    </div>
  );
}

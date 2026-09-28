import { useState } from "react";
import { ArrowDown, ArrowUp, GripVertical, X } from "lucide-react";
import { useTranslation } from "react-i18next";

import { FileDropZone } from "../../components/file-drop/FileDropZone";
import { ResultPanel } from "../../components/common/ResultPanel";
import {
  ErrorBanner,
  OutputPicker,
  PrimaryButton,
  ToolSection,
} from "../../components/common/ToolControls";
import { PDF_EXTENSIONS, pickFiles, pickOutputFile } from "../../lib/dialogs";
import { baseName, dirName, joinPath } from "../../lib/paths";
import { invokeCommand } from "../../lib/tauri";
import { useToolJob } from "../../hooks/useToolJob";

interface MergeResult {
  output: string;
  page_count: number;
}

export function MergeTool() {
  const { t } = useTranslation();
  const [inputs, setInputs] = useState<string[]>([]);
  const [output, setOutput] = useState<string | null>(null);
  const job = useToolJob<MergeResult>();

  const addFiles = (paths: string[]): void => {
    setInputs((current) => {
      const merged = [...current];
      for (const path of paths) {
        if (!merged.includes(path)) merged.push(path);
      }
      return merged;
    });
    job.reset();
  };

  const removeAt = (index: number): void => {
    setInputs((current) => current.filter((_, i) => i !== index));
    job.reset();
  };

  const move = (index: number, offset: number): void => {
    setInputs((current) => {
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
    const first = inputs[0];
    const suggested = first ? joinPath(dirName(first), "merged.pdf") : "merged.pdf";
    const chosen = await pickOutputFile(suggested);
    if (chosen) setOutput(chosen);
  };

  const canRun = inputs.length >= 2 && output !== null;

  const run = (): void => {
    if (!output) return;
    void job.run(() =>
      invokeCommand<MergeResult>("merge_pdfs", {
        request: { inputs, output },
      }),
    );
  };

  return (
    <div className="mx-auto max-w-3xl space-y-6">
      <ToolSection title={t("toolUi.input")}>
        {inputs.length === 0 ? (
          <FileDropZone
            onFiles={addFiles}
            accept={PDF_EXTENSIONS}
            multiple
            hint={t("tools.merge.description")}
          />
        ) : (
          <div className="space-y-2">
            <ul className="space-y-1">
              {inputs.map((path, index) => (
                <li
                  key={path}
                  className="flex items-center gap-2 rounded-lg border border-slate-200 bg-white px-3 py-2"
                >
                  <GripVertical className="h-4 w-4 shrink-0 text-slate-300" aria-hidden="true" />
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
                    disabled={index === inputs.length - 1}
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
                  const paths = await pickFiles(PDF_EXTENSIONS, true);
                  if (paths.length > 0) addFiles(paths);
                })();
              }}
              className="text-sm font-medium text-indigo-600 hover:text-indigo-800"
            >
              {t("toolUi.addMore")}
            </button>
          </div>
        )}
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
        label={t("tools.merge.title")}
      />
    </div>
  );
}

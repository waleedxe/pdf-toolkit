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

export function ExtractTool() {
  const { t } = useTranslation();
  const { input, output, job, selectInput, clearInput, chooseOutput } = usePdfTool();
  const [pages, setPages] = useState("1");

  const canRun = input !== null && output !== null && pages.trim().length > 0;

  const run = (): void => {
    if (!input || !output) return;
    void job.run(() =>
      invokeCommand<PdfOperationResult>("extract_pages", {
        request: { input, pages, output },
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
            hint={t("tools.extract.description")}
          />
        )}
      </ToolSection>

      <ToolSection title={t("toolUi.pages")}>
        <input
          value={pages}
          onChange={(event) => setPages(event.target.value)}
          placeholder={t("toolUi.pagesPlaceholder")}
          className="w-full rounded-lg border border-slate-300 px-3 py-2 text-sm text-slate-800 focus:border-indigo-400 focus:outline-none"
        />
        <p className="mt-2 text-sm text-slate-500">{t("toolUi.pagesHelp")}</p>
      </ToolSection>

      <ToolSection title={t("toolUi.output")}>
        <OutputPicker path={output} onChoose={() => void chooseOutput("extracted")} />
      </ToolSection>

      {job.error && <ErrorBanner message={job.error} />}
      {job.result && <ResultPanel outputs={[job.result.output]} onReset={job.reset} />}

      <PrimaryButton
        onClick={run}
        disabled={!canRun || job.status === "success"}
        running={job.status === "running"}
        label={t("tools.extract.title")}
      />
    </div>
  );
}

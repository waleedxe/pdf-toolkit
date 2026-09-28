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

export function UnlockTool() {
  const { t } = useTranslation();
  const { input, output, job, selectInput, clearInput, chooseOutput } = usePdfTool();
  const [password, setPassword] = useState("");

  const canRun = input !== null && output !== null && password.length > 0;

  const run = (): void => {
    if (!input || !output) return;
    void job.run(() =>
      invokeCommand<{ output: string; page_count: number }>("unlock_pdf", {
        request: { input, password, output },
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
            hint={t("tools.unlock.description")}
          />
        )}
      </ToolSection>

      <ToolSection title={t("toolUi.password")}>
        <input
          type="password"
          value={password}
          onChange={(event) => setPassword(event.target.value)}
          placeholder={t("toolUi.unlockPassword")}
          className={fieldClass}
        />
        <p className="mt-2 text-sm text-slate-500">{t("toolUi.unlockWarning")}</p>
      </ToolSection>

      <ToolSection title={t("toolUi.output")}>
        <OutputPicker path={output} onChoose={() => void chooseOutput("unlocked")} />
      </ToolSection>

      {job.error && <ErrorBanner message={job.error} />}
      {job.result && <ResultPanel outputs={[job.result.output]} onReset={job.reset} />}

      <PrimaryButton
        onClick={run}
        disabled={!canRun || job.status === "success"}
        running={job.status === "running"}
        label={t("tools.unlock.title")}
      />
    </div>
  );
}

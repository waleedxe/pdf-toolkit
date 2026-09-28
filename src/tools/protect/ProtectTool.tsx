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

export function ProtectTool() {
  const { t } = useTranslation();
  const { input, output, job, selectInput, clearInput, chooseOutput } = usePdfTool();
  const [password, setPassword] = useState("");
  const [confirm, setConfirm] = useState("");
  const [allowPrinting, setAllowPrinting] = useState(true);
  const [allowCopying, setAllowCopying] = useState(true);
  const [allowModifying, setAllowModifying] = useState(false);

  const mismatch = confirm.length > 0 && password !== confirm;
  const canRun = input !== null && output !== null && password.length > 0 && password === confirm;

  const run = (): void => {
    if (!input || !output) return;
    void job.run(() =>
      invokeCommand<{ output: string; page_count: number }>("protect_pdf", {
        request: {
          input,
          password,
          allow_printing: allowPrinting,
          allow_copying: allowCopying,
          allow_modifying: allowModifying,
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
            hint={t("tools.protect.description")}
          />
        )}
      </ToolSection>

      <ToolSection title={t("toolUi.password")}>
        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
          <label className="text-sm text-slate-700">
            {t("toolUi.password")}
            <input
              type="password"
              value={password}
              onChange={(event) => setPassword(event.target.value)}
              className={`mt-1 ${fieldClass}`}
            />
          </label>
          <label className="text-sm text-slate-700">
            {t("toolUi.confirmPassword")}
            <input
              type="password"
              value={confirm}
              onChange={(event) => setConfirm(event.target.value)}
              className={`mt-1 ${fieldClass}`}
            />
          </label>
        </div>
        {mismatch && <p className="mt-2 text-sm text-rose-600">{t("toolUi.passwordMismatch")}</p>}
      </ToolSection>

      <ToolSection title={t("toolUi.permissions")}>
        <div className="space-y-2">
          <label className="flex items-center gap-2 text-sm text-slate-700">
            <input
              type="checkbox"
              checked={allowPrinting}
              onChange={(event) => setAllowPrinting(event.target.checked)}
              className="h-4 w-4 rounded border-slate-300"
            />
            {t("toolUi.allowPrinting")}
          </label>
          <label className="flex items-center gap-2 text-sm text-slate-700">
            <input
              type="checkbox"
              checked={allowCopying}
              onChange={(event) => setAllowCopying(event.target.checked)}
              className="h-4 w-4 rounded border-slate-300"
            />
            {t("toolUi.allowCopying")}
          </label>
          <label className="flex items-center gap-2 text-sm text-slate-700">
            <input
              type="checkbox"
              checked={allowModifying}
              onChange={(event) => setAllowModifying(event.target.checked)}
              className="h-4 w-4 rounded border-slate-300"
            />
            {t("toolUi.allowModifying")}
          </label>
        </div>
      </ToolSection>

      <ToolSection title={t("toolUi.output")}>
        <OutputPicker path={output} onChoose={() => void chooseOutput("protected")} />
      </ToolSection>

      {job.error && <ErrorBanner message={job.error} />}
      {job.result && <ResultPanel outputs={[job.result.output]} onReset={job.reset} />}

      <PrimaryButton
        onClick={run}
        disabled={!canRun || job.status === "success"}
        running={job.status === "running"}
        label={t("tools.protect.title")}
      />
    </div>
  );
}

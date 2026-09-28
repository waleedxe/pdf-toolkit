import { AlertTriangle, Loader2 } from "lucide-react";
import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";

/** Bordered options/input section used by every tool. */
export function ToolSection({ title, children }: { title: string; children: ReactNode }) {
  return (
    <section className="rounded-xl border border-slate-200 bg-white p-5">
      <h2 className="mb-3 text-sm font-semibold uppercase tracking-wide text-slate-400">{title}</h2>
      {children}
    </section>
  );
}

/** Actionable error state. */
export function ErrorBanner({ message }: { message: string }) {
  const { t } = useTranslation();
  return (
    <div
      role="alert"
      className="flex items-start gap-3 rounded-xl border border-rose-200 bg-rose-50 p-4"
    >
      <AlertTriangle className="mt-0.5 h-5 w-5 shrink-0 text-rose-500" aria-hidden="true" />
      <div>
        <p className="font-medium text-rose-800">{t("toolUi.errorTitle")}</p>
        <p className="text-sm text-rose-700">{message}</p>
      </div>
    </div>
  );
}

interface PrimaryButtonProps {
  onClick: () => void;
  disabled: boolean;
  running: boolean;
  label: string;
}

/** The tool's main call-to-action, with an inline spinner while running. */
export function PrimaryButton({ onClick, disabled, running, label }: PrimaryButtonProps) {
  const { t } = useTranslation();
  return (
    <button
      type="button"
      onClick={onClick}
      disabled={disabled || running}
      className="inline-flex items-center gap-2 rounded-lg bg-indigo-600 px-4 py-2.5 text-sm font-semibold text-white transition-colors hover:bg-indigo-700 disabled:cursor-not-allowed disabled:opacity-50"
    >
      {running && <Loader2 className="h-4 w-4 animate-spin" aria-hidden="true" />}
      {running ? t("toolUi.processing") : label}
    </button>
  );
}

/** A single chosen file, with a remove control. */
export function SelectedFileRow({ path, onClear }: { path: string; onClear: () => void }) {
  const { t } = useTranslation();
  return (
    <div className="flex items-center justify-between gap-3 rounded-lg border border-slate-200 bg-white px-3 py-2">
      <span className="truncate text-sm text-slate-700" title={path}>
        {path}
      </span>
      <button
        type="button"
        onClick={onClear}
        className="shrink-0 text-xs font-medium text-slate-500 hover:text-rose-600"
      >
        {t("toolUi.remove")}
      </button>
    </div>
  );
}

/** Lets the user choose where a produced PDF should be written. */
export function OutputPicker({ path, onChoose }: { path: string | null; onChoose: () => void }) {
  const { t } = useTranslation();
  return (
    <div className="space-y-2">
      <button
        type="button"
        onClick={onChoose}
        className="inline-flex items-center gap-2 rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm font-medium text-slate-700 hover:bg-slate-50"
      >
        {t("toolUi.chooseOutput")}
      </button>
      {path ? (
        <p className="truncate text-sm text-slate-600" title={path}>
          {path}
        </p>
      ) : (
        <p className="text-sm text-slate-400">{t("toolUi.noOutput")}</p>
      )}
    </div>
  );
}

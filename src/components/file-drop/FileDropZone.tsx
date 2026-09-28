import { FileUp } from "lucide-react";
import { useTranslation } from "react-i18next";

import { useFileDrop } from "../../hooks/useFileDrop";
import { pickFiles } from "../../lib/dialogs";

interface FileDropZoneProps {
  /** Called with accepted native paths, in drop order. */
  onFiles: (paths: string[]) => void;
  /** Accepted extensions; omit to accept any file. */
  accept?: readonly string[];
  /** Whether more than one file may be selected/dropped. Defaults to `true`. */
  multiple?: boolean;
  /** Accordion-style description shown under the main label. */
  hint?: string;
}

/**
 * Shared drop target plus "browse" button.
 *
 * It only reports paths to the parent; it never reads or uploads file content.
 */
export function FileDropZone({ onFiles, accept, multiple = true, hint }: FileDropZoneProps) {
  const { t } = useTranslation();
  const { isDragging } = useFileDrop({
    onDrop: onFiles,
    ...(accept ? { accept } : {}),
    multiple,
  });

  const handleBrowse = async (): Promise<void> => {
    try {
      const paths = await pickFiles(accept ?? [], multiple);
      if (paths.length > 0) onFiles(paths);
    } catch {
      // The user closed the dialog or no native backend is available.
    }
  };

  return (
    <button
      type="button"
      onClick={() => void handleBrowse()}
      className={[
        "flex w-full flex-col items-center justify-center gap-3 rounded-xl border-2 border-dashed px-6 py-10 text-center transition-colors",
        isDragging
          ? "border-indigo-400 bg-indigo-50"
          : "border-slate-300 bg-white hover:border-indigo-300 hover:bg-slate-50",
      ].join(" ")}
    >
      <span className="flex h-12 w-12 items-center justify-center rounded-full bg-indigo-50 text-indigo-600">
        <FileUp className="h-6 w-6" aria-hidden="true" />
      </span>
      <span className="font-medium text-slate-700">{t("toolUi.dropHere")}</span>
      <span className="text-sm text-slate-500">{hint ?? t("toolUi.browse")}</span>
    </button>
  );
}

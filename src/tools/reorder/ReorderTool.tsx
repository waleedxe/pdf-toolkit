import { useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { ArrowLeft, ArrowRight, GripVertical, Loader2, RotateCcw } from "lucide-react";

import { FileDropZone } from "../../components/file-drop/FileDropZone";
import { PageThumbnail } from "../../components/page-thumbnail/PageThumbnail";
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

interface PdfInfo {
  page_count: number;
}

const THUMBNAIL_WIDTH = 220;

export function ReorderTool() {
  const { t } = useTranslation();
  const { input, output, job, selectInput, clearInput, chooseOutput } = usePdfTool();
  const [order, setOrder] = useState<number[]>([]);
  const [loading, setLoading] = useState(false);
  const [loadError, setLoadError] = useState<string | null>(null);
  const dragIndex = useRef<number | null>(null);

  const openInput = (paths: string[]): void => {
    const first = paths[0];
    if (!first) return;
    selectInput([first]);
    setOrder([]);
    setLoadError(null);
    setLoading(true);

    void (async () => {
      try {
        const info = await invokeCommand<PdfInfo>("pdf_info", {
          request: { input: first },
        });
        setOrder(Array.from({ length: info.page_count }, (_, index) => index + 1));
      } catch (cause) {
        setLoadError(cause instanceof Error ? cause.message : String(cause));
      } finally {
        setLoading(false);
      }
    })();
  };

  const clear = (): void => {
    clearInput();
    setOrder([]);
    setLoadError(null);
  };

  const move = (index: number, offset: number): void => {
    setOrder((current) => {
      const target = index + offset;
      if (target < 0 || target >= current.length) return current;
      const next = [...current];
      const [item] = next.splice(index, 1);
      if (item === undefined) return current;
      next.splice(target, 0, item);
      return next;
    });
  };

  const handleDrop = (index: number): void => {
    const from = dragIndex.current;
    dragIndex.current = null;
    if (from === null || from === index) return;
    setOrder((current) => {
      const next = [...current];
      const [item] = next.splice(from, 1);
      if (item === undefined) return current;
      next.splice(index, 0, item);
      return next;
    });
  };

  const resetOrder = (): void => {
    setOrder((current) => [...current].sort((a, b) => a - b));
  };

  const canRun = input !== null && output !== null && order.length > 0;

  const run = (): void => {
    if (!input || !output) return;
    void job.run(() =>
      invokeCommand<{ output: string; page_count: number }>("reorder_pages", {
        request: { input, order, output },
      }),
    );
  };

  return (
    <div className="mx-auto max-w-4xl space-y-6">
      <ToolSection title={t("toolUi.input")}>
        {input ? (
          <SelectedFileRow path={input} onClear={clear} />
        ) : (
          <FileDropZone
            onFiles={openInput}
            accept={PDF_EXTENSIONS}
            multiple={false}
            hint={t("tools.reorder.description")}
          />
        )}
      </ToolSection>

      {loadError && <ErrorBanner message={loadError} />}

      {loading && (
        <div className="flex items-center gap-2 text-sm text-slate-500">
          <Loader2 className="h-4 w-4 animate-spin" aria-hidden="true" />
          {t("toolUi.loadingPages")}
        </div>
      )}

      {input && order.length > 0 && (
        <ToolSection title={t("toolUi.pages")}>
          <p className="mb-3 text-sm text-slate-500">{t("toolUi.reorderHint")}</p>
          <div className="grid grid-cols-2 gap-3 sm:grid-cols-3 md:grid-cols-4">
            {order.map((page, index) => (
              <div
                key={`${page}-${index}`}
                draggable
                onDragStart={() => {
                  dragIndex.current = index;
                }}
                onDragOver={(event) => event.preventDefault()}
                onDrop={() => handleDrop(index)}
                className="group relative cursor-grab rounded-lg border border-slate-200 bg-white p-2 active:cursor-grabbing"
              >
                <PageThumbnail path={input} page={page} maxWidth={THUMBNAIL_WIDTH} />
                <div className="mt-1 flex items-center justify-between">
                  <span className="inline-flex items-center gap-1 text-xs text-slate-500">
                    <GripVertical className="h-3 w-3" aria-hidden="true" />
                    {t("toolUi.pageLabel", { page })}
                  </span>
                  <span className="flex gap-1">
                    <button
                      type="button"
                      onClick={() => move(index, -1)}
                      disabled={index === 0}
                      aria-label={t("toolUi.moveUp")}
                      className="text-slate-400 hover:text-slate-700 disabled:opacity-30"
                    >
                      <ArrowLeft className="h-3.5 w-3.5 rtl:hidden" aria-hidden="true" />
                      <ArrowRight className="hidden h-3.5 w-3.5 rtl:block" aria-hidden="true" />
                    </button>
                    <button
                      type="button"
                      onClick={() => move(index, 1)}
                      disabled={index === order.length - 1}
                      aria-label={t("toolUi.moveDown")}
                      className="text-slate-400 hover:text-slate-700 disabled:opacity-30"
                    >
                      <ArrowRight className="h-3.5 w-3.5 rtl:hidden" aria-hidden="true" />
                      <ArrowLeft className="hidden h-3.5 w-3.5 rtl:block" aria-hidden="true" />
                    </button>
                  </span>
                </div>
              </div>
            ))}
          </div>
          <button
            type="button"
            onClick={resetOrder}
            className="mt-3 inline-flex items-center gap-2 text-sm font-medium text-indigo-600 hover:text-indigo-800"
          >
            <RotateCcw className="h-4 w-4" aria-hidden="true" />
            {t("toolUi.resetOrder")}
          </button>
        </ToolSection>
      )}

      <ToolSection title={t("toolUi.output")}>
        <OutputPicker path={output} onChoose={() => void chooseOutput("reordered")} />
      </ToolSection>

      {job.error && <ErrorBanner message={job.error} />}
      {job.result && <ResultPanel outputs={[job.result.output]} onReset={job.reset} />}

      <PrimaryButton
        onClick={run}
        disabled={!canRun || job.status === "success"}
        running={job.status === "running"}
        label={t("tools.reorder.title")}
      />
    </div>
  );
}

import { useState } from "react";

import { useToolJob, type ToolJob } from "../hooks/useToolJob";
import { pickOutputFile } from "../lib/dialogs";
import { dirName, fileNameWithoutExtension, joinPath } from "../lib/paths";

/** Shape returned by every single-output PDF command. */
export interface PdfOperationResult {
  output: string;
  page_count: number;
}

export interface PdfToolController {
  input: string | null;
  output: string | null;
  job: ToolJob<PdfOperationResult>;
  selectInput: (paths: string[]) => void;
  clearInput: () => void;
  chooseOutput: (suffix: string) => Promise<void>;
}

/**
 * Shared state for tools that take one PDF and write one PDF
 * (Extract, Remove, Rotate, Merge uses its own).
 */
export function usePdfTool(): PdfToolController {
  const [input, setInput] = useState<string | null>(null);
  const [output, setOutput] = useState<string | null>(null);
  const job = useToolJob<PdfOperationResult>();

  const selectInput = (paths: string[]): void => {
    const first = paths[0];
    if (!first) return;
    setInput(first);
    setOutput(null);
    job.reset();
  };

  const clearInput = (): void => {
    setInput(null);
    setOutput(null);
    job.reset();
  };

  const chooseOutput = async (suffix: string): Promise<void> => {
    if (!input) return;
    const suggested = joinPath(dirName(input), `${fileNameWithoutExtension(input)}_${suffix}.pdf`);
    const chosen = await pickOutputFile(suggested);
    if (chosen) setOutput(chosen);
  };

  return { input, output, job, selectInput, clearInput, chooseOutput };
}

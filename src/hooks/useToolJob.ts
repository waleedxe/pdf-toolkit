import { useCallback, useState } from "react";

/** Lifecycle of a single tool run, mirroring the backend job states. */
export type JobStatus = "idle" | "running" | "success" | "error";

export interface ToolJob<TResult> {
  status: JobStatus;
  result: TResult | null;
  error: string | null;
  run: (task: () => Promise<TResult>) => Promise<void>;
  reset: () => void;
}

/**
 * Small state machine shared by every tool.
 *
 * Heavy work happens in the Rust backend; this only tracks the request/response
 * lifecycle so the UI can show running, success, and error states consistently.
 */
export function useToolJob<TResult>(): ToolJob<TResult> {
  const [status, setStatus] = useState<JobStatus>("idle");
  const [result, setResult] = useState<TResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  const run = useCallback(async (task: () => Promise<TResult>): Promise<void> => {
    setStatus("running");
    setError(null);
    setResult(null);
    try {
      const value = await task();
      setResult(value);
      setStatus("success");
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
      setStatus("error");
    }
  }, []);

  const reset = useCallback((): void => {
    setStatus("idle");
    setResult(null);
    setError(null);
  }, []);

  return { status, result, error, run, reset };
}

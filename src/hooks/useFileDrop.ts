import { useEffect, useRef, useState } from "react";
import { getCurrentWebview } from "@tauri-apps/api/webview";

import { extension } from "../lib/paths";
import { isTauri } from "../lib/tauri";

/**
 * Drag-and-drop file handling.
 *
 * Inside Tauri we listen to the native webview drag-drop events, which deliver
 * real filesystem paths (never document bytes). In a plain browser (dev and
 * tests) we fall back to DOM drag-and-drop.
 *
 * The hook treats the whole window as the drop target; tools that are visible
 * only one at a time therefore do not need their own hit-testing.
 */

export interface FileDropOptions {
  /** Called with the accepted native paths, in drop order. */
  onDrop: (paths: string[]) => void;
  /** Accepted file extensions, e.g. `["pdf"]`. Omit to accept everything. */
  accept?: readonly string[];
  /** Whether more than one file may be dropped at once. Defaults to `true`. */
  multiple?: boolean;
}

export interface FileDropState {
  /** True while files are hovering over the window. */
  isDragging: boolean;
}

function normalizeExtensions(accept: readonly string[] | undefined): Set<string> | null {
  if (!accept || accept.length === 0) return null;
  return new Set(
    accept.map((value) => (value.startsWith(".") ? value : `.${value}`).toLowerCase()),
  );
}

function selectPaths(
  paths: string[],
  accept: readonly string[] | undefined,
  multiple: boolean,
): string[] {
  const allowed = normalizeExtensions(accept);
  const accepted = allowed ? paths.filter((path) => allowed.has(extension(path))) : paths;
  return multiple ? accepted : accepted.slice(0, 1);
}

export function useFileDrop(options: FileDropOptions): FileDropState {
  const { onDrop } = options;
  const [isDragging, setIsDragging] = useState(false);

  // Keep the latest callback/options without re-subscribing on every render.
  const onDropRef = useRef(onDrop);
  const acceptRef = useRef(options.accept);
  const multipleRef = useRef(options.multiple ?? true);

  useEffect(() => {
    onDropRef.current = onDrop;
    acceptRef.current = options.accept;
    multipleRef.current = options.multiple ?? true;
  });

  useEffect(() => {
    const emit = (paths: string[]): void => {
      const accepted = selectPaths(paths, acceptRef.current, multipleRef.current);
      if (accepted.length > 0) onDropRef.current(accepted);
    };

    if (isTauri()) {
      let unlisten: (() => void) | undefined;
      let cancelled = false;

      void getCurrentWebview()
        .onDragDropEvent((event) => {
          const payload = event.payload;
          if (payload.type === "enter" || payload.type === "over") {
            setIsDragging(true);
          } else if (payload.type === "leave") {
            setIsDragging(false);
          } else if (payload.type === "drop") {
            setIsDragging(false);
            emit(payload.paths);
          }
        })
        .then((stop) => {
          if (cancelled) stop();
          else unlisten = stop;
        })
        .catch(() => {
          // Native drag-drop is unavailable; the DOM fallback below still runs.
        });

      return () => {
        cancelled = true;
        unlisten?.();
      };
    }

    // Browser fallback (development and tests).
    const handleDragOver = (event: DragEvent): void => {
      event.preventDefault();
      setIsDragging(true);
    };
    const handleDragLeave = (event: DragEvent): void => {
      if (event.relatedTarget === null) setIsDragging(false);
    };
    const handleDrop = (event: DragEvent): void => {
      event.preventDefault();
      setIsDragging(false);
      const files = Array.from(event.dataTransfer?.files ?? []);
      emit(
        files.map((file) => {
          const candidate = file as File & { path?: string };
          return candidate.path ?? file.name;
        }),
      );
    };

    window.addEventListener("dragover", handleDragOver);
    window.addEventListener("dragleave", handleDragLeave);
    window.addEventListener("drop", handleDrop);
    return () => {
      window.removeEventListener("dragover", handleDragOver);
      window.removeEventListener("dragleave", handleDragLeave);
      window.removeEventListener("drop", handleDrop);
    };
  }, []);

  return { isDragging };
}

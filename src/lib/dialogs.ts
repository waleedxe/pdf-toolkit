import { open, save } from "@tauri-apps/plugin-dialog";
import { openPath, revealItemInDir } from "@tauri-apps/plugin-opener";

/**
 * Native dialog and opener helpers.
 *
 * Wrapping the Tauri plugins here keeps the tool components free of plugin
 * imports and makes the calls easy to mock in tests.
 */

/** Extensions used by the file filters. */
export const PDF_EXTENSIONS = ["pdf"] as const;
export const IMAGE_EXTENSIONS = ["jpg", "jpeg", "png", "webp"] as const;

function filterFor(
  extensions: readonly string[],
): { name: string; extensions: string[] } | undefined {
  if (extensions.length === 0) return undefined;
  return { name: "Supported files", extensions: [...extensions] };
}

function asArray(value: string | string[] | null): string[] {
  if (value === null) return [];
  return Array.isArray(value) ? value : [value];
}

/** Open a native picker for files, optionally filtered by extension. */
export async function pickFiles(extensions: readonly string[], multiple = true): Promise<string[]> {
  const filter = filterFor(extensions);
  const result = await open({
    multiple,
    directory: false,
    ...(filter ? { filters: [filter] } : {}),
  });
  return asArray(result as string | string[] | null);
}

/** Open a native picker for a single PDF. */
export async function pickPdfFile(): Promise<string | null> {
  const files = await pickFiles(PDF_EXTENSIONS, false);
  return files[0] ?? null;
}

/** Open a native save dialog for a PDF output. */
export async function pickOutputFile(defaultName: string): Promise<string | null> {
  const result = await save({
    defaultPath: defaultName,
    filters: [{ name: "PDF documents", extensions: [...PDF_EXTENSIONS] }],
  });
  return result ?? null;
}

/** Open a native folder picker. */
export async function pickDirectory(): Promise<string | null> {
  const result = await open({ directory: true, multiple: false });
  return typeof result === "string" ? result : null;
}

/** Open a produced file with the OS default application. */
export async function openFile(path: string): Promise<void> {
  await openPath(path);
}

/** Reveal a produced file in the OS file manager. */
export async function revealPath(path: string): Promise<void> {
  await revealItemInDir(path);
}

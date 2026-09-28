import { invoke } from "@tauri-apps/api/core";

/**
 * Thin, typed wrapper around the Tauri command bridge.
 *
 * Everything that talks to the Rust backend goes through here so the rest of
 * the frontend never imports `@tauri-apps/api` directly. That keeps the bridge
 * easy to mock in tests and easy to swap later.
 */

/** Basic application information returned by the `app_info` command. */
export interface AppInfo {
  name: string;
  version: string;
  /** Version of the engine-agnostic `pdf-core` crate. */
  pdf_core_version: string;
}

/**
 * Whether the app is running inside the Tauri webview (as opposed to a plain
 * browser during `npm run dev` or in tests).
 */
export function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

/**
 * Invoke a Tauri command, failing fast with a clear message when there is no
 * native backend available (e.g. running the UI in a browser tab).
 */
export async function invokeCommand<TResult>(
  command: string,
  args?: Record<string, unknown>,
): Promise<TResult> {
  if (!isTauri()) {
    throw new Error(
      `The native backend is not available, so "${command}" cannot run. Launch the desktop app with \`npm run tauri dev\`.`,
    );
  }
  return invoke<TResult>(command, args);
}

/** Read application metadata from the backend. */
export function getAppInfo(): Promise<AppInfo> {
  return invokeCommand<AppInfo>("app_info");
}

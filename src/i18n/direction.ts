import type { SupportedLanguage } from "./index";

/**
 * Apply the language to the document root so that:
 * - screen readers and the browser know the content language;
 * - the layout direction flips between LTR and RTL.
 */
export function applyDocumentDirection(language: SupportedLanguage): void {
  const dir = language === "ar" ? "rtl" : "ltr";
  document.documentElement.lang = language;
  document.documentElement.dir = dir;
}

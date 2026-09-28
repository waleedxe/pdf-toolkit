import i18n from "i18next";

import type { SupportedLanguage } from "./index";

/**
 * i18next does not know about React state, so language changes are pushed here
 * as a side effect of the settings store.
 */
export async function changeLanguage(language: SupportedLanguage): Promise<void> {
  if (i18n.language !== language) {
    await i18n.changeLanguage(language);
  }
}

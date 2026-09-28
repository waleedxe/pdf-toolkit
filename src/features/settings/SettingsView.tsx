import { useTranslation } from "react-i18next";

import { LanguageSwitcher } from "../../components/common/LanguageSwitcher";

export function SettingsView() {
  const { t } = useTranslation();

  return (
    <div className="mx-auto max-w-2xl space-y-6">
      <section className="rounded-xl border border-slate-200 bg-white p-6">
        <h2 className="font-semibold text-slate-800">{t("settings.language")}</h2>
        <p className="mt-1 text-sm text-slate-500">{t("settings.languageHelp")}</p>
        <div className="mt-4">
          <LanguageSwitcher />
        </div>
      </section>

      <section className="rounded-xl border border-slate-200 bg-white p-6">
        <h2 className="font-semibold text-slate-800">{t("settings.privacyTitle")}</h2>
        <p className="mt-1 text-sm text-slate-500">{t("settings.privacyBody")}</p>
      </section>

      <section className="rounded-xl border border-slate-200 bg-white p-6">
        <h2 className="font-semibold text-slate-800">{t("settings.aboutTitle")}</h2>
        <p className="mt-1 text-sm text-slate-500">{t("settings.aboutBody")}</p>
      </section>
    </div>
  );
}

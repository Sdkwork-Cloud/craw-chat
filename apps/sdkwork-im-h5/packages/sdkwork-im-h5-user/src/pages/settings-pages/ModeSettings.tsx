import { useTranslation } from 'react-i18next';
import React, { useState, useEffect } from "react";
import { SettingsService } from "../../services/SettingsService";
import { PageLayout } from "../../components/SettingsCommons";

export const TeenMode = () => {
  const { t } = useTranslation();
const [enabled, setEnabled] = useState(false);

  useEffect(() => {
    SettingsService.getSettings().then((s) => setEnabled(s.teenMode));
  }, []);

  const handleToggle = async () => {
    const newVal = !enabled;
    setEnabled(newVal);
    await SettingsService.updateSettings({ teenMode: newVal });
  };

  return (
    <PageLayout title={t("user:mode.teen_title", "Teen mode")}>
      <div className="flex flex-col items-center py-10">
        <div className="w-16 h-16 bg-primary-blue/10 rounded-full flex items-center justify-center mb-4">
          <span className="text-primary-blue text-2xl">👦</span>
        </div>
        <h3 className="text-[18px] font-medium text-text-main mb-2">{t("user:mode.teen_title", "Teen mode")}</h3>
        <p className="text-[14px] text-text-sub text-center px-8 mb-8">{t("user:mode.teen_desc", "When enabled, some features and usage time are limited. Turning it on or off requires a separate password.")}</p>
        <button
          onClick={handleToggle}
          className="w-[200px] h-12 bg-accent-green text-white rounded-lg font-medium active:opacity-80 transition-opacity"
        >{enabled ? t("user:mode.teen_off", "Turn off teen mode") : t("user:mode.teen_on", "Turn on teen mode")}</button>
      </div>
    </PageLayout>
  );
};

export const ElderlyMode = () => {
  const { t } = useTranslation();
  
const [enabled, setEnabled] = useState(false);

  useEffect(() => {
    SettingsService.getSettings().then((s) => setEnabled(s.elderlyMode));
  }, []);

  const handleToggle = async () => {
    const newVal = !enabled;
    setEnabled(newVal);
    await SettingsService.updateSettings({ elderlyMode: newVal });
  };

  return (
    <PageLayout title={t("user:mode.care_title", "Care mode")}>
      <div className="flex flex-col items-center py-10">
        <div className="w-16 h-16 bg-orange-500/10 rounded-full flex items-center justify-center mb-4">
          <span className="text-orange-500 text-2xl">❤️</span>
        </div>
        <h3 className="text-[18px] font-medium text-text-main mb-2">{t("user:mode.care_title", "Care mode")}</h3>
        <p className="text-[14px] text-text-sub text-center px-8 mb-8">{t("user:mode.care_desc", "When enabled, text and buttons become larger with stronger colors.")}</p>
        <button
          onClick={handleToggle}
          className="w-[200px] h-12 bg-accent-green text-white rounded-lg font-medium active:opacity-80 transition-opacity"
        >{enabled ? t("user:mode.care_off", "Turn off care mode") : t("user:mode.care_on", "Turn on care mode")}</button>
      </div>
    </PageLayout>
  );
};

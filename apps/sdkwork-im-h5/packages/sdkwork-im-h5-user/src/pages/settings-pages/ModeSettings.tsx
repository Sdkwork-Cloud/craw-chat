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
    <PageLayout title={t("user:mode.teen_title", "青少年模式")}>
      <div className="flex flex-col items-center py-10">
        <div className="w-16 h-16 bg-primary-blue/10 rounded-full flex items-center justify-center mb-4">
          <span className="text-primary-blue text-2xl">👦</span>
        </div>
        <h3 className="text-[18px] font-medium text-text-main mb-2">{t("user:mode.teen_title", "青少年模式")}</h3>
        <p className="text-[14px] text-text-sub text-center px-8 mb-8">{t("user:mode.teen_desc", "开启后，将限制部分功能的使用，并限制使用时间。开启或关闭都需要输入独立密码。")}</p>
        <button
          onClick={handleToggle}
          className="w-[200px] h-12 bg-[#00B42A] text-white rounded-lg font-medium active:opacity-80 transition-opacity"
        >{enabled ? t("user:mode.teen_off", "关闭青少年模式") : t("user:mode.teen_on", "开启青少年模式")}</button>
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
    <PageLayout title={t("user:mode.care_title", "关怀模式")}>
      <div className="flex flex-col items-center py-10">
        <div className="w-16 h-16 bg-orange-500/10 rounded-full flex items-center justify-center mb-4">
          <span className="text-orange-500 text-2xl">❤️</span>
        </div>
        <h3 className="text-[18px] font-medium text-text-main mb-2">{t("user:mode.care_title", "关怀模式")}</h3>
        <p className="text-[14px] text-text-sub text-center px-8 mb-8">{t("user:mode.care_desc", "开启后，文字和按钮将变得更大，色彩更强。")}</p>
        <button
          onClick={handleToggle}
          className="w-[200px] h-12 bg-[#00B42A] text-white rounded-lg font-medium active:opacity-80 transition-opacity"
        >{enabled ? t("user:mode.care_off", "关闭关怀模式") : t("user:mode.care_on", "开启关怀模式")}</button>
      </div>
    </PageLayout>
  );
};

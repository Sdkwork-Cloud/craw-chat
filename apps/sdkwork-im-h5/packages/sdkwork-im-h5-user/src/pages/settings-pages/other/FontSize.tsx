import React, { useState, useEffect } from "react";
import { useTranslation } from 'react-i18next';
import { PageLayout } from "../../SettingsSubPages";
import { SettingsService } from "../../../services/SettingsService";

export const FontSize: React.FC = () => {
  const { t } = useTranslation();
  const [fontSize, setFontSize] = useState(2);

  useEffect(() => {
    SettingsService.getSettings().then((s) => setFontSize(s.fontSize));
  }, []);

  const handleFontSizeChange = async (
    e: React.ChangeEvent<HTMLInputElement>,
  ) => {
    const val = parseInt(e.target.value, 10);
    setFontSize(val);
    await SettingsService.updateSettings({ fontSize: val });
  };

  return (
    <PageLayout title={t("user:other_settings.font_size", "Font size")}>
      <div className="flex flex-col h-full">
        <div className="flex-1 p-6 flex flex-col gap-4">
          <div className="bg-chat-other-bg p-4 rounded-xl self-start max-w-[80%]">
            <p
              className="text-text-main"
              style={{ fontSize: `${14 + (fontSize - 2) * 2}px` }}
            >
              {t('user.auto_22ebf68', `预览字体大小`)}
            </p>
          </div>
          <div className="bg-primary-blue p-4 rounded-xl self-end max-w-[80%]">
            <p
              className="text-white"
              style={{ fontSize: `${14 + (fontSize - 2) * 2}px` }}
            >
              {t('user.auto_1f8461c5', `拖动下方滑块调整字体大小`)}
            </p>
          </div>
        </div>
        <div className="bg-chat-other-bg p-8 border-t border-border-color">
          <div className="flex items-center justify-between text-text-main mb-4">
            <span className="text-[12px]">A</span>
            <span className="text-[16px]">{t('user.auto_cea9f', `标准`)}</span>
            <span className="text-[24px]">A</span>
          </div>
          <input
            type="range"
            min="1"
            max="5"
            value={fontSize}
            onChange={handleFontSizeChange}
            className="w-full accent-[#00B42A]"
          />
        </div>
      </div>
    </PageLayout>
  );
};

import React, { useState, useEffect } from "react";
import { useTranslation } from 'react-i18next';
import { PageLayout, Group, ToggleItem } from "../../SettingsSubPages";
import { SettingsService } from "../../../services/SettingsService";

export const MediaSettings: React.FC = () => {
  const { t } = useTranslation();
  const [autoDownload, setAutoDownload] = useState(true);
  const [savePhoto, setSavePhoto] = useState(true);
  const [saveVideo, setSaveVideo] = useState(true);

  useEffect(() => {
    SettingsService.getSettings().then((s) => {
      setAutoDownload(s.autoDownload);
      setSavePhoto(s.savePhoto);
      setSaveVideo(s.saveVideo);
    });
  }, []);

  const handleToggle = async (key: string, val: boolean) => {
    if (key === "autoDownload") setAutoDownload(val);
    if (key === "savePhoto") setSavePhoto(val);
    if (key === "saveVideo") setSaveVideo(val);
    await SettingsService.updateSettings({ [key]: val });
  };

  return (
    <PageLayout title={t("user:other_settings.media_file_call", "照片、视频、文件和通话")}>
      <Group>
        <ToggleItem
          label={t('user.auto_prop_3c4e8950', "自动下载")}
          checked={autoDownload}
          onChange={(v: boolean) => handleToggle("autoDownload", v)}
          hideBorder
        />
      </Group>
      <p className="text-[13px] text-text-sub px-4 mb-4">
        {t('user.auto_2c00f43', `在其他设备查看的照片、视频和文件在手机上自动下载。`)}
      </p>
      <Group>
        <ToggleItem
          label={t('user.auto_prop_e2dc0', "照片")}
          checked={savePhoto}
          onChange={(v: boolean) => handleToggle("savePhoto", v)}
        />
        <ToggleItem
          label={t('user.auto_prop_11478b', "视频")}
          checked={saveVideo}
          onChange={(v: boolean) => handleToggle("saveVideo", v)}
          hideBorder
        />
      </Group>
      <p className="text-[13px] text-text-sub px-4 mb-4">
        {t('user.auto_5346231c', `拍摄或编辑后的照片和视频保存到系统相册。`)}
      </p>
    </PageLayout>
  );
};

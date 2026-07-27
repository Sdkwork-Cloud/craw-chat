import React, { useState } from "react";
import { useTranslation } from "react-i18next";
import { PageLayout, Group, ToggleItem } from "../SettingsSubPages";

export const MoreSecurity: React.FC = () => {
  const { t } = useTranslation();
  const [deviceCheck, setDeviceCheck] = useState(true);
  const [autoLock, setAutoLock] = useState(false);

  return (
    <PageLayout title={t("user:account_sec.more_sec_settings", "更多安全设置")}>
      <Group className="mt-4">
        <ToggleItem
          label={t("user:account_sec.device_protection", "新设备登录验证")}
          checked={deviceCheck}
          onChange={setDeviceCheck}
        />
        <ToggleItem
          label={t("user:account_sec.background_autolock", "后台自动锁定")}
          checked={autoLock}
          onChange={setAutoLock}
          hideBorder
        />
      </Group>
    </PageLayout>
  );
};

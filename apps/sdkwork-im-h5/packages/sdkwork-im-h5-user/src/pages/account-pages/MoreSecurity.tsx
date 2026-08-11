import React, { useState } from "react";
import { useTranslation } from "react-i18next";
import { PageLayout, Group, ToggleItem } from "../SettingsSubPages";

export const MoreSecurity: React.FC = () => {
  const { t } = useTranslation();
  const [deviceCheck, setDeviceCheck] = useState(true);
  const [autoLock, setAutoLock] = useState(false);

  return (
    <PageLayout title={t("user:account_sec.more_sec_settings", "More security settings")}>
      <Group className="mt-4">
        <ToggleItem
          label={t("user:account_sec.device_protection", "New device login verification")}
          checked={deviceCheck}
          onChange={setDeviceCheck}
        />
        <ToggleItem
          label={t("user:account_sec.background_autolock", "Auto-lock in background")}
          checked={autoLock}
          onChange={setAutoLock}
          hideBorder
        />
      </Group>
    </PageLayout>
  );
};

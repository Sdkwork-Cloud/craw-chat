import React, { useState } from "react";
import { useTranslation } from 'react-i18next';
import { PageLayout, Group, ToggleItem } from "../../SettingsSubPages";

export const AdManagement: React.FC = () => {
  const { t } = useTranslation();
  const [adEnabled, setAdEnabled] = useState(true);

  return (
    <PageLayout title={t('user.auto_prop_594e8d97', "个性化广告管理")}>
      <Group>
        <ToggleItem
          label={t('user.auto_prop_7a722ee4', "个性化广告")}
          checked={adEnabled}
          onChange={setAdEnabled}
          hideBorder
        />
      </Group>
      <p className="text-[13px] text-text-sub px-4 mb-4">
        {t('user.auto_2160cc48', `关闭后，您仍然会看到广告，但相关性会降低。`)}
      </p>
    </PageLayout>
  );
};

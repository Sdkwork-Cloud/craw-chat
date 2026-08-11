import React from "react";
import { useTranslation } from 'react-i18next';
import { PageLayout, Group, ListItem } from "../../SettingsSubPages";

export const SystemPermissions: React.FC = () => {
  const { t } = useTranslation();
  return (
    <PageLayout title={t('user.auto_prop_7249bbf6', "System permissions")}>
      <Group>
        <ListItem label={t('user.auto_prop_eb994', "Album")} rightText={t('user.auto_prop_16d1e2d', "Granted")} />
        <ListItem label={t('user.auto_prop_ecf42', "Camera")} rightText={t('user.auto_prop_16d1e2d', "Granted")} />
        <ListItem label={t('user.auto_prop_25dfe09', "Microphone")} rightText={t('user.auto_prop_16d1e2d', "Granted")} />
        <ListItem label={t('user.auto_prop_25f4ba2f', "Location")} rightText={t('user.auto_prop_47452fd2', "While using the app")} hideBorder />
      </Group>
    </PageLayout>
  );
};

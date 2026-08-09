import React from "react";
import { useTranslation } from 'react-i18next';
import { PageLayout, Group, ListItem } from "../../SettingsSubPages";

export const SystemPermissions: React.FC = () => {
  const { t } = useTranslation();
  return (
    <PageLayout title={t('user.auto_prop_7249bbf6', "系统权限管理")}>
      <Group>
        <ListItem label={t('user.auto_prop_eb994', "相册")} rightText={t('user.auto_prop_16d1e2d', "已授权")} />
        <ListItem label={t('user.auto_prop_ecf42', "相机")} rightText={t('user.auto_prop_16d1e2d', "已授权")} />
        <ListItem label={t('user.auto_prop_25dfe09', "麦克风")} rightText={t('user.auto_prop_16d1e2d', "已授权")} />
        <ListItem label={t('user.auto_prop_25f4ba2f', "位置信息")} rightText={t('user.auto_prop_47452fd2', "使用应用期间")} hideBorder />
      </Group>
    </PageLayout>
  );
};

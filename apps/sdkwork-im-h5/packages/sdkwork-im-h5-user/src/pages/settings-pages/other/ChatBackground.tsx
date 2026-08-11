import React from "react";
import { useTranslation } from 'react-i18next';
import { PageLayout, Group, ListItem } from "../../SettingsSubPages";
import { showToast } from "@sdkwork/im-h5-commons";

export const ChatBackground: React.FC = () => {
  const { t } = useTranslation();
  return (
    <PageLayout title={t("user:other_settings.chat_bg", "Chat wallpaper")}>
      <Group>
        <ListItem label={t('user.auto_prop_1c9de83b', "Choose a wallpaper")} onClick={() => showToast(t("commons.feature_unavailable", "This feature is not available yet while the real service is being integrated."))} />
        <ListItem label={t('user.auto_prop_583883b1', "Choose from photo album")} onClick={() => showToast(t("commons.feature_unavailable", "This feature is not available yet while the real service is being integrated."))} />
        <ListItem label={t('user.auto_prop_17cb4ad', "Take a photo")} hideBorder onClick={() => showToast(t("commons.feature_unavailable", "This feature is not available yet while the real service is being integrated."))} />
      </Group>
      <Group>
        <ListItem
          label={t('user.auto_prop_6b0c5030', "Apply the wallpaper to all chats")}
          hideBorder
          onClick={() => showToast(t("commons.feature_unavailable", "This feature is not available yet while the real service is being integrated."))}
        />
      </Group>
    </PageLayout>
  );
};

import React from "react";
import { useTranslation } from 'react-i18next';
import { PageLayout, Group, ListItem } from "../../SettingsSubPages";
import { showToast } from "@sdkwork/im-h5-commons";

export const ChatBackground: React.FC = () => {
  const { t } = useTranslation();
  return (
    <PageLayout title={t("user:other_settings.chat_bg", "聊天背景")}>
      <Group>
        <ListItem label={t('user.auto_prop_1c9de83b', "选择背景图")} onClick={() => showToast("已应用")} />
        <ListItem label={t('user.auto_prop_583883b1', "从手机相册选择")} onClick={() => showToast("已应用")} />
        <ListItem label={t('user.auto_prop_17cb4ad', "拍一张")} hideBorder onClick={() => showToast("已应用")} />
      </Group>
      <Group>
        <ListItem
          label={t('user.auto_prop_6b0c5030', "将背景应用到所有聊天场景")}
          hideBorder
          onClick={() => showToast("全局应用成功")}
        />
      </Group>
    </PageLayout>
  );
};

import React, { useState } from "react";
import { PageLayout, Group, ListItem } from "../../components/SettingsCommons";
import { Check } from "lucide-react";
import { showPrompt, showToast } from "@sdkwork/im-h5-commons";
import { useNavigate } from "react-router";
import { useTranslation } from 'react-i18next';

export const Gender = () => {
  const { t } = useTranslation();
const [gender, setGender] = useState(t("user:profile.male", t("settings:profile.male", "男")));
  return (
    <PageLayout title={t("user:profile.set_gender", "设置性别")}>
      <Group>
        <div
          onClick={() => setGender(t("user:profile.male", t("settings:profile.male", "男")))}
          className="flex items-center justify-between px-4 py-3.5 bg-chat-other-bg border-b border-border-color/60 cursor-pointer"
        >
          <span className="text-[16px] text-text-main">{t("user:profile.male", t("settings:profile.male", "男"))}</span>{gender === t("user:profile.male", t("settings:profile.male", "男")) &&<Check className="w-5 h-5 text-accent-green" />}
        </div>
        <div
          onClick={() => setGender(t("user:profile.female", "女"))}
          className="flex items-center justify-between px-4 py-3.5 bg-chat-other-bg cursor-pointer"
        >
          <span className="text-[16px] text-text-main">{t("user:profile.female", "女")}</span>{gender === t("user:profile.female", "女") &&<Check className="w-5 h-5 text-accent-green" />}
        </div>
      </Group>
    </PageLayout>
  );
};

export const Region = () => {
  const { t } = useTranslation();
  
return (
  <PageLayout title={t("user:profile.set_region", "设置地区")}>
    <Group>
      <ListItem label={t("user:profile.china", "中国大陆")} rightText={t("user:profile.beijing", "北京")} hideBorder />
    </Group>
  </PageLayout>
);
};

export const Signature = () => {
  const { t } = useTranslation();
  
return (
  <PageLayout title={t("user:profile.signature", t("settings:profile.signature", "个性签名"))}>
    <div className="p-4">
      <textarea
        className="w-full h-32 bg-chat-other-bg p-4 rounded-xl text-text-main outline-none resize-none"
        placeholder={t("user:profile.sig_placeholder", "介绍一下自己吧...")}
      ></textarea>
      <button
        className="mt-6 w-full h-12 bg-accent-green text-white rounded-lg font-medium active:opacity-80 transition-opacity"
        onClick={() => showToast(t("user:profile.save_success", "保存成功"))}
      >{t("settings:profile.save", "保存")}</button>
    </div>
  </PageLayout>
);
};

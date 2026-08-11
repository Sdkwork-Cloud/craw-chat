import React, { useState } from "react";
import { PageLayout, Group, ListItem } from "../../components/SettingsCommons";
import { Check } from "lucide-react";
import { showPrompt, showToast } from "@sdkwork/im-h5-commons";
import { useNavigate } from "react-router";
import { useTranslation } from 'react-i18next';

export const Gender = () => {
  const { t } = useTranslation();
const [gender, setGender] = useState(t("user:profile.male", t("settings:profile.male", "Male")));
  return (
    <PageLayout title={t("user:profile.set_gender", "Set gender")}>
      <Group>
        <div
          onClick={() => setGender(t("user:profile.male", t("settings:profile.male", "Male")))}
          className="flex items-center justify-between px-4 py-3.5 bg-chat-other-bg border-b border-border-color/60 cursor-pointer"
        >
          <span className="text-[16px] text-text-main">{t("user:profile.male", t("settings:profile.male", "Male"))}</span>{gender === t("user:profile.male", t("settings:profile.male", "Male")) &&<Check className="w-5 h-5 text-accent-green" />}
        </div>
        <div
          onClick={() => setGender(t("user:profile.female", "Female"))}
          className="flex items-center justify-between px-4 py-3.5 bg-chat-other-bg cursor-pointer"
        >
          <span className="text-[16px] text-text-main">{t("user:profile.female", "Female")}</span>{gender === t("user:profile.female", "Female") &&<Check className="w-5 h-5 text-accent-green" />}
        </div>
      </Group>
    </PageLayout>
  );
};

export const Region = () => {
  const { t } = useTranslation();
  
return (
  <PageLayout title={t("user:profile.set_region", "Set region")}>
    <Group>
      <ListItem label={t("user:profile.china", "Mainland China")} rightText={t("user:profile.beijing", "Beijing")} hideBorder />
    </Group>
  </PageLayout>
);
};

export const Signature = () => {
  const { t } = useTranslation();
  
return (
  <PageLayout title={t("user:profile.signature", t("settings:profile.signature", "Signature"))}>
    <div className="p-4">
      <textarea
        className="w-full h-32 bg-chat-other-bg p-4 rounded-xl text-text-main outline-none resize-none"
        placeholder={t("user:profile.sig_placeholder", "Introduce yourself...")}
      ></textarea>
      <button
        className="mt-6 w-full h-12 bg-accent-green text-white rounded-lg font-medium active:opacity-80 transition-opacity"
        onClick={() => showToast(t("commons.feature_unavailable", "This feature is not available yet while the real service is being integrated."))}
      >{t("settings:profile.save", "Save")}</button>
    </div>
  </PageLayout>
);
};

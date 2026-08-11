import React from "react";
import { useNavigate } from "react-router";
import { useTranslation } from 'react-i18next';
import {
  showToast,
  Avatar,
} from "@sdkwork/im-h5-commons";
import { PageLayout, Group, ListItem } from "../../components/SettingsCommons";

export const AccountSecurity = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  return (
    <PageLayout title={t("user:settings.account_security", "Account & security")}>
      <Group>
        <ListItem
          label={t('user.auto_prop_1712c64', 'WeChat ID')}
          rightText=""
          onClick={() => navigate("/settings/account/wechat-id")}
        />
        <ListItem
          label={t("user:account_sec.phone", "Phone number")}
          rightText=""
          hideBorder
          onClick={() => navigate("/settings/account/phone")}
        />
      </Group>
      <Group>
        <ListItem
          label={t('user.auto_prop_2cb5ca2e', 'WeChat password')}
          onClick={() => navigate("/settings/account/password")}
        />
        <ListItem
          label={t("user:account_sec.voice_lock", "Voice lock")}
          rightText={t("user:account_sec.not_set", "Not set")}
          hideBorder
          onClick={() => navigate("/settings/account/voice-lock")}
        />
      </Group>
      <Group>
        <ListItem
          label={t("user:account_sec.emergency_contacts", "Emergency contacts")}
          hideBorder
          onClick={() => navigate("/settings/account/emergency")}
        />
      </Group>
      <Group>
        <ListItem
          label={t('user.auto_prop_n51f86178', 'Login device management')}
          hideBorder
          onClick={() => navigate("/settings/devices")}
        />
      </Group>
      <Group>
        <ListItem
          label={t('user.auto_prop_n3ed8e3ab', 'More security settings')}
          hideBorder
          onClick={() => navigate("/settings/account/more")}
        />
      </Group>
    </PageLayout>
  );
};

/**
 * Login-device management — fail-closed (PRD).
 *
 * No real IAM session/device-list API is composed: the previously hard-coded
 * device rows and the fake "已下线该设备" success were fabricated and are
 * removed. The page renders the typed unavailable state instead.
 */
export const Devices = () => {
  const { t } = useTranslation();

  return (
    <PageLayout title={t('user.auto_prop_114509', 'Devices')}>
      <div className="flex flex-col items-center justify-center gap-3 px-8 text-center py-20">
        <span className="text-[40px]">🖥️</span>
        <p className="text-[15px] text-text-main">
          {t("commons.feature_unavailable", "This feature is not available yet while the real service is being integrated.")}
        </p>
      </div>
    </PageLayout>
  );
};

export const SwitchAccount = () => {
  const { t } = useTranslation();
  
return (
    <PageLayout title={t('user.auto_prop_26d01b0c', 'Switch account')}>
      <div className="flex flex-col items-center py-10">
        <Avatar
          fallback="?"
          size="lg"
          className="w-20 h-20 rounded-full mb-4"
        />
        <h3 className="text-[18px] font-medium text-text-main mb-8">{t('user.auto_2c9b5a6b', 'Current account')}</h3>
        <button
          className="w-[200px] h-12 bg-chat-other-bg text-text-main rounded-lg font-medium active:bg-active-bg transition-colors mb-4 border border-border-color"
          onClick={() =>
            showToast(t("commons.feature_unavailable", "This feature is not available yet while the real service is being integrated."))
          }
        >{t('user.auto_7e69752b', '+ Add account')}</button>
      </div>
    </PageLayout>
  );
};

import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { useTranslation } from 'react-i18next';
import { ChevronLeft } from "lucide-react";
import { IconButton, showConfirm } from "@sdkwork/im-h5-commons";
import { requestImH5SessionLogout } from "@sdkwork/im-h5-core/session";
import { SettingsService } from "../services/SettingsService";
import { Group, ListItem } from "../components/SettingsCommons";

export const SettingsPage: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const [isLoggingOut, setIsLoggingOut] = useState(false);
  const [teenMode, setTeenMode] = useState(false);
  const [elderlyMode, setElderlyMode] = useState(false);

  useEffect(() => {
    SettingsService.getSettings().then((s) => {
      setTeenMode(s.teenMode);
      setElderlyMode(s.elderlyMode);
    });
  }, []);

  const handleLogout = async () => {
    if (isLoggingOut) return;
    const confirmed = await showConfirm(
      t("user:settings.logout_confirm", "You will stop receiving new message notifications after logging out. Log out?"),
    );
    if (!confirmed) return;
    setIsLoggingOut(true);
    try {
      await requestImH5SessionLogout();
    } catch {
      // The runtime always clears the local session (server revoke failures
      // included), so AuthGate falls back to the login screen either way.
    } finally {
      setIsLoggingOut(false);
    }
  };

  return (
    <div className="flex flex-col h-full bg-bg-color overflow-y-auto">
      {/* Header */}
      <header className="h-[56px] flex items-center justify-between px-1 sticky top-0 z-10 shrink-0 pt-safe bg-bg-color/90 backdrop-blur-xl">
        <div className="flex items-center z-10 flex-1">
          <IconButton
            icon={
              <ChevronLeft
                className="w-6 h-6 text-text-main"
                strokeWidth={2.5}
              />
            }
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="absolute inset-x-0 flex items-center justify-center pointer-events-none">
          <h2 className="text-[17px] font-bold text-text-main tracking-tight">{t('user.auto_116b70', 'Settings')}</h2>
        </div>
        <div className="flex-1" />
      </header>

      <div className="flex flex-col pb-12 mt-2">
        <Group>
          <ListItem
            label={t("user:settings.account_security", "Account & security")}
            value={t('user.auto_prop_16ab7d9', 'Protected')}
            hideBorder
            onClick={() => navigate("/settings/account")}
          />
        </Group>

        <Group>
          <ListItem
            label={t("user:settings.teen_mode", "Teen mode")}
            value={teenMode ? "已开启" : "未开启"}
            onClick={() => navigate("/settings/teen-mode")}
          />
          <ListItem
            label={t("user:settings.care_mode", "Care mode")}
            value={elderlyMode ? "已开启" : "未开启"}
            hideBorder
            onClick={() => navigate("/settings/elderly-mode")}
          />
        </Group>

        <Group>
          <ListItem
            label={t("user:settings.message_notifications", "New message notifications")}
            onClick={() => navigate("/settings/notifications")}
          />
          <ListItem label={t("user:settings.chat", "Chats")} onClick={() => navigate("/settings/chat")} />
          <ListItem
            label={t('user.auto_prop_114509', 'Devices')}
            onClick={() => navigate("/settings/devices")}
          />
          <ListItem
            label={t("user:settings.general", "General")}
            hideBorder
            onClick={() => navigate("/settings/general")}
          />
        </Group>

        <Group>
          <ListItem
            label={t('user.auto_prop_301edd8d', 'Friend permissions')}
            onClick={() => navigate("/settings/friend-permissions")}
          />
          <ListItem
            label={t('user.auto_prop_n421afd43', 'Personal info & permissions')}
            onClick={() => navigate("/settings/privacy")}
          />
          <ListItem
            label={t('user.auto_prop_9efd9be', 'Personal data collection list')}
            onClick={() => navigate("/settings/info-collection")}
          />
          <ListItem
            label={t('user.auto_prop_7362f474', 'Third-party data sharing list')}
            hideBorder
            onClick={() => navigate("/settings/third-party-sharing")}
          />
        </Group>

        <Group>
          <ListItem
            label={t("user:settings.plugins", "Plugins")}
            hideBorder
            onClick={() => navigate("/settings/plugins")}
          />
        </Group>

        <Group>
          <ListItem
            label={t("user:settings.help_feedback", "Help & feedback")}
            onClick={() => navigate("/settings/help")}
          />
          <ListItem
            label={t("user:settings.about", "About Sdkwork IM H5")}
            value={t('user.auto_prop_701c7979', 'Version 1.0.0')}
            hideBorder
            onClick={() => navigate("/settings/about")}
          />
        </Group>

        <Group className="mt-4">
          <ListItem
            label={t("user:settings.switch_account", "Switch account")}
            onClick={() => navigate("/settings/switch-account")}
          />
          <ListItem
            label={isLoggingOut
              ? t("user:settings.logging_out", "Logging out...")
              : t("user:settings.logout", "Log out")}
            danger
            hideBorder
            onClick={handleLogout}
          />
        </Group>
      </div>
    </div>
  );
};


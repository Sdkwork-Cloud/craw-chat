import React from "react";
import { useNavigate } from "react-router";
import { useTranslation } from "react-i18next";
import { PageLayout, Group, ListItem, ToggleItem } from "../SettingsSubPages";
import { showToast } from "@sdkwork/im-h5-commons";

/**
 * Voice lock — fail-closed (PRD).
 *
 * Voice-lock enrollment/verification has no composed backend flow; the toggle
 * must not pretend to enable a security feature. The toggle is read-only and
 * surfaces the typed unavailable state on interaction.
 */
export const VoiceLock: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();

  const unavailable = () =>
    showToast(t("commons.feature_unavailable", "This feature is not available yet while the real service is being integrated."));

  return (
    <PageLayout title={t("user:account_sec.voice_lock", "Voice lock")}>
      <Group className="mt-4">
        <ToggleItem
          label={t("user:account_sec.login_sdkwork_im_h5", "Log into Sdkwork IM H5")}
          checked={false}
          onChange={unavailable}
          hideBorder
        />
      </Group>
      <p className="text-[13px] text-text-sub px-4 mb-8">{t("user:account_sec.voice_lock_desc", "When enabled, you can unlock the app or verify your identity with your voice.")}</p>
      <Group>
        <ListItem
          label={t("user:account_sec.reset_voice_lock", "Reset voice lock")}
          hideBorder
          onClick={unavailable}
        />
      </Group>
    </PageLayout>
  );
};

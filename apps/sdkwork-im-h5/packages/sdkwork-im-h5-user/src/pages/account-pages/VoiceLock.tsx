import React, { useState } from "react";
import { useNavigate } from "react-router";
import { useTranslation } from "react-i18next";
import { PageLayout, Group, ListItem, ToggleItem } from "../SettingsSubPages";

export const VoiceLock: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const [enabled, setEnabled] = useState(false);

  return (
    <PageLayout title={t("user:account_sec.voice_lock", "声音锁")}>
      <Group className="mt-4">
        <ToggleItem
          label={t("user:account_sec.login_sdkwork_im_h5", "登录 Sdkwork IM H5")}
          checked={enabled}
          onChange={setEnabled}
          hideBorder
        />
      </Group>
      <p className="text-[13px] text-text-sub px-4 mb-8">{t("user:account_sec.voice_lock_desc", "开启后，可以使用声音解锁应用或验证身份。")}</p>
      <Group>
        <ListItem
          label={t("user:account_sec.reset_voice_lock", "重设声音锁")}
          hideBorder
          onClick={() => navigate("/settings/account/voice-lock/reset")}
        />
      </Group>
    </PageLayout>
  );
};

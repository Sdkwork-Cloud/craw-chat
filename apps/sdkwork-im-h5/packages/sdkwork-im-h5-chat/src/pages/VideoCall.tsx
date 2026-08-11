import React from "react";
import { useTranslation } from "react-i18next";
import { VideoOff } from "lucide-react";
import { CapabilityUnavailablePage } from "@sdkwork/im-h5-commons";

/**
 * Video call surface — fail-closed (PRD).
 *
 * No end-to-end signaling/media wiring exists for H5 video calls. This page
 * must never simulate a connection or show placeholder "remote video" frames:
 * it renders the typed unavailable state. The chat header keeps the call
 * entry point; every entry lands here until a real call capability (RTC media
 * from sdkwork-rtc + IM call signaling) is composed.
 */
export const VideoCall: React.FC = () => {
  const { t } = useTranslation();

  return (
    <CapabilityUnavailablePage
      icon={VideoOff}
      title={t("chat.call.unavailable_title", "Calls are not available yet")}
      message={t("chat.call.unavailable_desc", "Call signaling and media are not connected yet; calls cannot be started right now.")}
      onBack={() => window.history.back()}
    />
  );
};

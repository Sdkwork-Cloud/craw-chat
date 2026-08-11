import React from "react";
import { useTranslation } from "react-i18next";
import { PhoneOff } from "lucide-react";
import { CapabilityUnavailablePage } from "@sdkwork/im-h5-commons";

/**
 * Voice call surface — fail-closed (PRD).
 *
 * Call signaling (`/im/v3/api/calls/*`) and the WebSocket call workflow are
 * IM-owned, but the H5 voice-call flow has no end-to-end signaling/media
 * wiring yet. This page must never simulate a connection: it renders the
 * typed unavailable state. The chat header keeps the call entry point; every
 * entry lands here until a real call capability is composed.
 */
export const VoiceCall: React.FC = () => {
  const { t } = useTranslation();

  return (
    <CapabilityUnavailablePage
      icon={PhoneOff}
      title={t("chat.call.unavailable_title", "Calls are not available yet")}
      message={t("chat.call.unavailable_desc", "Call signaling and media are not connected yet; calls cannot be started right now.")}
      onBack={() => window.history.back()}
    />
  );
};

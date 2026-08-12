import React from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { UserX } from "lucide-react";
import { CapabilityUnavailablePage } from "@sdkwork/im-h5-commons";

/**
 * Shared fail-closed pages for legacy user surfaces (moments, characters,
 * works, voice, billing, life services) that own no browser business state
 * and have no composed owner SDK surface in the current H5 release.
 */
export const LegacyUserSurfaceUnavailablePage: React.FC<{
  titleKey: string;
  messageKey?: string;
}> = ({ titleKey, messageKey }) => {
  const { t } = useTranslation();
  const navigate = useNavigate();

  return (
    <CapabilityUnavailablePage
      icon={UserX}
      title={t(titleKey)}
      message={t(messageKey ?? "user.surface_unavailable", "This surface is unavailable until its owner SDK is composed.")}
      onBack={() => navigate(-1)}
    />
  );
};

export default LegacyUserSurfaceUnavailablePage;

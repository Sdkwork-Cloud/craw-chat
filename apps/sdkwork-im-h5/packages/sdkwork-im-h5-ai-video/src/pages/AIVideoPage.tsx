import { VideoOff } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";

import { CapabilityUnavailablePage } from "@sdkwork/im-h5-commons";

export function AIVideoPage() {
  const { t } = useTranslation("ai_video");
  const navigate = useNavigate();

  return (
    <CapabilityUnavailablePage
      icon={VideoOff}
      message={t("unavailable")}
      onBack={() => navigate(-1)}
      title={t("title")}
    />
  );
}

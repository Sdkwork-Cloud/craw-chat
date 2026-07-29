import { RadioTower } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";

import { CapabilityUnavailablePage } from "@sdkwork/im-h5-commons";

export function ChannelsPage() {
  const { t } = useTranslation("channels");
  const navigate = useNavigate();

  return (
    <CapabilityUnavailablePage
      icon={RadioTower}
      message={t("unavailable")}
      onBack={() => navigate(-1)}
      title={t("title")}
    />
  );
}

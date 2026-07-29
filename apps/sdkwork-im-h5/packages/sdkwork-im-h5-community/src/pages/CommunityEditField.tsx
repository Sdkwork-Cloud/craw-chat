import { Users } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";

import { CapabilityUnavailablePage } from "@sdkwork/im-h5-commons";

export function CommunityEditField() {
  const { t } = useTranslation("community");
  const navigate = useNavigate();

  return (
    <CapabilityUnavailablePage
      icon={Users}
      message={t("unavailable")}
      onBack={() => navigate(-1)}
      title={t("title")}
    />
  );
}

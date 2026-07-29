import { Building2 } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";

import { CapabilityUnavailablePage } from "@sdkwork/im-h5-commons";

export function EnterpriseRecruitment() {
  const { t } = useTranslation("enterprise");
  const navigate = useNavigate();

  return (
    <CapabilityUnavailablePage
      icon={Building2}
      message={t("unavailable")}
      onBack={() => navigate(-1)}
      title={t("title")}
    />
  );
}

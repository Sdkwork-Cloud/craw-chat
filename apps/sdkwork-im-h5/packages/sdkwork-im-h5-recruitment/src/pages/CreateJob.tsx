import { Briefcase } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";

import { CapabilityUnavailablePage } from "@sdkwork/im-h5-commons";

export function CreateJob() {
  const { t } = useTranslation("recruitment");
  const navigate = useNavigate();

  return (
    <CapabilityUnavailablePage
      icon={Briefcase}
      message={t("unavailable")}
      onBack={() => navigate(-1)}
      title={t("title")}
    />
  );
}

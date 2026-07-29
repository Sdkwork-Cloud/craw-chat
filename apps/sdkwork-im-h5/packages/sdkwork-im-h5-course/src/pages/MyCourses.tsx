import { GraduationCap } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";

import { CapabilityUnavailablePage } from "@sdkwork/im-h5-commons";

export function MyCourses() {
  const { t } = useTranslation("course");
  const navigate = useNavigate();

  return (
    <CapabilityUnavailablePage
      icon={GraduationCap}
      message={t("unavailable")}
      onBack={() => navigate(-1)}
      title={t("title")}
    />
  );
}

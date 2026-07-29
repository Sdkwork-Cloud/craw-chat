import { MapPinOff } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";

import { CapabilityUnavailablePage } from "@sdkwork/im-h5-commons";

export function AttendanceApp() {
  const { t } = useTranslation("attendance");
  const navigate = useNavigate();

  return (
    <CapabilityUnavailablePage
      icon={MapPinOff}
      message={t("unavailable")}
      onBack={() => navigate(-1)}
      title={t("title")}
    />
  );
}

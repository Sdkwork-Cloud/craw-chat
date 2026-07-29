import { CalendarX2 } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";

import { CapabilityUnavailablePage } from "@sdkwork/im-h5-commons";

export function CalendarWorkspace() {
  const { t } = useTranslation("calendar");
  const navigate = useNavigate();

  return (
    <CapabilityUnavailablePage
      icon={CalendarX2}
      message={t("unavailable")}
      onBack={() => navigate(-1)}
      title={t("title")}
    />
  );
}

import { FileX2 } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";

import { CapabilityUnavailablePage } from "@sdkwork/im-h5-commons";

export function AIWritingPage() {
  const { t } = useTranslation("ai_writing");
  const navigate = useNavigate();

  return (
    <CapabilityUnavailablePage
      icon={FileX2}
      message={t("unavailable")}
      onBack={() => navigate(-1)}
      title={t("header_title")}
    />
  );
}

import { Database } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";

import { CapabilityUnavailablePage } from "@sdkwork/im-h5-commons";

export function CreateKnowledgeBase() {
  const { t } = useTranslation("knowledge");
  const navigate = useNavigate();

  return (
    <CapabilityUnavailablePage
      icon={Database}
      message={t("unavailable")}
      onBack={() => navigate(-1)}
      title={t("title")}
    />
  );
}

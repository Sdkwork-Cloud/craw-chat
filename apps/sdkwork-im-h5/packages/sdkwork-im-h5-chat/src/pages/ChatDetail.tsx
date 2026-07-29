import { MessageSquareWarning } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";

import { CapabilityUnavailablePage } from "@sdkwork/im-h5-commons";

export function ChatDetail() {
  const navigate = useNavigate();
  const { t } = useTranslation("chat");

  return (
    <CapabilityUnavailablePage
      icon={MessageSquareWarning}
      message={t("detail.legacy_unavailable")}
      onBack={() => navigate(-1)}
      title={t("detail.legacy_title")}
    />
  );
}

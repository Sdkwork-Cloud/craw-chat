import { ClipboardX } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";

import { CapabilityUnavailablePage } from "@sdkwork/im-h5-commons";

export function ApprovalApp() {
  const { t } = useTranslation("approval");
  const navigate = useNavigate();

  return (
    <CapabilityUnavailablePage
      icon={ClipboardX}
      message={t("unavailable")}
      onBack={() => navigate(-1)}
      title={t("title")}
    />
  );
}

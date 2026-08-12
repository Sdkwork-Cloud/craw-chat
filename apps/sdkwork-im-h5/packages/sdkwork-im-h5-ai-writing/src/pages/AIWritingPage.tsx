import { CapabilityUnavailablePage } from "@sdkwork/im-h5-commons";
import { PenLine } from "lucide-react";
import { useNavigate } from "react-router";
import { useTranslation } from "react-i18next";

/**
 * AI Writing — fail-closed (PRD): the owning capability has no composed SDK
 * surface in the current H5 release. The page renders a typed unavailable
 * state instead of fabricated generated content.
 */
export const AIWritingPage: React.FC = () => {
  const { t } = useTranslation("ai_writing");
  const navigate = useNavigate();

  return (
    <CapabilityUnavailablePage
      icon={PenLine}
      title={t("title", "AI Writing")}
      message={t("unavailable", "AI Writing is unavailable until its owner SDK is composed.")}
      onBack={() => navigate(-1)}
    />
  );
};

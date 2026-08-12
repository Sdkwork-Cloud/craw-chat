
import { CapabilityUnavailablePage } from "@sdkwork/im-h5-commons";
import { FilePlus2 } from "lucide-react";
import { useNavigate } from "react-router";
import { useTranslation } from "react-i18next";

/**
 * CreateReport — fail-closed (PRD): the owning capability has no composed SDK
 * surface in the current H5 release. The page renders a typed unavailable
 * state instead of fabricated business data.
 */
export const CreateReport = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();

  return (
    <CapabilityUnavailablePage
      icon={FilePlus2}
      title={t("report.create.title", "CreateReport")}
      message={t("report.create.unavailable", "This feature is unavailable until its owner SDK is composed.")}
      onBack={() => navigate(-1)}
    />
  );
};

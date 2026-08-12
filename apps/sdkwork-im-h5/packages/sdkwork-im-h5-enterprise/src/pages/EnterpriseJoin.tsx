
import { CapabilityUnavailablePage } from "@sdkwork/im-h5-commons";
import { Building2 } from "lucide-react";
import { useNavigate } from "react-router";
import { useTranslation } from "react-i18next";

/**
 * EnterpriseJoin — fail-closed (PRD): the owning capability has no composed SDK
 * surface in the current H5 release. The page renders a typed unavailable
 * state instead of fabricated business data.
 */
export const EnterpriseJoin = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();

  return (
    <CapabilityUnavailablePage
      icon={Building2}
      title={t("enterprise.join.title", "EnterpriseJoin")}
      message={t("enterprise.join.unavailable", "This feature is unavailable until its owner SDK is composed.")}
      onBack={() => navigate(-1)}
    />
  );
};

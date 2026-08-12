import { useTranslation } from "react-i18next";
import React from "react";
import { CapabilityUnavailablePage } from "@sdkwork/im-h5-commons";
import { Building2 } from "lucide-react";
import { useNavigate } from "react-router";

export const EnterpriseCenter = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();

  // Enterprise Center — fail-closed (PRD): no composed owner SDK surface
  // exists for the enterprise directory, so fabricated enterprise, supply,
  // demand, and recruitment listings are removed. The page renders a typed
  // unavailable state instead of synthetic business data.

  return (
    <CapabilityUnavailablePage
      icon={Building2}
      title={t("enterprise.auto_prop_2518cc2f", "Business Center")}
      message={t("enterprise.unavailable", "Enterprise Center is unavailable until its owner SDK is composed.")}
      onBack={() => navigate(-1)}
    />
  );
};



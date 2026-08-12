
import { CapabilityUnavailablePage } from "@sdkwork/im-h5-commons";
import { Briefcase } from "lucide-react";
import { useNavigate } from "react-router";
import { useTranslation } from "react-i18next";

/**
 * CreateJob — fail-closed (PRD): the owning capability has no composed SDK
 * surface in the current H5 release. The page renders a typed unavailable
 * state instead of fabricated business data.
 */
export const CreateJob = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();

  return (
    <CapabilityUnavailablePage
      icon={Briefcase}
      title={t("recruitment.createJob.title", "CreateJob")}
      message={t("recruitment.createJob.unavailable", "This feature is unavailable until its owner SDK is composed.")}
      onBack={() => navigate(-1)}
    />
  );
};

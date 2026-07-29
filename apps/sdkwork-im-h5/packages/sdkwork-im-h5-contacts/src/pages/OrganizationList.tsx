import { Building2 } from "lucide-react";
import { useTranslation } from "react-i18next";

import { CapabilityUnavailablePage } from "../components/CapabilityUnavailablePage";

export function OrganizationList() {
  const { t } = useTranslation();
  return (
    <CapabilityUnavailablePage
      icon={Building2}
      message={t(
        'contacts.organization_unavailable',
        'The organization directory is not available in this build.',
      )}
      title={t('contacts.my_orgs')}
    />
  );
}

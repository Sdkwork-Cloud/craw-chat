import { Bot } from "lucide-react";
import { useTranslation } from "react-i18next";

import { CapabilityUnavailablePage } from "../components/CapabilityUnavailablePage";

export function AgentSearch() {
  const { t } = useTranslation();
  return (
    <CapabilityUnavailablePage
      icon={Bot}
      message={t(
        'contacts.agents_unavailable',
        'Agent catalog access is not available in this build.',
      )}
      title={t('contacts.search_agents')}
    />
  );
}

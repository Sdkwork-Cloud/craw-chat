import { Bot } from "lucide-react";
import { useTranslation } from "react-i18next";

import { CapabilityUnavailablePage } from "../components/CapabilityUnavailablePage";

export function AgentCreate() {
  const { t } = useTranslation();
  return (
    <CapabilityUnavailablePage
      icon={Bot}
      message={t(
        'contacts.agent_lifecycle_unavailable',
        'Agent lifecycle operations are not available in this build.',
      )}
      title={t('contacts.create_agent')}
    />
  );
}

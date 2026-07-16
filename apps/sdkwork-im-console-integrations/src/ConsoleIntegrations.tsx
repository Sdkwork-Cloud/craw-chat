import React from 'react';
import { ConsoleContractEmptyState } from '@sdkwork/im-pc-commons';
import { CONSOLE_INTEGRATION_CONTRACT_UNAVAILABLE } from './services/IntegrationService';

/**
 * Keep the retired route explicit for existing deep links. Navigation omits it
 * until a generated app SDK capability provides an authoritative API.
 */
export const ConsoleIntegrations = () => (
  <ConsoleContractEmptyState
    title="Console integrations unavailable"
    description={CONSOLE_INTEGRATION_CONTRACT_UNAVAILABLE}
  />
);

import React from 'react';
import { ConsoleContractEmptyState } from '@sdkwork/im-pc-commons';
import { CONSOLE_SETTINGS_CONTRACT_UNAVAILABLE } from './services/SysSettingsService';

/**
 * This legacy route remains mounted to give bookmarked URLs an explicit,
 * fail-closed result until a generated app SDK capability is available.
 */
export const ConsoleSettings = () => (
  <ConsoleContractEmptyState
    title="Console settings unavailable"
    description={CONSOLE_SETTINGS_CONTRACT_UNAVAILABLE}
  />
);

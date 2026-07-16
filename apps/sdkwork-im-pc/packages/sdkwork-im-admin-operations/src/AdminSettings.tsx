import React from 'react';
import { ConsoleContractEmptyState } from '@sdkwork/im-pc-commons';
import { ADMIN_SETTINGS_CONTRACT_UNAVAILABLE } from './services/AdminSettingsService';

/**
 * Preserve direct links but fail closed until the backend-admin SDK provides a
 * platform-settings capability.
 */
export const AdminSettings = () => (
  <ConsoleContractEmptyState
    title="Platform settings unavailable"
    description={ADMIN_SETTINGS_CONTRACT_UNAVAILABLE}
  />
);

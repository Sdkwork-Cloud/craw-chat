import React from 'react';
import { ConsoleContractEmptyState } from '@sdkwork/im-pc-commons';
import { ADMIN_ANNOUNCEMENT_CONTRACT_UNAVAILABLE } from './services/AdminAnnouncementService';

/**
 * Preserve direct links but fail closed until the backend-admin SDK provides a
 * platform-announcement capability.
 */
export const AdminAnnouncements = () => (
  <ConsoleContractEmptyState
    title="Platform announcements unavailable"
    description={ADMIN_ANNOUNCEMENT_CONTRACT_UNAVAILABLE}
  />
);

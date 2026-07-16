/**
 * The generated backend SDK has no platform-settings operation. A backend-admin
 * page must not present configuration or maintenance controls without a
 * server-authoritative contract.
 */
export const ADMIN_SETTINGS_CONTRACT_UNAVAILABLE =
  'Platform settings are unavailable because the required backend API contract has not been published.';

export const adminSettingsCapability = {
  available: false as const,
  reason: ADMIN_SETTINGS_CONTRACT_UNAVAILABLE,
};

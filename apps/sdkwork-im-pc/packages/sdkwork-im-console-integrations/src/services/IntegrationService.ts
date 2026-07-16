/**
 * The generated app SDK does not currently expose an integration catalogue or
 * integration-management API. Do not manufacture client-side records or
 * mutation controls before that authority exists.
 */
export const CONSOLE_INTEGRATION_CONTRACT_UNAVAILABLE =
  'Console integrations are unavailable because the required app API contract has not been published.';

export const consoleIntegrationCapability = {
  available: false as const,
  reason: CONSOLE_INTEGRATION_CONTRACT_UNAVAILABLE,
};

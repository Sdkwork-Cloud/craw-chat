/**
 * The console-settings API has not been published in the generated app SDK.
 * Keep this capability explicit so a route cannot appear to read or modify
 * settings that the client has no authoritative contract for.
 */
export const CONSOLE_SETTINGS_CONTRACT_UNAVAILABLE =
  'Console settings are unavailable because the required app API contract has not been published.';

export const consoleSettingsCapability = {
  available: false as const,
  reason: CONSOLE_SETTINGS_CONTRACT_UNAVAILABLE,
};

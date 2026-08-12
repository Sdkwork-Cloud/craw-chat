import { UserCapabilityUnavailableError } from "./UserCapabilityUnavailableError";

export interface AppSettings {
  teenMode: boolean;
  elderlyMode: boolean;
  darkMode: boolean;
  landscape: boolean;
  fontSize: number;
  autoDownload: boolean;
  savePhoto: boolean;
  saveVideo: boolean;
  voiceLock: boolean;
}

const INITIAL_SETTINGS: AppSettings = {
  teenMode: false,
  elderlyMode: false,
  darkMode: false,
  landscape: false,
  fontSize: 2,
  autoDownload: true,
  savePhoto: true,
  saveVideo: true,
  voiceLock: false,
};

export let CURRENT_SETTINGS: AppSettings = { ...INITIAL_SETTINGS };

/**
 * User settings — fail-closed (PRD): settings persistence has no composed
 * owner SDK surface in the current H5 release, and settings pages must not
 * own browser business state. Every access throws a typed
 * `UserCapabilityUnavailableError` so the settings surface renders a typed
 * unavailable state instead of fabricating persisted preferences.
 */
export const SettingsService = {
  async getSettings(): Promise<AppSettings> {
    throw new UserCapabilityUnavailableError("User settings");
  },

  async updateSettings(_updates: Partial<AppSettings>): Promise<AppSettings> {
    throw new UserCapabilityUnavailableError("User settings update");
  },
};

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

export const SettingsService = {
  async getSettings(): Promise<AppSettings> {
    throw new UserCapabilityUnavailableError("User settings");
  },

  async updateSettings(_updates: Partial<AppSettings>): Promise<AppSettings> {
    throw new UserCapabilityUnavailableError("User settings");
  },
};

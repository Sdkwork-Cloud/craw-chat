export interface SysSettings {
  orgName: string;
  envFlag: string;
  customDomain: string;
  logoText: string;
  publishableKey: string;
  secretKey: string;
}

const CONSOLE_SETTINGS_CONTRACT_UNAVAILABLE = 'console settings contract is not available';

class SysSettingsService {
  async getSettings(): Promise<SysSettings> {
    throw new Error(CONSOLE_SETTINGS_CONTRACT_UNAVAILABLE);
  }

  async updateSettings(_updates: Partial<SysSettings>): Promise<SysSettings> {
    throw new Error(CONSOLE_SETTINGS_CONTRACT_UNAVAILABLE);
  }
}

export const sysSettingsService = new SysSettingsService();

export interface AdminSettingsData {
  platformName: string;
  supportContact: string;
  allowSelfService: boolean;
}

const BACKEND_SETTINGS_CONTRACT_UNAVAILABLE = 'backend settings contract is not available';

class AdminSettingsService {
  async getSettings(): Promise<AdminSettingsData> {
    throw new Error(BACKEND_SETTINGS_CONTRACT_UNAVAILABLE);
  }

  async updateSettings(_updates: Partial<AdminSettingsData>): Promise<void> {
    throw new Error(BACKEND_SETTINGS_CONTRACT_UNAVAILABLE);
  }
}

export const adminSettingsService = new AdminSettingsService();

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

const STORAGE_KEY = "clawchat_app_settings";

export let CURRENT_SETTINGS: AppSettings = { ...INITIAL_SETTINGS };

const loadSettings = () => {
  try {
    const data = localStorage.getItem(STORAGE_KEY);
    if (data) {
      CURRENT_SETTINGS = JSON.parse(data);
    } else {
      CURRENT_SETTINGS = { ...INITIAL_SETTINGS };
    }
  } catch (e) {
    CURRENT_SETTINGS = { ...INITIAL_SETTINGS };
  }
  return CURRENT_SETTINGS;
};

const saveSettings = () => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(CURRENT_SETTINGS));
  } catch (e) {
    console.error("Failed to save settings", e);
  }
};

loadSettings();

export const SettingsService = {
  async getSettings(): Promise<AppSettings> {
    return { ...loadSettings() };
  },

  async updateSettings(updates: Partial<AppSettings>): Promise<AppSettings> {
    loadSettings();
    const newSettings = { ...CURRENT_SETTINGS, ...updates };
    CURRENT_SETTINGS = newSettings;
    saveSettings();
    return { ...newSettings };
  },
};

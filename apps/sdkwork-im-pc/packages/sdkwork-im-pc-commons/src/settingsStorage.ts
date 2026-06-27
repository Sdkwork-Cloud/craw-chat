export const SDKWORK_IM_PC_SETTINGS_STORAGE_KEY = 'im-settings';

export const SDKWORK_IM_PC_LANGUAGE_CHANGED_EVENT = 'sdkwork-im-pc:language-changed';

export function readPersistedSettingsRecord(): Record<string, unknown> | undefined {
  if (typeof localStorage === 'undefined') {
    return undefined;
  }

  const rawValue = localStorage.getItem(SDKWORK_IM_PC_SETTINGS_STORAGE_KEY);
  if (!rawValue) {
    return undefined;
  }

  try {
    const parsed = JSON.parse(rawValue) as Record<string, unknown>;
    return parsed;
  } catch {
    localStorage.removeItem(SDKWORK_IM_PC_SETTINGS_STORAGE_KEY);
    return undefined;
  }
}

export function resolvePersistedLanguage<T extends string>(
  supportedLanguages: readonly T[],
  fallback: T,
): T {
  const language = readPersistedSettingsRecord()?.lang;
  return supportedLanguages.includes(language as T) ? language as T : fallback;
}

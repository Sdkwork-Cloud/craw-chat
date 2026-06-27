import { readPersistedSettingsRecord } from './settingsStorage';

export type HostAppearanceTheme = 'system' | 'dark' | 'light';
export type ResolvedHostAppearanceMode = 'dark' | 'light';

export const SDKWORK_IM_PC_SETTINGS_CHANGED_EVENT = 'sdkwork-im-pc:settings-changed';

function resolveHostAppearanceMode(theme: HostAppearanceTheme): ResolvedHostAppearanceMode {
  if (theme === 'light') {
    return 'light';
  }
  if (theme === 'dark') {
    return 'dark';
  }
  if (typeof window === 'undefined') {
    return 'dark';
  }
  return window.matchMedia('(prefers-color-scheme: light)').matches ? 'light' : 'dark';
}

export function readPersistedHostAppearanceTheme(): HostAppearanceTheme {
  const theme = readPersistedSettingsRecord()?.theme;
  if (theme === 'light' || theme === 'dark' || theme === 'system') {
    return theme;
  }
  return 'system';
}

export function applyHostAppearanceTheme(theme: HostAppearanceTheme): ResolvedHostAppearanceMode {
  if (typeof document === 'undefined') {
    return resolveHostAppearanceMode(theme);
  }

  const mode = resolveHostAppearanceMode(theme);
  const root = document.documentElement;

  root.classList.toggle('light-mode', mode === 'light');
  root.classList.toggle('dark', mode === 'dark');
  root.classList.remove('light');
  root.style.colorScheme = mode;

  return mode;
}

export function bootstrapHostAppearanceBridge(): () => void {
  applyHostAppearanceTheme(readPersistedHostAppearanceTheme());

  if (typeof window === 'undefined') {
    return () => undefined;
  }

  const handleSettingsChanged = (event: Event) => {
    const theme = (event as CustomEvent<{ settings?: { theme?: HostAppearanceTheme } }>).detail
      ?.settings?.theme;
    applyHostAppearanceTheme(theme ?? readPersistedHostAppearanceTheme());
  };

  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  const handleSystemThemeChanged = () => {
    if (readPersistedHostAppearanceTheme() === 'system') {
      applyHostAppearanceTheme('system');
    }
  };

  window.addEventListener(SDKWORK_IM_PC_SETTINGS_CHANGED_EVENT, handleSettingsChanged);
  mediaQuery.addEventListener('change', handleSystemThemeChanged);

  return () => {
    window.removeEventListener(SDKWORK_IM_PC_SETTINGS_CHANGED_EVENT, handleSettingsChanged);
    mediaQuery.removeEventListener('change', handleSystemThemeChanged);
  };
}

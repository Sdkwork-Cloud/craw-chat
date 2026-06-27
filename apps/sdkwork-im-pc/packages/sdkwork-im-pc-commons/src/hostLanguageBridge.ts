import {
  resolvePersistedLanguage,
  SDKWORK_IM_PC_LANGUAGE_CHANGED_EVENT,
} from './settingsStorage';

export const IM_PC_SUPPORTED_LANGUAGES = ['zh-CN', 'en-US'] as const;
export type ImPcLanguage = (typeof IM_PC_SUPPORTED_LANGUAGES)[number];

export function normalizeImPcLanguage(
  value: unknown,
  fallback: ImPcLanguage = 'zh-CN',
): ImPcLanguage {
  return IM_PC_SUPPORTED_LANGUAGES.includes(value as ImPcLanguage)
    ? (value as ImPcLanguage)
    : fallback;
}

export function resolveImPcHostLanguage(): ImPcLanguage {
  return resolvePersistedLanguage(IM_PC_SUPPORTED_LANGUAGES, 'zh-CN');
}

export function mapImPcLanguageToZhEn(language: string): 'zh' | 'en' {
  const normalized = normalizeImPcLanguage(language);
  return normalized === 'en-US' ? 'en' : 'zh';
}

export function mapZhEnToImPcLanguage(language: 'zh' | 'en'): ImPcLanguage {
  return language === 'en' ? 'en-US' : 'zh-CN';
}

export interface HostLanguageBridge {
  resolveInitialLanguage(): string;
  onLanguageChange(listener: (language: string) => void): () => void;
}

export function createImPcHostLanguageBridge(): HostLanguageBridge {
  return {
    resolveInitialLanguage() {
      return resolveImPcHostLanguage();
    },
    onLanguageChange(listener) {
      const handler = (event: Event) => {
        const lang = (event as CustomEvent<{ lang?: string }>).detail?.lang;
        if (typeof lang === 'string') {
          listener(normalizeImPcLanguage(lang));
        }
      };
      window.addEventListener(SDKWORK_IM_PC_LANGUAGE_CHANGED_EVENT, handler);
      return () => window.removeEventListener(SDKWORK_IM_PC_LANGUAGE_CHANGED_EVENT, handler);
    },
  };
}

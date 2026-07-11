import { createInstance } from 'i18next';
import { initReactI18next } from 'react-i18next';
import {
  IM_PC_SUPPORTED_LANGUAGES,
  SDKWORK_IM_PC_LANGUAGE_CHANGED_EVENT,
  normalizeImPcLanguage,
  resolveImPcHostLanguage,
} from '@sdkwork/im-pc-commons';
import zhCN from './zh-CN/communication/im-pc-workspace/home.json';
import enUS from './en-US/communication/im-pc-workspace/home.json';

type SupportedLanguage = (typeof IM_PC_SUPPORTED_LANGUAGES)[number];

function normalizeLanguage(value: unknown): SupportedLanguage {
  return normalizeImPcLanguage(value);
}

export function resolveInitialLanguage(): SupportedLanguage {
  return resolveImPcHostLanguage();
}

const i18n = createInstance();

i18n
  .use(initReactI18next)
  .init({
    resources: {
      'zh-CN': { translation: zhCN },
      'en-US': { translation: enUS }
    },
    lng: resolveInitialLanguage(),
    fallbackLng: 'zh-CN',
    supportedLngs: [...IM_PC_SUPPORTED_LANGUAGES],
    load: 'currentOnly',
    returnEmptyString: false,
    interpolation: {
      escapeValue: false
    }
  });

if (typeof window !== 'undefined') {
  window.addEventListener(SDKWORK_IM_PC_LANGUAGE_CHANGED_EVENT, (event) => {
    const nextLanguage = normalizeLanguage((event as CustomEvent<{ lang?: string }>).detail?.lang);
    if (i18n.language !== nextLanguage) {
      void i18n.changeLanguage(nextLanguage);
    }
  });
}

export default i18n;

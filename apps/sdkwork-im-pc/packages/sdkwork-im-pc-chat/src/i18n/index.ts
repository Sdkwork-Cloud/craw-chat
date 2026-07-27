import { createInstance } from 'i18next';
import { initReactI18next } from 'react-i18next';
import {
  SDKWORK_IM_PC_LANGUAGE_CHANGED_EVENT,
  resolvePersistedLanguage,
} from '@sdkwork/im-pc-commons';
import { sdkworkSubscriptionCheckoutResources } from '@sdkwork/membership-pc-subscription/catalog';
import { imTokenPlanI18nResources } from '@sdkwork/im-pc-token-plan/i18n';
import enUSAgent from './en-US/communication/im-pc-chat/agent.json';
import enUSChat from './en-US/communication/im-pc-chat/chat.json';
import enUSContacts from './en-US/communication/im-pc-chat/contacts.json';
import enUSFavorites from './en-US/communication/im-pc-chat/favorites.json';
import enUSProfile from './en-US/communication/im-pc-chat/profile.json';
import enUSScanQr from './en-US/communication/im-pc-chat/scan-qr.json';
import enUSSettingsModal from './en-US/communication/im-pc-chat/settings-modal.json';
import enUSSidebar from './en-US/communication/im-pc-chat/sidebar.json';
import enUSTokenPlan from './en-US/communication/im-pc-chat/token-plan.json';
import zhCNAgent from './zh-CN/communication/im-pc-chat/agent.json';
import zhCNChat from './zh-CN/communication/im-pc-chat/chat.json';
import zhCNContacts from './zh-CN/communication/im-pc-chat/contacts.json';
import zhCNFavorites from './zh-CN/communication/im-pc-chat/favorites.json';
import zhCNProfile from './zh-CN/communication/im-pc-chat/profile.json';
import zhCNScanQr from './zh-CN/communication/im-pc-chat/scan-qr.json';
import zhCNSettingsModal from './zh-CN/communication/im-pc-chat/settings-modal.json';
import zhCNSidebar from './zh-CN/communication/im-pc-chat/sidebar.json';
import zhCNTokenPlan from './zh-CN/communication/im-pc-chat/token-plan.json';

const zhCN = {
  ...sdkworkSubscriptionCheckoutResources['zh-CN'],
  ...imTokenPlanI18nResources['zh-CN'],
  ...zhCNSidebar,
  ...zhCNTokenPlan,
  ...zhCNAgent,
  ...zhCNProfile,
  ...zhCNContacts,
  ...zhCNFavorites,
  ...zhCNSettingsModal,
  ...zhCNChat,
  ...zhCNScanQr,
};

const enUS = {
  ...sdkworkSubscriptionCheckoutResources['en-US'],
  ...imTokenPlanI18nResources['en-US'],
  ...enUSSidebar,
  ...enUSTokenPlan,
  ...enUSAgent,
  ...enUSProfile,
  ...enUSContacts,
  ...enUSFavorites,
  ...enUSSettingsModal,
  ...enUSChat,
  ...enUSScanQr,
};

const SUPPORTED_LANGUAGES = ['zh-CN', 'en-US'] as const;
type SupportedLanguage = typeof SUPPORTED_LANGUAGES[number];

function normalizeLanguage(value: unknown): SupportedLanguage {
  return SUPPORTED_LANGUAGES.includes(value as SupportedLanguage)
    ? value as SupportedLanguage
    : 'zh-CN';
}

export function resolveInitialLanguage(): SupportedLanguage {
  return resolvePersistedLanguage(SUPPORTED_LANGUAGES, 'zh-CN');
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

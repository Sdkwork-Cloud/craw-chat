import { createInstance } from 'i18next';
import { initReactI18next } from 'react-i18next';
import {
  SDKWORK_IM_PC_LANGUAGE_CHANGED_EVENT,
  resolvePersistedLanguage,
} from '@sdkwork/im-pc-commons';

import zhCNCommon from './locales/zh-CN/common.json';
import zhCNShop from './locales/zh-CN/shop.json';
import zhCNProduct from './locales/zh-CN/product.json';
import zhCNCart from './locales/zh-CN/cart.json';
import zhCNCheckout from './locales/zh-CN/checkout.json';

import enUSCommon from './locales/en-US/common.json';
import enUSShop from './locales/en-US/shop.json';
import enUSProduct from './locales/en-US/product.json';
import enUSCart from './locales/en-US/cart.json';
import enUSCheckout from './locales/en-US/checkout.json';

const SUPPORTED_LANGUAGES = ['zh-CN', 'en-US'] as const;
type SupportedLanguage = typeof SUPPORTED_LANGUAGES[number];

function normalizeLanguage(value: unknown): SupportedLanguage {
  return SUPPORTED_LANGUAGES.includes(value as SupportedLanguage)
    ? value as SupportedLanguage
    : 'zh-CN';
}

function resolveInitialLanguage(): SupportedLanguage {
  return resolvePersistedLanguage(SUPPORTED_LANGUAGES, 'zh-CN');
}

const resources = {
  'zh-CN': {
    common: zhCNCommon,
    shop: zhCNShop,
    product: zhCNProduct,
    cart: zhCNCart,
    checkout: zhCNCheckout,
  },
  'en-US': {
    common: enUSCommon,
    shop: enUSShop,
    product: enUSProduct,
    cart: enUSCart,
    checkout: enUSCheckout,
  },
};

const i18n = createInstance();

i18n
  .use(initReactI18next)
  .init({
    resources,
    lng: resolveInitialLanguage(),
    fallbackLng: 'zh-CN',
    ns: ['common', 'shop', 'product', 'cart', 'checkout'],
    defaultNS: 'common',
    interpolation: {
      escapeValue: false,
    },
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

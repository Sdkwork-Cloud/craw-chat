import {
  createContext,
  useCallback,
  useContext,
  useMemo,
  type ReactNode,
} from "react";

import enUSAppNotFound from "./en-US/communication/im-h5-commons/app-not-found.json";
import enUSChatConversation from "./en-US/communication/im-h5-commons/chat-conversation.json";
import enUSChatInbox from "./en-US/communication/im-h5-commons/chat-inbox.json";
import zhCNAppNotFound from "./zh-CN/communication/im-h5-commons/app-not-found.json";
import zhCNChatConversation from "./zh-CN/communication/im-h5-commons/chat-conversation.json";
import zhCNChatInbox from "./zh-CN/communication/im-h5-commons/chat-inbox.json";

type LocaleId = "en-US" | "zh-CN";
const enUS = {
  ...enUSChatInbox,
  ...enUSChatConversation,
  ...enUSAppNotFound,
};
const zhCN = {
  ...zhCNChatInbox,
  ...zhCNChatConversation,
  ...zhCNAppNotFound,
};
type MessageKey = keyof typeof enUS;
type MessageValues = Record<string, string | number>;

const LOCALE_MESSAGES: Record<LocaleId, Record<string, string>> = {
  "en-US": enUS,
  "zh-CN": zhCN,
};

const I18nContext = createContext<{
  locale: LocaleId;
  t: (key: MessageKey, values?: MessageValues) => string;
}>({
  locale: "en-US",
  t: (key) => key,
});

function resolveInitialLocale(): LocaleId {
  if (typeof navigator === "undefined") {
    return "en-US";
  }
  return navigator.language.toLowerCase().startsWith("zh") ? "zh-CN" : "en-US";
}

function formatMessage(template: string, values?: MessageValues): string {
  if (!values) {
    return template;
  }
  return template.replace(/\{\{(\w+)\}\}/gu, (_match, name: string) =>
    String(values[name] ?? ""),
  );
}

export function I18nProvider({ children }: { children: ReactNode }) {
  const locale = resolveInitialLocale();
  const t = useCallback(
    (key: MessageKey, values?: MessageValues) => {
      const template =
        LOCALE_MESSAGES[locale][key] ?? LOCALE_MESSAGES["en-US"][key] ?? key;
      return formatMessage(template, values);
    },
    [locale],
  );
  const value = useMemo(() => ({ locale, t }), [locale, t]);
  return <I18nContext.Provider value={value}>{children}</I18nContext.Provider>;
}

export function useI18n() {
  return useContext(I18nContext);
}

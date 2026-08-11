import React, { useState, useEffect, useCallback } from "react";
import { PageLayout, Group, ListItem } from "../../components/SettingsCommons";
import { ShieldAlert } from "lucide-react";
import {
  Avatar,
  showToast,
  showPrompt,
} from "@sdkwork/im-h5-commons";
import { ContactService, type Contact } from "@sdkwork/im-h5-contacts";
import { useNavigate } from "react-router";
import { useTranslation } from 'react-i18next';

export const Blacklist = () => {
  const { t } = useTranslation();
  const [blacklist, setBlacklist] = useState<Contact[]>([]);
  const [loading, setLoading] = useState(true);
  const [loadError, setLoadError] = useState(false);

  const load = useCallback(async () => {
    setLoading(true);
    try {
      const blocked: Contact[] = [];
      let cursor: string | undefined;
      do {
        const page = await ContactService.listContactPage(cursor);
        blocked.push(...page.items.filter((contact) => contact.isBlocked === true));
        cursor = page.hasMore ? page.nextCursor : undefined;
      } while (cursor);
      setBlacklist(blocked);
      setLoadError(false);
    } catch (error) {
      console.error("Unable to load blacklist", error);
      setLoadError(true);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    void load();
  }, [load]);

  const handleRemove = async (user: Contact) => {
    try {
      await ContactService.updateContactPreferences(user.id, { isBlocked: false });
      setBlacklist(blacklist.filter((item) => item.id !== user.id));
      showToast(t("user:system.removed_from_blacklist", "Removed from blacklist"));
    } catch (error) {
      console.error("Unable to remove from blacklist", error);
      showToast(t("user:system.remove_blacklist_failed", "Failed to remove from blacklist, please try again later"));
    }
  };

  return (
    <PageLayout title={t("user:system.blacklist", "Blacklist")}>
      <div className="flex flex-col h-full bg-bg-color">
        <div className="p-4 flex items-center gap-3 bg-red-50 dark:bg-red-900/10 border-b border-red-100 dark:border-red-900/20">
          <ShieldAlert className="w-5 h-5 text-red-500 shrink-0" />
          <p className="text-[13px] text-red-600 dark:text-red-400">{t("user:system.blacklist_desc", "You will not receive messages from contacts on this list, and they cannot view your Moments.")}</p>
        </div>

        <div className="flex-1 overflow-y-auto">
          {blacklist.map((user) => (
            <div
              key={user.id}
              className="flex items-center justify-between p-4 bg-chat-other-bg border-b border-border-color"
            >
              <div className="flex items-center gap-3">
                <Avatar src={user.avatar} className="w-10 h-10 rounded-lg" />
                <span className="text-[16px] text-text-main font-medium">
                  {user.name}
                </span>
              </div>
              <button
                className="px-3 py-1.5 rounded-full border border-border-color text-text-sub text-[13px] active:bg-active-bg transition-colors"
                onClick={() => void handleRemove(user)}
              >移除</button>
            </div>
          ))}
          {!loading && !loadError && blacklist.length === 0 && (
            <div className="p-10 text-center text-[14px] text-text-sub">
              {t("user:system.blacklist_empty", "No blacklisted members")}
            </div>
          )}
          {loading && (
            <div className="p-10 text-center text-[14px] text-text-sub">
              {t("common.loading", "Loading...")}
            </div>
          )}
          {!loading && loadError && (
            <button
              type="button"
              className="w-full p-10 text-center text-[14px] text-primary-blue"
              onClick={() => void load()}
            >
              {t("user:system.blacklist_load_failed", "Failed to load, tap to retry")}
            </button>
          )}
        </div>
      </div>
    </PageLayout>
  );
};

export const FAQ = () => {
  const { t } = useTranslation();
  
const [active, setActive] = useState<string | null>(null);

  const toggle = (id: string) => setActive(active === id ? null : id);
  return (
    <PageLayout title={t("user:system.faq_title", "FAQ")}>
      <Group>
        <ListItem label={t("user:system.q1", "How do I recover my password?")} onClick={() => toggle("q1")} />
        {active === "q1" && (
          <div className="px-4 py-3 text-[14px] text-text-sub bg-chat-other-bg">{t("user:system.a1", "Tap \u201CForgot password\u201D on the login page and reset it with a phone verification code.")}</div>
        )}

        <ListItem label={t("user:system.q2", "How do I unfreeze my account?")} onClick={() => toggle("q2")} />
        {active === "q2" && (
          <div className="px-4 py-3 text-[14px] text-text-sub bg-chat-other-bg">{t("user:system.a2", "Go to the Security Center and file an appeal with real-name verification and friend verification.")}</div>
        )}

        <ListItem
          label={t("user:system.q3", "How do I change my WeChat ID?")}
          hideBorder
          onClick={() => toggle("q3")}
        />
        {active === "q3" && (
          <div className="px-4 py-3 text-[14px] text-text-sub bg-chat-other-bg">{t("user:system.a3", "Your WeChat ID can only be changed once a year; tap \u201CWeChat ID\u201D on the personal info page to change it.")}</div>
        )}
      </Group>
    </PageLayout>
  );
};

export const Feedback = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  return (
    <PageLayout title={t("user:help_about.feedback", "Feedback")}>
      <div className="p-4">
        <textarea
          className="w-full h-40 bg-chat-other-bg p-4 rounded-xl text-text-main outline-none resize-none"
          placeholder={t("user:system.feedback_placeholder", "Describe the issue or suggestion you encountered...")}
        ></textarea>
        <button
          className="mt-6 w-full h-12 bg-accent-green text-white rounded-lg font-medium active:opacity-80 transition-opacity"
          onClick={() => showToast(t("commons.feature_unavailable", "This feature is not available yet while the real service is being integrated."))}
        >提交</button>
      </div>
    </PageLayout>
  );
};

export const Features = () => {
  const { t } = useTranslation();
  
return (
  <PageLayout title={t("user:system.features_title", "Features")}>
    <div className="p-6 text-center text-text-sub">
      <h3 className="text-[18px] font-bold text-text-main mb-4">{t("user:system.changelog", "Sdkwork IM H5 1.0.0 changelog")}</h3>
      <p className="text-[14px] leading-relaxed">{t("user:system.f1", "1. Brand-new UI design")}<br />{t("user:system.f2", "2. Agent chat support")}<br />{t("user:system.f3", "3. Improved performance and experience")}</p>
    </div>
  </PageLayout>
);
};

export const Complain = () => {
  const { t } = useTranslation();
  
const handleComplain = async (type: string) => {
    const reason = await showPrompt(
      `${t("user:system.complain_submitting", "Complaining about [{{type}}]. You can add more info:", { type })}`,
      "",
    );
    if (reason !== null) {
      showToast(t("commons.feature_unavailable", "This feature is not available yet while the real service is being integrated."));
    }
  };

  return (
    <PageLayout title={t("user:system.complain_title", "Report")}>
      <Group>
        <ListItem label={t("user:system.fraud", "Fraud")} onClick={() =>handleComplain(t("user:system.fraud", "Fraud"))} /><ListItem label={t("user:system.porn_violence", "Pornography & violence")} onClick={() =>handleComplain(t("user:system.porn_violence", "Pornography & violence"))} /><ListItem
          label={t("user:system.rumor", "Political rumors")}
          hideBorder
          onClick={() => handleComplain(t("user:system.rumor", "Political rumors"))}
        />
      </Group>
    </PageLayout>
  );
};

export const TOS = () => {
  const { t } = useTranslation();
  
return (
  <PageLayout title={t("user:system.terms_title", "Software License and Service Agreement")}>
    <div className="p-6 text-text-sub text-[14px] leading-relaxed">{t("user:system.welcome", "Welcome to Sdkwork IM H5!")}<br />
      <br />{t("user:system.terms_desc", "Before using this software, please read and fully understand this agreement...")}</div>
  </PageLayout>
);
};

export const PrivacyPolicy = () => {
  const { t } = useTranslation();
  
return (
  <PageLayout title={t("user:system.privacy_title", "Privacy Protection Guidelines")}>
    <div className="p-6 text-text-sub text-[14px] leading-relaxed">{t("user:system.privacy_welcome", "We take your privacy seriously.")}<br />
      <br />{t("user:system.privacy_desc", "These guidelines explain how we collect, use, store and share your personal information...")}</div>
  </PageLayout>
);
};

export const ManageChatHistory = () => {
  const { t } = useTranslation();
  
return (
  <PageLayout title={t("user:system.storage", "Manage chat history")}>
    <Group>
      <ListItem label={t("settings:system.user_1", "Zhang San")} rightText="450 MB" />
      <ListItem label={t("settings:system.user_2", "Li Si")} rightText="120 MB" />
      <ListItem label={t("user:system.work_group", "Work group")} rightText="890 MB" hideBorder />
    </Group>
  </PageLayout>
);
};

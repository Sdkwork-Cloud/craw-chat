import React, { useState } from "react";
import { PageLayout, Group, ListItem } from "../../components/SettingsCommons";
import { ShieldAlert } from "lucide-react";
import {
  Avatar,
  showToast,
  showPrompt,
} from "@sdkwork/im-h5-commons";
import { useNavigate } from "react-router";
import { useTranslation } from 'react-i18next';

export const Blacklist = () => {
  const { t } = useTranslation();
const [blacklist, setBlacklist] = useState([
    {
      id: 1,
      name: t("settings:system.blocked_user_1", "微商推广张三"),
      avatar: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/avatars/b1/100.png",
    },
    {
      id: 2,
      name: t("settings:system.blocked_user_2", "贷款专员李四"),
      avatar: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/avatars/b2/100.png",
    },
  ]);

  return (
    <PageLayout title={t("user:system.blacklist", "通讯录黑名单")}>
      <div className="flex flex-col h-full bg-bg-color">
        <div className="p-4 flex items-center gap-3 bg-red-50 dark:bg-red-900/10 border-b border-red-100 dark:border-red-900/20">
          <ShieldAlert className="w-5 h-5 text-red-500 shrink-0" />
          <p className="text-[13px] text-red-600 dark:text-red-400">{t("user:system.blacklist_desc", "你将不会收到列表中联系人的消息，并且他们无法查看你的朋友圈。")}</p>
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
                onClick={async () => {
                  showToast(t("user:system.removed_from_blacklist", "已移出黑名单"));
                  setBlacklist(blacklist.filter((u) => u.id !== user.id));
                }}
              >移除</button>
            </div>
          ))}
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
    <PageLayout title={t("user:system.faq_title", "常见问题")}>
      <Group>
        <ListItem label={t("user:system.q1", "如何找回密码？")} onClick={() => toggle("q1")} />
        {active === "q1" && (
          <div className="px-4 py-3 text-[14px] text-text-sub bg-chat-other-bg">{t("user:system.a1", "您可以在登录页面点击“找回密码”并通过手机验证码重置密码。")}</div>
        )}

        <ListItem label={t("user:system.q2", "如何解冻账号？")} onClick={() => toggle("q2")} />
        {active === "q2" && (
          <div className="px-4 py-3 text-[14px] text-text-sub bg-chat-other-bg">{t("user:system.a2", "请前往安全中心进行申诉解冻，需要提供实名认证和好友辅助验证。")}</div>
        )}

        <ListItem
          label={t("user:system.q3", "如何修改微信号？")}
          hideBorder
          onClick={() => toggle("q3")}
        />
        {active === "q3" && (
          <div className="px-4 py-3 text-[14px] text-text-sub bg-chat-other-bg">{t("user:system.a3", "微信号一年只能修改一次，您可以在“个人信息”页点击“微信号”进行修改。")}</div>
        )}
      </Group>
    </PageLayout>
  );
};

export const Feedback = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  return (
    <PageLayout title={t("user:help_about.feedback", "意见反馈")}>
      <div className="p-4">
        <textarea
          className="w-full h-40 bg-chat-other-bg p-4 rounded-xl text-text-main outline-none resize-none"
          placeholder={t("user:system.feedback_placeholder", "请详细描述您遇到的问题或建议...")}
        ></textarea>
        <button
          className="mt-6 w-full h-12 bg-[#00B42A] text-white rounded-lg font-medium active:opacity-80 transition-opacity"
          onClick={async () => {
            showToast(t("user:system.submit_success", "提交成功，感谢反馈！"));
            navigate(-1);
          }}
        >提交</button>
      </div>
    </PageLayout>
  );
};

export const Features = () => {
  const { t } = useTranslation();
  
return (
  <PageLayout title={t("user:system.features_title", "功能介绍")}>
    <div className="p-6 text-center text-text-sub">
      <h3 className="text-[18px] font-bold text-text-main mb-4">{t("user:system.changelog", "Sdkwork IM H5 1.0.0 更新日志")}</h3>
      <p className="text-[14px] leading-relaxed">{t("user:system.f1", "1. 全新的 UI 设计")}<br />{t("user:system.f2", "2. 支持智能体聊天")}<br />{t("user:system.f3", "3. 优化了性能和体验")}</p>
    </div>
  </PageLayout>
);
};

export const Complain = () => {
  const { t } = useTranslation();
  
const handleComplain = async (type: string) => {
    const reason = await showPrompt(
      `${t("user:system.complain_submitting", "正在投诉 [{{type}}]。您可以补充更多信息：", { type })}`,
      "",
    );
    if (reason !== null) {
      showToast(t("user:system.complain_success", "投诉已提交受理"));
    }
  };

  return (
    <PageLayout title={t("user:system.complain_title", "投诉")}>
      <Group>
        <ListItem label={t("user:system.fraud", "欺诈骗钱")} onClick={() =>handleComplain(t("user:system.fraud", "欺诈骗钱"))} /><ListItem label={t("user:system.porn_violence", "色情暴力")} onClick={() =>handleComplain(t("user:system.porn_violence", "色情暴力"))} /><ListItem
          label={t("user:system.rumor", "政治谣言")}
          hideBorder
          onClick={() => handleComplain(t("user:system.rumor", "政治谣言"))}
        />
      </Group>
    </PageLayout>
  );
};

export const TOS = () => {
  const { t } = useTranslation();
  
return (
  <PageLayout title={t("user:system.terms_title", "软件许可及服务协议")}>
    <div className="p-6 text-text-sub text-[14px] leading-relaxed">{t("user:system.welcome", "欢迎使用 Sdkwork IM H5！")}<br />
      <br />{t("user:system.terms_desc", "在使用本软件前，请您务必仔细阅读并透彻理解本协议...")}</div>
  </PageLayout>
);
};

export const PrivacyPolicy = () => {
  const { t } = useTranslation();
  
return (
  <PageLayout title={t("user:system.privacy_title", "隐私保护指引")}>
    <div className="p-6 text-text-sub text-[14px] leading-relaxed">{t("user:system.privacy_welcome", "我们非常重视您的隐私保护。")}<br />
      <br />{t("user:system.privacy_desc", "本指引将向您说明我们如何收集、使用、存储和共享您的个人信息...")}</div>
  </PageLayout>
);
};

export const ManageChatHistory = () => {
  const { t } = useTranslation();
  
return (
  <PageLayout title={t("user:system.storage", "管理聊天记录")}>
    <Group>
      <ListItem label={t("settings:system.user_1", "张三")} rightText="450 MB" />
      <ListItem label={t("settings:system.user_2", "李四")} rightText="120 MB" />
      <ListItem label={t("user:system.work_group", "工作群")} rightText="890 MB" hideBorder />
    </Group>
  </PageLayout>
);
};

import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { ChevronLeft, ChevronRight, QrCode } from "lucide-react";
import { Avatar, IconButton } from "@sdkwork/im-h5-commons";
import { ProfileService, type UserProfile } from "../services/ProfileService";
import { ProfileListItem } from "../components/ProfileListItem";

export const MyProfile: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const [profile, setProfile] = useState<UserProfile | null>(null);

  useEffect(() => {
    ProfileService.getUserProfile().then(setProfile);
  }, []);

  return (
    <div className="flex flex-col h-full bg-bg-color overflow-y-auto">
      {/* Header */}
      <header className="h-[56px] flex items-center justify-between px-1 glass-header sticky top-0 z-10 shrink-0 pt-safe">
        <div className="flex items-center z-10 flex-1">
          <IconButton
            icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
          <h2 className="text-[17px] font-medium text-text-main">{t('user.auto_24b99e7e', '个人信息')}</h2>
        </div>
        <div className="flex-1" />
      </header>

      <div className="flex flex-col pb-8 mt-2">
        {/* Group 1 */}
        <div className="mb-2 border-y border-border-color flex flex-col">
          <div
            onClick={() => navigate("/my-profile/avatar")}
            className="flex items-center justify-between px-4 py-3 bg-chat-other-bg active:bg-active-bg transition-colors cursor-pointer border-b border-border-color"
          >
            <span className="text-[16px] text-text-main">{t('user.auto_b1e1b', '头像')}</span>
            <div className="flex items-center gap-2 text-text-sub">
              <Avatar
                src={profile?.avatar || "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/me/200x200.png"}
                size="md"
                className="w-14 h-14 rounded-xl"
              />
              <ChevronRight className="w-5 h-5 opacity-50" />
            </div>
          </div>
          <ProfileListItem
            label={t('user.auto_prop_a88ea', '名字')}
            rightText={profile?.name || "User"}
            onClick={() => navigate("/my-profile/name")}
          />
          <ProfileListItem
            label={t('user.auto_prop_17cb85a', '拍一拍')}
            onClick={() => navigate("/my-profile/tickle")}
          />
          <ProfileListItem
            label={t('user.auto_prop_1712c64', '微信号')}
            rightText={profile?.wechatId || "wxid_123456789"}
          />
          <ProfileListItem
            label={t('user.auto_prop_n62fa905a', '我的二维码')}
            rightElement={<QrCode className="w-5 h-5" />}
            onClick={() => navigate("/my-profile/qrcode")}
          />
          <ProfileListItem label={t('user.auto_prop_cd0a6', '更多')} onClick={() => navigate("/my-profile/more")} />
        </div>

        {/* Group 2 */}
        <div className="mb-2 border-y border-border-color flex flex-col">
          <ProfileListItem
            label={t('user.auto_prop_30ca7afd', '来电铃声')}
            onClick={() => navigate("/my-profile/ringtone")}
          />
        </div>

        {/* Group 3 */}
        <div className="border-y border-border-color flex flex-col">
          <ProfileListItem
            label={t('user.auto_prop_17164b3', '微信豆')}
            onClick={() => navigate("/my-profile/beans")}
          />
          <ProfileListItem
            label={t('user.auto_prop_2e5be3e3', '我的地址')}
            onClick={() => navigate("/my-profile/address")}
          />
        </div>
      </div>
    </div>
  );
};

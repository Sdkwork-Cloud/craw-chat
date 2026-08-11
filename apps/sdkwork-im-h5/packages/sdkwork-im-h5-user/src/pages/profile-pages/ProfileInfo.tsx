import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { QrCode } from "lucide-react";
import {
  Avatar,
  showToast,
  showPrompt,
} from "@sdkwork/im-h5-commons";
import {
  ProfileService,
  type UserProfile,
} from "../../services/ProfileService";
import { PageLayout, Group, ListItem } from "../../components/SettingsCommons";

export const ProfileAvatar = () => {
  const { t } = useTranslation();
const [profile, setProfile] = useState<UserProfile | null>(null);

  useEffect(() => {
    ProfileService.getUserProfile().then(setProfile);
  }, []);

  return (
    <PageLayout title={t('user.auto_prop_24baafeb', 'Profile photo')}>
      <div className="flex flex-col items-center justify-center py-20">
        <Avatar
          fallback={profile?.name || "?"}
          src={profile?.avatar ?? ""}
          size="lg"
          className="w-64 h-64 rounded-xl shadow-lg"
        />
        <button
          className="mt-12 w-[200px] h-12 bg-chat-other-bg text-text-main rounded-lg font-medium active:bg-active-bg transition-colors border border-border-color"
          onClick={async () => {
            const url = await showPrompt(
              "请输入新头像的图片网址",
              profile?.avatar ?? "",
            );
            if (url) {
              ProfileService.updateUserProfile({ avatar: url });
              showToast(t('user.auto_fn_n44c8107b', 'New profile photo applied'));
              window.location.reload();
            }
          }}
        >{t('user.auto_304cf589', 'Change photo')}</button>
      </div>
    </PageLayout>
  );
};

export const ProfileName = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  const [name, setName] = useState("");

  useEffect(() => {
    ProfileService.getUserProfile().then((p) => setName(p.name));
  }, []);

  const handleSave = async () => {
    await ProfileService.updateUserProfile({ name });
    showToast(t('user.auto_fn_518ad458', 'Changes saved'));
    navigate(-1);
  };

  return (
    <PageLayout title={t('user.auto_prop_3053486f', 'Change name')}>
      <div className="px-4 py-6">
        <input
          type="text"
          value={name}
          onChange={(e) => setName(e.target.value)}
          className="w-full bg-transparent border-b-2 border-accent-green text-[18px] text-text-main pb-2 outline-none"
        />
        <p className="text-[13px] text-text-sub mt-2">{t('user.auto_n1227fe61', 'A good name makes it easier for friends to remember you.')}</p>
        <button
          onClick={handleSave}
          className="mt-8 w-full h-12 bg-accent-green text-white rounded-lg font-medium active:opacity-80 transition-opacity"
        >{t('user.auto_a071b', 'Save')}</button>
      </div>
    </PageLayout>
  );
};

export const ProfileTickle = () => {
  const { t } = useTranslation();
  
const [tickle, setTickle] = useState("");
  const navigate = useNavigate();
  return (
    <PageLayout title={t('user.auto_prop_17cb85a', 'Nudge')}>
      <div className="px-4 py-6">
        <div className="flex items-center gap-2 mb-2">
          <span className="text-[16px] text-text-main">{t('user.auto_n5f2a95e3', 'Friends nudged me')}</span>
          <input
            type="text"
            value={tickle}
            onChange={(e) => setTickle(e.target.value)}
            placeholder={t('user.auto_prop_1ccfb7b', 'on the shoulder')}
            className="flex-1 bg-chat-other-bg px-3 py-2 rounded-lg text-[16px] text-text-main outline-none border border-border-color focus:border-accent-green transition-colors"
          />
        </div>
        <p className="text-[13px] text-text-sub">{t('user.auto_68734489', 'This text shows when friends nudge you.')}</p>
        <button
          className="mt-8 w-full h-12 bg-accent-green text-white rounded-lg font-medium active:opacity-80 transition-opacity"
          onClick={async () => {
            showToast(t('user.auto_fn_518ad458', 'Changes saved'));
            navigate(-1);
          }}
        >{t('user.auto_b7804', 'Done')}</button>
      </div>
    </PageLayout>
  );
};

export const ProfileQRCode = () => {
  const { t } = useTranslation();
  
const [profile, setProfile] = useState<UserProfile | null>(null);

  useEffect(() => {
    ProfileService.getUserProfile().then(setProfile);
  }, []);

  return (
    <PageLayout title={t('user.auto_prop_n62fa905a', 'My QR code')}>
      <div className="flex flex-col items-center py-10 px-4">
        <div className="w-full max-w-[320px] bg-chat-other-bg rounded-2xl shadow-sm border border-border-color p-6">
          <div className="flex items-center gap-4 mb-6">
            <Avatar
              src={profile?.avatar || "https://picsum.photos/seed/me/200/200"}
              size="md"
              className="w-14 h-14 rounded-xl"
            />
            <div>
              <h3 className="text-[18px] font-bold text-text-main">
                {profile?.name || "User"}
              </h3>
              <p className="text-[13px] text-text-sub">{profile?.region || "北京 海淀"}</p>
            </div>
          </div>
          <div
            className="w-full aspect-square bg-white rounded-xl flex items-center justify-center p-4"
            onClick={() => showToast(t('user.auto_fn_732c5cd8', 'QR code saved to album'))}
          >
            <QrCode className="w-full h-full text-black" />
          </div>
          <p className="text-[13px] text-text-sub text-center mt-6">{t('user.auto_6512840a', 'Scan the QR code above to add me as a friend')}</p>
        </div>
      </div>
    </PageLayout>
  );
};

export const ProfileMore = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  const [profile, setProfile] = useState<UserProfile | null>(null);

  useEffect(() => {
    ProfileService.getUserProfile().then(setProfile);
  }, []);

  return (
    <PageLayout title={t('user.auto_prop_30254bd4', 'More info')}>
      <Group>
        <ListItem
          label={t('user.auto_prop_bf6e4', 'Gender')}
          rightText={profile?.gender || "未设置"}
          onClick={() => navigate("/my-profile/more/gender")}
        />
        <ListItem
          label={t('user.auto_prop_ae20a', 'Region')}
          rightText={profile?.region || "未设置"}
          onClick={() => navigate("/my-profile/more/region")}
        />
        <ListItem
          label={t('user.auto_prop_2500444c', 'Signature')}
          rightText={profile?.signature || "未填写"}
          hideBorder
          onClick={() => navigate("/my-profile/more/signature")}
        />
      </Group>
    </PageLayout>
  );
};

export const ProfileRingtone = () => {
  const { t } = useTranslation();
  
return (
  <PageLayout title={t('user.auto_prop_30ca7afd', 'Ringtone')}>
    <div className="flex flex-col items-center py-20">
      <div className="w-20 h-20 bg-primary-blue/10 rounded-full flex items-center justify-center mb-6">
        <span className="text-primary-blue text-3xl">🎵</span>
      </div>
      <h3 className="text-[18px] font-medium text-text-main mb-2">{t('user.auto_4a536159', 'Default ringtone')}</h3>
      <p className="text-[14px] text-text-sub mb-8">{t('user.auto_6f2aec0', 'Using the system default ringtone')}</p>
      <button
        className="w-[200px] h-12 bg-chat-other-bg text-text-main rounded-lg font-medium active:bg-active-bg transition-colors border border-border-color"
        onClick={async () => {
          const ringtone = await showPrompt("请输入新的铃声名称");
          if (ringtone) {
            showToast(`已应用新铃声: ${ringtone}`);
          }
        }}
      >{t('user.auto_305433fb', 'Change ringtone')}</button>
    </div>
  </PageLayout>
  );
};

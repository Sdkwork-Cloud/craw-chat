import { useTranslation } from "react-i18next";
import React from "react";
import { PageLayout } from "../../components/SettingsCommons";
import { showToast } from "@sdkwork/im-h5-commons";
import { useNavigate } from "react-router";

export const WechatID = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  return (
    <PageLayout title={t('user.auto_prop_n6a30c687', 'Change WeChat ID')}>
      <div className="p-4">
        <input
          type="text"
          defaultValue=""
          className="w-full bg-transparent border-b-2 border-accent-green text-[18px] text-text-main pb-2 outline-none"
        />
        <p className="text-[13px] text-text-sub mt-2">{t('user.auto_5d5135b0', 'Your WeChat ID is your unique account identifier and can only be changed once a year.')}</p>
        <button
          className="mt-8 w-full h-12 bg-accent-green text-white rounded-lg font-medium active:opacity-80 transition-opacity"
          onClick={() => showToast(t('commons.feature_unavailable', 'This feature is not available yet while the real service is being integrated.'))}
        >{t('user.auto_a071b', 'Save')}</button>
      </div>
    </PageLayout>
  );
};

export const ResetVoiceLock = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  return (
    <PageLayout title={t('user.auto_prop_477faf2d', 'Reset voice lock')}>
      <div className="flex flex-col items-center py-20">
        <p className="text-[24px] font-medium text-text-main mb-12">
          5 8 2 9 0 1
        </p>
        <div
          className="w-24 h-24 bg-accent-green rounded-full flex items-center justify-center active:scale-95 transition-transform cursor-pointer shadow-lg shadow-green-500/30"
          onClick={() => showToast(t('commons.feature_unavailable', 'This feature is not available yet while the real service is being integrated.'))}
        >
          <span className="text-white text-3xl">🎤</span>
        </div>
        <p className="text-[14px] text-text-sub mt-8">{t('user.auto_n3aa5d93a', 'Hold the button above and read the numbers at a steady pace')}</p>
      </div>
    </PageLayout>
  );
};

export const BindQQ = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  return (
    <PageLayout title={t('user.auto_prop_24b8024e', 'Link QQ')}>
      <div className="p-4">
        <input
          type="text"
          placeholder={t('user.auto_prop_18dd7', 'QQ number')}
          className="w-full bg-chat-other-bg p-4 rounded-xl text-text-main outline-none mb-4"
        />
        <input
          type="password"
          placeholder={t('user.auto_prop_31971b', 'QQ password')}
          className="w-full bg-chat-other-bg p-4 rounded-xl text-text-main outline-none mb-8"
        />
        <button
          className="w-full h-12 bg-accent-green text-white rounded-lg font-medium active:opacity-80 transition-opacity"
          onClick={() => showToast(t('commons.feature_unavailable', 'This feature is not available yet while the real service is being integrated.'))}
        >{t('user.auto_fb6e9', 'Link')}</button>
      </div>
    </PageLayout>
  );
};

export const BindEmail = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  return (
    <PageLayout title={t('user.auto_prop_3b0fa16c', 'Link email')}>
      <div className="p-4">
        <input
          type="email"
          placeholder={t('user.auto_prop_n35387c04', 'Enter your email address')}
          className="w-full bg-chat-other-bg p-4 rounded-xl text-text-main outline-none mb-8"
        />
        <button
          className="w-full h-12 bg-accent-green text-white rounded-lg font-medium active:opacity-80 transition-opacity"
          onClick={() => showToast(t('commons.feature_unavailable', 'This feature is not available yet while the real service is being integrated.'))}
        >{t('user.auto_3c864bcd', 'Send verification email')}</button>
      </div>
    </PageLayout>
  );
};

export const RecoverPassword = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  return (
    <PageLayout title={t('user.auto_prop_n5f360849', 'Recover account')}>
      <div className="p-6 text-center">
        <div className="w-16 h-16 bg-primary-blue/10 rounded-full flex items-center justify-center mx-auto mb-6">
          <span className="text-primary-blue text-2xl">🛡️</span>
        </div>
        <h3 className="text-[18px] font-medium text-text-main mb-4">{t('user.auto_41738847', 'Account appeal')}</h3>
        <p className="text-[14px] text-text-sub mb-8">{t('user.auto_n777a8346', 'If your phone, QQ or email are all unavailable, you can recover your password through an appeal.')}</p>
        <button
          className="w-full h-12 bg-accent-green text-white rounded-lg font-medium active:opacity-80 transition-opacity"
          onClick={() => showToast(t('commons.feature_unavailable', 'This feature is not available yet while the real service is being integrated.'))}
        >{t('user.auto_2c8f1101', 'Start appeal')}</button>
      </div>
    </PageLayout>
  );
};

export const DeleteAccount = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  return (
    <PageLayout title={t('user.auto_prop_33c24aa9', 'Delete account')}>
      <div className="p-6 text-center">
        <div className="w-16 h-16 bg-red-500/10 rounded-full flex items-center justify-center mx-auto mb-6">
          <span className="text-red-500 text-2xl">⚠️</span>
        </div>
        <h3 className="text-[18px] font-medium text-text-main mb-4">{t('user.auto_n772cfd2e', 'Delete your Sdkwork IM H5 account')}</h3>
        <p className="text-[14px] text-text-sub mb-8 text-left leading-relaxed">{t('user.auto_3c61f057', 'After deletion your account will be permanently removed and cannot be recovered.')}<br />
          <br />{t('user.auto_n268ec2ab', '1. All your chat history and contacts will be cleared.')}<br />{t('user.auto_5efafa2e', '2. Your credits and balance will be cleared.')}<br />{t('user.auto_f8e1ba', '3. Linked third-party apps will be unlinked.')}</p>
        <button
          className="w-full h-12 bg-accent-red text-white rounded-lg font-medium active:opacity-80 transition-opacity"
          onClick={() => showToast(t('commons.feature_unavailable', 'This feature is not available yet while the real service is being integrated.'))}
        >{t('user.auto_3761c93c', 'Request deletion')}</button>
      </div>
    </PageLayout>
  );
};

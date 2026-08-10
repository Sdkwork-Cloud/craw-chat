import React, { useState } from "react";
import { useTranslation } from "react-i18next";
import { PageLayout } from "../SettingsSubPages";
import { showToast } from "@sdkwork/im-h5-commons";

export const ChangePhoneNumber: React.FC = () => {
  const { t } = useTranslation();
  const [step, setStep] = useState(1);
  const [phone, setPhone] = useState("");
  const [code, setCode] = useState("");

  const handleSubmit = () => {
    if (!phone || !code) return showToast(t("user:account_sec.enter_full_info", "请输入完整信息"));
    showToast(t("user:account_sec.phone_changed", "手机号已更变"));
  };

  return (
    <PageLayout title={t("user:account_sec.bind_phone", "绑定手机号")}>
      {step === 1 ? (
        <div className="flex flex-col items-center py-10 px-4">
          <div className="w-16 h-16 bg-primary-blue/10 rounded-full flex items-center justify-center mb-6">
            <span className="text-primary-blue text-3xl">📱</span>
          </div>
          <h3 className="text-[18px] font-medium text-text-main mb-2">{t("user:account_sec.your_phone", "你的手机号码：+86 138****8888")}</h3>
          <p className="text-[14px] text-text-sub text-center mb-8">{t("user:account_sec.bind_phone_desc", "绑定的手机号可用于登录 Sdkwork IM H5，或找回密码。")}</p>
          <button
            onClick={() => setStep(2)}
            className="w-full h-12 bg-accent-green text-white rounded-lg font-medium active:opacity-80 transition-opacity"
          >{t("user:account_sec.change_phone", "更换手机号")}</button>
        </div>
      ) : (
        <div className="px-4 py-6">
          <h3 className="text-[20px] font-medium text-text-main mb-6">{t("user:account_sec.verify_new_phone", "验证新手机号")}</h3>
          <div className="flex items-center border-b border-border-color py-3 mb-4">
            <span className="text-[16px] text-text-main mr-4">+86</span>
            <input
              type="tel"
              placeholder={t("user:account_sec.enter_phone", "请填写手机号")}
              value={phone}
              onChange={(e) => setPhone(e.target.value)}
              className="flex-1 bg-transparent text-[16px] text-text-main outline-none"
            />
          </div>
          <div className="flex items-center border-b border-border-color py-3 mb-8">
            <input
              type="text"
              placeholder={t("user:account_sec.verification_code", "验证码")}
              value={code}
              onChange={(e) => setCode(e.target.value)}
              className="flex-1 bg-transparent text-[16px] text-text-main outline-none"
            />
            <button
              className="text-accent-green text-[15px] font-medium ml-4"
              onClick={() => showToast(t("user:account_sec.code_sent", "验证码已发送"))}
            >{t("user:account_sec.get_code", "获取验证码")}</button>
          </div>
          <button
            className="w-full h-12 bg-accent-green text-white rounded-lg font-medium active:opacity-80 transition-opacity"
            onClick={handleSubmit}
          >{t("user:account_sec.submit", "提交")}</button>
        </div>
      )}
    </PageLayout>
  );
};

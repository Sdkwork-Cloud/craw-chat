import React, { useState } from "react";
import { useTranslation } from "react-i18next";
import { PageLayout } from "../SettingsSubPages";
import { showToast } from "@sdkwork/im-h5-commons";

/**
 * Change phone number — fail-closed (PRD).
 *
 * The IAM binding endpoint (`iam.users.current.phoneBindings.create`) exists,
 * but the verification-code delivery for the phone-bind scene is not composed
 * in the H5 frontend (`verificationCodeRequests` only serves LOGIN/REGISTER/
 * RESET_PASSWORD). Until the bind-code flow is wired, no fake code or success
 * is shown: every action surfaces the typed unavailable state.
 */
export const ChangePhoneNumber: React.FC = () => {
  const { t } = useTranslation();
  const [step, setStep] = useState(1);
  const [phone, setPhone] = useState("");
  const [code, setCode] = useState("");

  const unavailable = () =>
    showToast(t("commons.feature_unavailable", "This feature is not available yet; the real service is being connected. Stay tuned."));

  const handleSubmit = () => {
    if (!phone || !code) return showToast(t("user:account_sec.enter_full_info", "Please enter the full information"));
    unavailable();
  };

  return (
    <PageLayout title={t("user:account_sec.bind_phone", "Link phone number")}>
      {step === 1 ? (
        <div className="flex flex-col items-center py-10 px-4">
          <div className="w-16 h-16 bg-primary-blue/10 rounded-full flex items-center justify-center mb-6">
            <span className="text-primary-blue text-3xl">📱</span>
          </div>
          <h3 className="text-[18px] font-medium text-text-main mb-2">{t("user:account_sec.your_phone", "Your phone number: +86 138****8888")}</h3>
          <p className="text-[14px] text-text-sub text-center mb-8">{t("user:account_sec.bind_phone_desc", "Your linked phone number can be used to log into Sdkwork IM H5 or recover your password.")}</p>
          <button
            onClick={() => setStep(2)}
            className="w-full h-12 bg-accent-green text-white rounded-lg font-medium active:opacity-80 transition-opacity"
          >{t("user:account_sec.change_phone", "Change phone number")}</button>
        </div>
      ) : (
        <div className="px-4 py-6">
          <h3 className="text-[20px] font-medium text-text-main mb-6">{t("user:account_sec.verify_new_phone", "Verify new phone number")}</h3>
          <div className="flex items-center border-b border-border-color py-3 mb-4">
            <span className="text-[16px] text-text-main mr-4">+86</span>
            <input
              type="tel"
              placeholder={t("user:account_sec.enter_phone", "Enter your phone number")}
              value={phone}
              onChange={(e) => setPhone(e.target.value)}
              className="flex-1 bg-transparent text-[16px] text-text-main outline-none"
            />
          </div>
          <div className="flex items-center border-b border-border-color py-3 mb-8">
            <input
              type="text"
              placeholder={t("user:account_sec.verification_code", "Verification code")}
              value={code}
              onChange={(e) => setCode(e.target.value)}
              className="flex-1 bg-transparent text-[16px] text-text-main outline-none"
            />
            <button
              className="text-accent-green text-[15px] font-medium ml-4"
              onClick={unavailable}
            >{t("user:account_sec.get_code", "Get code")}</button>
          </div>
          <button
            className="w-full h-12 bg-accent-green text-white rounded-lg font-medium active:opacity-80 transition-opacity"
            onClick={handleSubmit}
          >{t("user:account_sec.submit", "Submit")}</button>
        </div>
      )}
    </PageLayout>
  );
};

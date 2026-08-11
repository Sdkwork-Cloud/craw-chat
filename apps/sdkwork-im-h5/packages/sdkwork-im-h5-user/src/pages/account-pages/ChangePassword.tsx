import React, { useState } from "react";
import { useTranslation } from "react-i18next";
import { PageLayout } from "../SettingsSubPages";
import { showToast } from "@sdkwork/im-h5-commons";
import { getIamAppSdkClient } from "@sdkwork/im-h5-core/sdk";

/**
 * Change password — REAL IAM call.
 *
 * PATCH /app/v3/api/iam/users/current/password (generated IAM app SDK,
 * `iam.users.current.password.update`) with
 * `{ oldPassword, newPassword, confirmPassword }`. No fake success: the
 * server response (or its typed problem-detail error) is surfaced as-is.
 */
export const ChangePassword: React.FC = () => {
  const { t } = useTranslation();
  const [oldPassword, setOldPassword] = useState("");
  const [newPassword, setNewPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [submitting, setSubmitting] = useState(false);

  const handleSubmit = async () => {
    if (!oldPassword || !newPassword || !confirmPassword) {
      return showToast(t("user:account_sec.enter_full_info", "Please enter the full information"));
    }
    if (newPassword !== confirmPassword) {
      return showToast(t("user:account_sec.pwd_mismatch", "The two new passwords do not match"));
    }
    setSubmitting(true);
    try {
      await getIamAppSdkClient().iam.users.current.password.update({
        oldPassword,
        newPassword,
        confirmPassword,
      });
      showToast(t("user:account_sec.password_changed", "Password changed"));
      setOldPassword("");
      setNewPassword("");
      setConfirmPassword("");
    } catch (error) {
      const message = error instanceof Error ? error.message : undefined;
      showToast(
        message || t("user:account_sec.password_change_failed", "Password change failed, please try again later"),
      );
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <PageLayout title={t("user:account_sec.set_password", "Set password")}>
      <div className="px-4 py-6">
        <div className="border-b border-border-color py-3 mb-2">
          <input
            type="password"
            value={oldPassword}
            onChange={(e) => setOldPassword(e.target.value)}
            placeholder={t("user:account_sec.enter_old_pwd", "Enter your current password")}
            className="w-full bg-transparent text-[16px] text-text-main outline-none"
          />
        </div>
        <div className="border-b border-border-color py-3 mb-2">
          <input
            type="password"
            value={newPassword}
            onChange={(e) => setNewPassword(e.target.value)}
            placeholder={t("user:account_sec.enter_new_pwd", "Enter a new password")}
            className="w-full bg-transparent text-[16px] text-text-main outline-none"
          />
        </div>
        <div className="border-b border-border-color py-3 mb-8">
          <input
            type="password"
            value={confirmPassword}
            onChange={(e) => setConfirmPassword(e.target.value)}
            placeholder={t("user:account_sec.confirm_new_pwd", "Enter the new password again")}
            className="w-full bg-transparent text-[16px] text-text-main outline-none"
          />
        </div>
        <p className="text-[13px] text-text-sub mb-8">{t("user:account_sec.pwd_requirements", "The password must contain letters and numbers and be at least 8 characters long.")}</p>
        <button
          disabled={submitting}
          className="w-full h-12 bg-accent-green text-white rounded-lg font-medium active:opacity-80 transition-opacity disabled:opacity-60"
          onClick={() => void handleSubmit()}
        >{t("user:account_sec.done", "Done")}</button>
      </div>
    </PageLayout>
  );
};

import React from "react";
import { useTranslation } from "react-i18next";
import { PageLayout } from "../SettingsSubPages";
import { showToast } from "@sdkwork/im-h5-commons";

export const ChangePassword: React.FC = () => {
  const { t } = useTranslation();

  return (
    <PageLayout title={t("user:account_sec.set_password", "设置密码")}>
      <div className="px-4 py-6">
        <div className="border-b border-border-color py-3 mb-2">
          <input
            type="password"
            placeholder={t("user:account_sec.enter_old_pwd", "请填写原密码")}
            className="w-full bg-transparent text-[16px] text-text-main outline-none"
          />
        </div>
        <div className="border-b border-border-color py-3 mb-2">
          <input
            type="password"
            placeholder={t("user:account_sec.enter_new_pwd", "请填写新密码")}
            className="w-full bg-transparent text-[16px] text-text-main outline-none"
          />
        </div>
        <div className="border-b border-border-color py-3 mb-8">
          <input
            type="password"
            placeholder={t("user:account_sec.confirm_new_pwd", "请再次填写新密码")}
            className="w-full bg-transparent text-[16px] text-text-main outline-none"
          />
        </div>
        <p className="text-[13px] text-text-sub mb-8">{t("user:account_sec.pwd_requirements", "密码必须包含字母和数字，且长度不少于8位。")}</p>
        <button
          className="w-full h-12 bg-[#00B42A] text-white rounded-lg font-medium active:opacity-80 transition-opacity"
          onClick={() => showToast(t("user:account_sec.operation_executed", "操作已执行"))}
        >{t("user:account_sec.done", "完成")}</button>
      </div>
    </PageLayout>
  );
};

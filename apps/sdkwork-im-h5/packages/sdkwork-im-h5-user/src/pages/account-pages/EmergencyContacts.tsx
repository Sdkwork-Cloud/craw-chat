import React from "react";
import { useTranslation } from "react-i18next";
import { PageLayout } from "../SettingsSubPages";
import { Plus } from "lucide-react";
import { showToast } from "@sdkwork/im-h5-commons";

export const EmergencyContacts: React.FC = () => {
  const { t } = useTranslation();

  return (
    <PageLayout title={t("user:account_sec.emergency_contacts", "应急联系人")}>
      <div className="flex flex-col items-center py-10 px-4">
        <div className="w-16 h-16 bg-[#00B42A]/10 rounded-full flex items-center justify-center mb-6">
          <span className="text-[#00B42A] text-3xl">👥</span>
        </div>
        <h3 className="text-[18px] font-medium text-text-main mb-2">{t("user:account_sec.add_emergency_contact", "添加应急联系人")}</h3>
        <p className="text-[14px] text-text-sub text-center mb-8">{t("user:account_sec.emergency_desc", "当你的账号遇到安全风险或忘记密码时，可通过应急联系人辅助找回。建议设置3位以上联系人。")}</p>
        <div
          className="w-full h-14 bg-[#00B42A]/10 border border-dashed border-[#00B42A] rounded-xl flex items-center justify-center text-[#00B42A] font-medium cursor-pointer active:opacity-70 transition-opacity"
          onClick={() => showToast(t("user:account_sec.select_from_contacts", "请从通讯录选择"))}
        >
          <Plus className="w-5 h-5 mr-2" />
          {t("user:account_sec.add_contact", "添加联系人")}
        </div>
      </div>
    </PageLayout>
  );
};

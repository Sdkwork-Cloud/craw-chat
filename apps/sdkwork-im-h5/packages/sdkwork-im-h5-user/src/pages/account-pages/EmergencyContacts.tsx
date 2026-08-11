import React from "react";
import { useTranslation } from "react-i18next";
import { PageLayout } from "../SettingsSubPages";
import { Plus } from "lucide-react";
import { showToast } from "@sdkwork/im-h5-commons";

/**
 * Emergency contacts — fail-closed (PRD). No real emergency-contact API is
 * composed, so the add action surfaces the typed unavailable state instead of
 * a fake "select from contacts" flow.
 */
export const EmergencyContacts: React.FC = () => {
  const { t } = useTranslation();

  return (
    <PageLayout title={t("user:account_sec.emergency_contacts", "Emergency contacts")}>
      <div className="flex flex-col items-center py-10 px-4">
        <div className="w-16 h-16 bg-accent-green/10 rounded-full flex items-center justify-center mb-6">
          <span className="text-accent-green text-3xl">👥</span>
        </div>
        <h3 className="text-[18px] font-medium text-text-main mb-2">{t("user:account_sec.add_emergency_contact", "Add emergency contact")}</h3>
        <p className="text-[14px] text-text-sub text-center mb-8">{t("user:account_sec.emergency_desc", "If your account is at risk or you forget your password, emergency contacts can help you recover it. We recommend adding at least 3 contacts.")}</p>
        <div
          className="w-full h-14 bg-accent-green/10 border border-dashed border-accent-green rounded-xl flex items-center justify-center text-accent-green font-medium cursor-pointer active:opacity-70 transition-opacity"
          onClick={() => showToast(t("commons.feature_unavailable", "This feature is not available yet while the real service is being integrated."))}
        >
          <Plus className="w-5 h-5 mr-2" />
          {t("user:account_sec.add_contact", "Add contact")}
        </div>
      </div>
    </PageLayout>
  );
};

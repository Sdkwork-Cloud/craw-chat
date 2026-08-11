import React from "react";
import { useTranslation } from 'react-i18next';
import { PageLayout } from "../../SettingsSubPages";

/**
 * Authorized-app management — fail-closed (PRD).
 *
 * No real OAuth app-authorization API is composed: the previously hard-coded
 * authorized-app list (WPS / 滴滴 / 京东) and the fake "已解除授权" success
 * were fabricated and are removed. The page renders the typed unavailable
 * state instead.
 */
export const AuthManagement: React.FC = () => {
  const { t } = useTranslation();

  return (
    <PageLayout title={t('user.auto_prop_2ed19e80', "Manage permissions")}>
      <div className="flex flex-col items-center justify-center gap-3 px-8 text-center py-20">
        <span className="text-[40px]">🔐</span>
        <p className="text-[15px] text-text-main">
          {t("commons.feature_unavailable", "This feature is not available yet while the real service is being integrated.")}
        </p>
      </div>
    </PageLayout>
  );
};

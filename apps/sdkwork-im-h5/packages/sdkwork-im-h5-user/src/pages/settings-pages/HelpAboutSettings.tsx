import React from "react";
import { useNavigate } from "react-router";
import { useTranslation } from 'react-i18next';
import { PageLayout, Group, ListItem } from "../../components/SettingsCommons";

export const HelpFeedback = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  return (
    <PageLayout title={t("user:help_about.title", "Help & feedback")}>
      <Group>
        <ListItem
          label={t("user:help_about.faq", "FAQ")}
          onClick={() => navigate("/settings/help/faq")}
        />
        <ListItem
          label={t("user:help_about.feedback", "Feedback")}
          hideBorder
          onClick={() => navigate("/settings/help/feedback")}
        />
      </Group>
    </PageLayout>
  );
};

export const About = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  return (
    <PageLayout title={t("user:help_about.about_title", "About Sdkwork IM H5")}>
      <div className="flex flex-col items-center py-10">
        <div className="w-20 h-20 bg-primary-blue rounded-2xl flex items-center justify-center mb-4 shadow-lg">
          <span className="text-white text-3xl font-bold">C</span>
        </div>
        <h3 className="text-[20px] font-bold text-text-main mb-1">ClawChat</h3>
        <p className="text-[14px] text-text-sub mb-8">Version 1.0.0</p>
      </div>
      <Group>
        <ListItem
          label={t("user:help_about.features", "Features")}
          onClick={() => navigate("/settings/about/features")}
        />
        <ListItem
          label={t("user:help_about.complain", "Report")}
          onClick={() => navigate("/settings/about/complain")}
        />
        <ListItem
          label={t("user:help_about.terms", "Software License and Service Agreement")}
          onClick={() => navigate("/settings/about/tos")}
        />
        <ListItem
          label={t("user:help_about.privacy", "Privacy Protection Guidelines")}
          hideBorder
          onClick={() => navigate("/settings/about/privacy")}
        />
      </Group>
    </PageLayout>
  );
};

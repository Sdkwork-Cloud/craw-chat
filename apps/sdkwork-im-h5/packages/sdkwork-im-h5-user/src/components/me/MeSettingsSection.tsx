import React from "react";
import { Settings } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { MenuItem } from "../MenuItem";

export const MeSettingsSection: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();

  return (
    <div className="mb-6 border-y border-border-color flex flex-col bg-chat-other-bg">
      <MenuItem
        icon={Settings}
        label={t('user.auto_prop_116b70', 'Settings')}
        colorClass="text-zinc-500"
        onClick={() => navigate("/settings")}
      />
    </div>
  );
};

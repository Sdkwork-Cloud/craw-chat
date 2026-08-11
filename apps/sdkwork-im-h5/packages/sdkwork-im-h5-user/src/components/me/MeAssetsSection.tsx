import React from "react";
import { UserRound, Mic } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { MenuItem } from "../MenuItem";

export const MeAssetsSection: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();

  return (
    <div className="mb-2 border-y border-border-color flex flex-col bg-chat-other-bg">
      <MenuItem
        icon={UserRound}
        label={t('user.auto_prop_2e6230b3', 'My characters')}
        colorClass="text-emerald-500"
        onClick={() => navigate("/me/characters")}
      />
      <div className="h-[0.5px] bg-border-color ml-[52px]" />
      <MenuItem
        icon={Mic}
        label={t('user.auto_prop_2e5c5ad6', 'My voices')}
        colorClass="text-purple-500"
        onClick={() => navigate("/me/voices")}
      />
    </div>
  );
};

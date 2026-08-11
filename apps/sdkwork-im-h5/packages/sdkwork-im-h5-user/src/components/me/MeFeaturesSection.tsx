import React from "react";
import {
  Bookmark,
  BookOpen,
  Bot,
  Compass,
  Cpu,
  Folder,
  Smile,
} from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { MenuItem } from "../MenuItem";

export const MeFeaturesSection: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();

  return (
    <div className="mb-2 border-y border-border-color flex flex-col bg-chat-other-bg">
      <MenuItem
        icon={Bookmark}
        label={t('user.auto_prop_cc759', 'Favorites')}
        colorClass="text-rose-500"
        onClick={() => navigate("/me/favorites")}
      />
      <div className="h-[0.5px] bg-border-color ml-[52px]" />
      <MenuItem
        icon={BookOpen}
        label={t('user.auto_prop_1d35e32', 'Knowledge base')}
        colorClass="text-indigo-500"
        onClick={() => navigate("/workspace/knowledge")}
      />
      <div className="h-[0.5px] bg-border-color ml-[52px]" />
      <MenuItem
        icon={Bot}
        label={t('user.auto_prop_1909df0', 'Agents')}
        colorClass="text-blue-500"
        onClick={() => navigate("/me/agents")}
      />
      <div className="h-[0.5px] bg-border-color ml-[52px]" />
      <MenuItem
        icon={Compass}
        label={t('user.auto_prop_2e5be31b', 'My communities')}
        colorClass="text-emerald-500"
        onClick={() => navigate("/community")}
      />
      <div className="h-[0.5px] bg-border-color ml-[52px]" />
      <MenuItem
        icon={Cpu}
        label={t('user.auto_prop_e913e20', 'My smart devices')}
        colorClass="text-rose-500"
        onClick={() => navigate("/hardware")}
      />
      <div className="h-[0.5px] bg-border-color ml-[52px]" />
      <MenuItem
        icon={Folder}
        label={t('user.auto_prop_2e5aeeb8', 'My works')}
        colorClass="text-purple-500"
        onClick={() => navigate("/me/works")}
      />
      <div className="h-[0.5px] bg-border-color ml-[52px]" />
      <MenuItem
        icon={Smile}
        label={t('user.auto_prop_10e55d', 'Stickers')}
        colorClass="text-orange-500"
        onClick={() => navigate("/me/emoji")}
      />
    </div>
  );
};

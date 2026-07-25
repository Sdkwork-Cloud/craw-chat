import { useTranslation } from "react-i18next";
import React from "react";
import { useNavigate } from "react-router";
import {
  Settings,
  Wallet,
  Bookmark,
  BookOpen,
  Smile,
  Bot,
  Folder,
  Package,
  UserRound,
  Mic,
  Cpu,
  Compass,
} from "lucide-react";
import { useAppStore } from "@sdkwork/im-h5-core";

import { MenuItem } from "../components/MenuItem";
import { ProfileHeaderCard } from "../components/ProfileHeaderCard";
import { MeHeader } from "../components/MeHeader";

export const Me: React.FC = () => {
  const { t } = useTranslation();
  const { currentUser } = useAppStore();
  const navigate = useNavigate();

  return (
    <div className="flex flex-col h-full bg-[#f4f6f9] dark:bg-[#0a0a0a] overflow-y-auto pb-[84px]">
      {/* Header */}
      <MeHeader onContactClick={() => navigate('/workspace/contacts')} />

      <div className="flex flex-col mt-2">
        {/* Profile Section */}
        <ProfileHeaderCard
          currentUser={currentUser}
          onClick={() => navigate("/my-profile")}
        />

        {/* Services */}
        <div className="mb-2 border-y border-border-color flex flex-col bg-chat-other-bg">
          <MenuItem
            icon={Wallet}
            label={t('user.auto_prop_ccd34', '服务')}
            colorClass="text-blue-500"
            onClick={() => navigate("/me/services")}
          />
          <div className="h-[0.5px] bg-border-color ml-[52px]" />
          <MenuItem
            icon={Package}
            label={t('user.auto_prop_40bbe269', '订单中心')}
            colorClass="text-orange-500"
            onClick={() => navigate("/me/orders")}
          />
        </div>

        {/* AI Assets */}
        <div className="mb-2 border-y border-border-color flex flex-col bg-chat-other-bg">
          <MenuItem
            icon={UserRound}
            label={t('user.auto_prop_2e6230b3', '我的角色')}
            colorClass="text-emerald-500"
            onClick={() => navigate("/me/characters")}
          />
          <div className="h-[0.5px] bg-border-color ml-[52px]" />
          <MenuItem
            icon={Mic}
            label={t('user.auto_prop_2e5c5ad6', '我的声音')}
            colorClass="text-purple-500"
            onClick={() => navigate("/me/voices")}
          />
        </div>

        {/* Features */}
        <div className="mb-2 border-y border-border-color flex flex-col bg-chat-other-bg">
          <MenuItem
            icon={Bookmark}
            label={t('user.auto_prop_cc759', '收藏')}
            colorClass="text-rose-500"
            onClick={() => navigate("/me/favorites")}
          />
          <div className="h-[0.5px] bg-border-color ml-[52px]" />
          <MenuItem
            icon={BookOpen}
            label={t('user.auto_prop_1d35e32', '知识库')}
            colorClass="text-indigo-500"
            onClick={() => navigate("/workspace/knowledge")}
          />
          <div className="h-[0.5px] bg-border-color ml-[52px]" />
          <MenuItem
            icon={Bot}
            label={t('user.auto_prop_1909df0', '智能体')}
            colorClass="text-blue-500"
            onClick={() => navigate("/me/agents")}
          />
          <div className="h-[0.5px] bg-border-color ml-[52px]" />
          <MenuItem
            icon={Compass}
            label={t('user.auto_prop_2e5be31b', '我的圈子')}
            colorClass="text-emerald-500"
            onClick={() => navigate("/me/communities")}
          />
          <div className="h-[0.5px] bg-border-color ml-[52px]" />
          <MenuItem
            icon={Cpu}
            label={t('user.auto_prop_e913e20', '我的智能硬件')}
            colorClass="text-rose-500"
            onClick={() => navigate("/hardware")}
          />
          <div className="h-[0.5px] bg-border-color ml-[52px]" />
          <MenuItem
            icon={Folder}
            label={t('user.auto_prop_2e5aeeb8', '我的作品')}
            colorClass="text-purple-500"
            onClick={() => navigate("/me/works")}
          />
          <div className="h-[0.5px] bg-border-color ml-[52px]" />
          <MenuItem
            icon={Smile}
            label={t('user.auto_prop_10e55d', '表情')}
            colorClass="text-orange-500"
            onClick={() => navigate("/me/emoji")}
          />
        </div>

        {/* Settings */}
        <div className="mb-6 border-y border-border-color flex flex-col bg-chat-other-bg">
          <MenuItem
            icon={Settings}
            label={t('user.auto_prop_116b70', '设置')}
            colorClass="text-zinc-500"
            onClick={() => navigate("/settings")}
          />
        </div>
      </div>
    </div>
  );
};


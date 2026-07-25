import { useTranslation } from "react-i18next";
import React from "react";
import { useNavigate } from "react-router";
import {
  ChevronRight,
  Camera,
  Scan,
  Gamepad2,
  Search,
  Video,
  ShoppingBag,
  Compass,
  BookOpen,
  Building2,
  Contact,
} from "lucide-react";
import { cn, IconButton } from "@sdkwork/im-h5-commons";
import { DiscoverItem } from "../components/DiscoverItem";

export const Discover: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();

  return (
    <div className="flex flex-col h-full bg-[#f4f6f9] dark:bg-[#0a0a0a] overflow-y-auto pb-[84px]">
      {/* Header */}
      <header className="h-[56px] px-4 flex items-center justify-between sticky top-0 z-10 shrink-0 pt-safe bg-[#f4f6f9]/90 dark:bg-[#0a0a0a]/90 backdrop-blur-xl">
        <div className="w-[32px]" />
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
          <h1 className="text-[17px] font-bold text-text-main tracking-tight">{t('user.auto_a99ff', '发现')}</h1>
        </div>
        <div className="flex justify-end">
          <IconButton
            icon={<Contact className="w-[22px] h-[22px] text-text-main" />}
            onClick={() => navigate('/workspace/contacts')}
          />
        </div>
      </header>

      <div className="flex flex-col mt-2">
        {/* Moments */}
        <div className="mb-2 border-y border-border-color bg-chat-other-bg">
          <DiscoverItem
            icon={Camera}
            label={t('user.auto_prop_18d4ce8', '朋友圈')}
            colorClass="text-[#3b82f6]"
            hasBorder={false}
            onClick={() => navigate("/discover/moments")}
            rightElement={
              <div className="relative">
                <img
                  src="https://picsum.photos/seed/moment/32/32"
                  alt="New moment"
                  className="w-8 h-8 rounded-md"
                />
                <div className="absolute -top-1 -right-1 w-2.5 h-2.5 bg-red-500 rounded-full border-2 border-chat-other-bg" />
              </div>
            }
          />
        </div>

        {/* Course */}
        <div className="mb-2 border-y border-border-color bg-chat-other-bg">
          <DiscoverItem
            icon={BookOpen}
            label={t('user.auto_prop_298bb0a4', '在线课程')}
            colorClass="text-emerald-500"
            hasBorder={false}
            onClick={() => navigate("/course")}
          />
        </div>

        {/* Channels */}
        <div className="mb-2 border-y border-border-color flex flex-col bg-chat-other-bg">
          <DiscoverItem
            icon={Video}
            label={t('user.auto_prop_9f0e5', '作品')}
            hasBorder={false}
            colorClass="text-orange-500"
            onClick={() => navigate("/discover/channels")}
          />
        </div>

        {/* Community */}
        <div className="mb-2 border-y border-border-color bg-chat-other-bg">
          <DiscoverItem
            icon={Compass}
            label={t('user.auto_prop_ae548', '圈子')}
            colorClass="text-blue-500"
            hasBorder={false}
            onClick={() => navigate("/community")}
          />
        </div>

        {/* Scan & Search */}
        <div className="mb-2 border-y border-border-color flex flex-col bg-chat-other-bg">
          <DiscoverItem
            icon={Building2}
            label={t('user.auto_prop_2518cc2f', '企业中心')}
            colorClass="text-indigo-500"
            onClick={() => navigate("/enterprise")}
          />
          <DiscoverItem
            icon={Scan}
            label={t('user.auto_prop_17b4816', '扫一扫')}
            colorClass="text-blue-500"
            onClick={() => navigate("/scan")}
          />
          <DiscoverItem
            icon={Search}
            label={t('user.auto_prop_181a338', '搜一搜')}
            colorClass="text-rose-500"
            hasBorder={false}
            onClick={() => navigate("/discover/search")}
          />
        </div>

        {/* Games & Shopping */}
        <div className="mb-2 border-y border-border-color flex flex-col bg-chat-other-bg">
          <DiscoverItem
            icon={Gamepad2}
            label={t('user.auto_prop_dbad7', '游戏')}
            colorClass="text-green-500"
            onClick={() => navigate("/discover/games")}
          />
          <DiscoverItem
            icon={ShoppingBag}
            label={t('user.auto_prop_118adc', '购物')}
            colorClass="text-orange-500"
            hasBorder={false}
            onClick={() => navigate("/discover/shopping")}
          />
        </div>
      </div>
    </div>
  );
};

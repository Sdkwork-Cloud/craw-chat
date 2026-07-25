import { useTranslation } from "react-i18next";
import React from "react";
import { cn, IconButton } from "@sdkwork/im-h5-commons";
import { Users, MessageSquare, Check, MoreHorizontal } from "lucide-react";
import { Community } from "../types";
import { useNavigate } from "react-router";

interface CommunityCardProps {
  community: Community;
  onClick?: () => void;
  onLongPressProps?: any;
  onMoreClick?: (e: React.MouseEvent) => void;
  onJoinClick?: (e: React.MouseEvent) => void;
}

export const CommunityCard: React.FC<CommunityCardProps> = ({
  community,
  onClick,
  onLongPressProps,
  onMoreClick,
  onJoinClick,
}) => {
  const { t } = useTranslation();
return (
    <div
      className="bg-white dark:bg-[#1E1E1E] mb-2 overflow-hidden cursor-pointer active:bg-black/5 dark:active:bg-white/5 transition-colors border-b border-black/5 dark:border-white/5"
      onClick={onClick}
      {...onLongPressProps}
    >
      <div className="h-[120px] w-full relative overflow-hidden pointer-events-none">
        <img
          src={community.coverImage}
          alt={community.name}
          className="w-full h-full object-cover"
        />
        <div className="absolute inset-0 bg-gradient-to-t from-black/60 to-transparent"></div>
        {onMoreClick && (
          <div className="absolute top-2 right-2 flex items-center gap-2 pointer-events-auto">
            <IconButton
              icon={<MoreHorizontal className="w-5 h-5 text-white shadow-sm" />}
              className="bg-black/20 backdrop-blur-md w-8 h-8 rounded-full"
              onClick={onMoreClick}
            />
          </div>
        )}
        <div className="absolute bottom-3 left-4 flex flex-col gap-0.5 pr-4">
          <h3 className="text-white text-[18px] font-bold shadow-sm line-clamp-1">
            {community.name}
          </h3>
          <div className="flex gap-2">
            {community.tags.slice(0, 3).map((tag, idx) => (
              <span
                key={idx}
                className="bg-white/20 backdrop-blur-md px-1.5 py-0.5 rounded text-[11px] text-white shadow-sm"
              >
                #{tag}
              </span>
            ))}
          </div>
        </div>
      </div>

      <div className="p-3 flex flex-col gap-2 pointer-events-none">
        <p className="text-[13px] text-text-sub line-clamp-2 leading-relaxed">
          {community.description}
        </p>

        <div className="flex items-center justify-between mt-1">
          <div className="flex items-center gap-4">
            <div className="flex items-center gap-1 text-text-sub">
              <Users className="w-4 h-4" />
              <span className="text-[12px]">{t('community.auto_n4e746140', '{community.memberCount} 成员')}</span>
            </div>
            <div className="flex items-center gap-1 text-text-sub">
              <MessageSquare className="w-4 h-4" />
              <span className="text-[12px]">{t('community.auto_ndae6275', '{community.postCount} 动态')}</span>
            </div>
          </div>

          {onJoinClick && (
            <div className="pointer-events-auto">
              {community.isJoined ? (
                <div className="px-3 py-1.5 rounded-full border border-black/10 dark:border-white/10 text-text-sub flex items-center gap-1 bg-black/5 dark:bg-white/5">
                  <Check className="w-3.5 h-3.5" />
                  <span className="text-[13px] font-medium">{t('community.auto_16afc37', '已加入')}</span>
                </div>
              ) : community.isPaid ? (
                <button
                  onClick={onJoinClick}
                  className="px-4 py-1.5 rounded-full bg-orange-500 text-white font-medium text-[13px] shadow-sm shadow-orange-500/20 active:scale-[0.98] transition-transform"
                >{t('community.auto_n3990cdea', '¥{community.price} 加入')}</button>
              ) : (
                <button
                  onClick={onJoinClick}
                  className="px-4 py-1.5 rounded-full bg-blue-500 text-white font-medium text-[13px] shadow-sm shadow-blue-500/20 active:scale-[0.98] transition-transform"
                >{t('community.auto_27118551', '免费加入')}</button>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

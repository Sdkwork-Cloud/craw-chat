import { useTranslation } from "react-i18next";
import React from "react";
import { CommunityGroup } from "../types";
import { IconButton } from "@sdkwork/im-h5-commons";
import { MessageSquare, QrCode } from "lucide-react";
import { useNavigate } from "react-router";

interface GroupListProps {
  groups: CommunityGroup[];
  communityId: string;
  platformNameMap: Record<string, string>;
}

export const GroupList: React.FC<GroupListProps> = ({ groups, communityId, platformNameMap }) => {
  const { t } = useTranslation();
const navigate = useNavigate();

  return (
    <div className="pb-24 flex flex-col bg-white dark:bg-[#1C1C1E]">
      {groups.map(group => (
        <div key={group.id} className="bg-white dark:bg-[#1C1C1E] p-0 flex flex-col gap-0 border-b border-black/5 dark:border-white/5 active:bg-black/5 dark:active:bg-white/5 transition-colors cursor-pointer" onClick={() => navigate(`/community/${communityId}/group/${group.id}`)}>
          <div className="flex items-center justify-between pointer-events-none">
            <div className="flex items-center gap-0 flex-1 overflow-hidden">
              <div className="w-12 h-12 flex shrink-0 items-center justify-center bg-blue-500/10">
                <MessageSquare className="w-6 h-6 text-blue-500" />
              </div>
              <div className="flex flex-col flex-1 overflow-hidden">
                <h3 className="text-[16px] font-bold text-text-main truncate">{group.name}</h3>
                <div className="flex items-center gap-1 mt-0.5">
                  <span className="text-[12px] font-medium px-1 py-0.5 bg-black/5 dark:bg-white/10 text-text-sub">{platformNameMap[group.platform] || group.platform}</span>
                  <span className="text-[12px] text-text-sub">{t('community.auto_n26d9d2a6', '{group.memberCount} 人已加')}</span>
                  <div className="flex items-center gap-1 text-[12px] text-text-sub ml-1 border-l border-black/10 dark:border-white/10 pl-1">
                    <QrCode className="w-3.5 h-3.5" />{t('community.auto_n248999bf', '{(group.qrCodes?.length || 0) + (group.qrCodeUrl && !group.qrCodes?.length ? 1 : 0)} 码')}</div>
                </div>
              </div>
            </div>
          </div>
          
          {group.description && (
            <p className="text-[14px] text-text-sub leading-relaxed pointer-events-none">{group.description}</p>
          )}
        </div>
      ))}
      
      {groups.length === 0 && (
        <div className="h-40 flex items-center justify-center text-text-sub">{t('community.auto_302753be', '暂无群组')}</div>
      )}
    </div>
  );
};

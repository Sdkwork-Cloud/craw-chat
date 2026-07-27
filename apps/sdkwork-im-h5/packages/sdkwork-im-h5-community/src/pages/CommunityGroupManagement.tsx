import { useTranslation } from "react-i18next";
import React, { useState, useEffect, useRef } from "react";
import { useParams, useNavigate } from "react-router";
import { CommunityService } from "../services/CommunityService";
import { CommunityGroup } from "../types";
import { IconButton, showToast, cn, useLongPress } from "@sdkwork/im-h5-commons";
import { ChevronLeft, Plus, MessageSquare, Edit2, Trash2, QrCode } from "lucide-react";

export const CommunityGroupManagement: React.FC = () => {
  const { t } = useTranslation();
const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [groups, setGroups] = useState<CommunityGroup[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    loadData();
  }, [id]);

  const loadData = async () => {
    if (!id) return;
    setIsLoading(true);
    try {
      const g = await CommunityService.getGroupsByCommunity(id);
      setGroups(g);
    } catch {
      showToast(t('community.auto_fn_n638c6acd', '获取群组失败'));
    } finally {
      setIsLoading(false);
    }
  };

  const [actionSheetGroup, setActionSheetGroup] = useState<CommunityGroup | null>(null);

  const longPressGroupRef = useRef<CommunityGroup | null>(null);
  const longPressHandlers = useLongPress({
    delay: 500,
    onLongPress: () => {
      const group = longPressGroupRef.current;
      if (group) {
        setActionSheetGroup(group);
      }
    },
  });

  const startLongPress = (group: CommunityGroup) => ({
    onTouchStart: () => {
      longPressGroupRef.current = group;
      longPressHandlers.onPointerDown();
    },
    onTouchEnd: longPressHandlers.onPointerUp,
    onTouchMove: longPressHandlers.onPointerUp,
    onMouseDown: () => {
      longPressGroupRef.current = group;
      longPressHandlers.onPointerDown();
    },
    onMouseUp: longPressHandlers.onPointerUp,
    onMouseLeave: longPressHandlers.onPointerUp,
  });

  const handleDelete = async (groupId: string) => {
    if (!id) return;
    try {
      await CommunityService.deleteGroup(id, groupId);
      showToast(t('community.auto_fn_41bb1a16', '群组已删除'));
      loadData();
    } catch {
      showToast(t('community.auto_fn_2794e158', '删除失败'));
    }
  };

  const platformNameMap: Record<string, string> = {
    wechat: '微信',
    qq: 'QQ',
    feishu: '飞书',
    dingtalk: '钉钉',
    telegram: 'Telegram',
    discord: 'Discord',
    whatsapp: 'WhatsApp',
    other: '其他'
  };

  if (isLoading) {
    return (
      <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black text-text-main">
         <header className="h-[56px] px-4 flex items-center justify-between sticky top-0 z-10 pt-safe bg-bg-color shrink-0 shadow-sm">
            <IconButton icon={<ChevronLeft className="w-6 h-6 text-text-main" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
            <h1 className="text-[17px] font-semibold flex-1 text-center pr-8">{t('community.auto_3bf0f825', '群组管理')}</h1>
         </header>
         <div className="flex-1 flex items-center justify-center text-text-sub">{t('community.auto_7f6f37e', '加载中...')}</div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black text-text-main">
      <header className="h-[56px] px-4 flex items-center justify-between sticky top-0 z-10 pt-safe bg-bg-color shrink-0 shadow-sm border-b border-black/5 dark:border-white/5">
        <IconButton icon={<ChevronLeft className="w-6 h-6 text-text-main" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
        <h1 className="text-[17px] font-semibold flex-1 text-center">{t('community.auto_3bf0f825', '群组管理')}</h1>
        <IconButton icon={<Plus className="w-6 h-6 text-blue-500" />} className="bg-transparent w-10 h-10 -mr-2" onClick={() => navigate(`/community/${id}/groups/create`)} />
      </header>

      <div className="flex-1 overflow-y-auto w-full pb-safe">
        <div className="flex flex-col bg-white dark:bg-[#1C1C1E]">
           {groups.map(group => {
              const totalQrs = (group.qrCodes?.length || 0) + (group.qrCodeUrl && !group.qrCodes?.length ? 1 : 0);
              return (
                 <div 
                   key={group.id} 
                   className="p-0 flex flex-col border-b border-black/5 dark:border-white/5 active:bg-black/5 dark:active:bg-white/5 transition-colors cursor-pointer select-none"
                   {...startLongPress(group)}
                 >
                    <div className="flex items-start justify-between mb-0">
                       <div className="flex items-center gap-0 flex-1 overflow-hidden pointer-events-none">
                          <div className="w-12 h-12 flex shrink-0 items-center justify-center bg-blue-500/10">
                             <MessageSquare className="w-6 h-6 text-blue-500" />
                          </div>
                          <div className="flex flex-col flex-1 overflow-hidden pr-2">
                             <h3 className="text-[16px] font-bold text-text-main truncate leading-tight">{group.name}</h3>
                             <div className="flex items-center gap-2 mt-1">
                               <span className="text-[11px] font-medium px-1.5 py-0.5 bg-black/5 dark:bg-white/10 rounded text-text-sub">{platformNameMap[group.platform] || group.platform}</span>
                               <div className="flex items-center gap-1 text-[12px] text-text-sub">
                                 <QrCode className="w-3.5 h-3.5" />{t('community.auto_n7d01998b', '{totalQrs} 码')}</div>
                             </div>
                          </div>
                       </div>
                    </div>
                    {group.description && (
                       <div className="text-[14px] text-text-sub leading-relaxed pointer-events-none">
                         {group.description}
                       </div>
                    )}
                 </div>
              );
           })}
           {groups.length === 0 && (
             <div className="flex flex-col items-center justify-center py-20 text-text-sub gap-3 bg-[#F2F2F7] dark:bg-black">
                <MessageSquare className="w-12 h-12 opacity-30" />
                <span className="text-[14px]">{t('community.auto_1be28790', '暂无群组，点击右上角添加')}</span>
             </div>
           )}
        </div>
      </div>

      {actionSheetGroup && (
        <div className="fixed inset-0 z-50 flex flex-col justify-end pointer-events-auto">
          <div 
            className="absolute inset-0 bg-black/40 transition-opacity"
            onClick={() => setActionSheetGroup(null)}
          />
          <div className="bg-[#F2F2F7] dark:bg-[#1C1C1E] rounded-t-2xl w-full relative z-10 overflow-hidden pb-safe animate-in slide-in-from-bottom duration-300">
            <div className="p-4 flex items-center justify-center border-b border-black/5 dark:border-white/5 bg-white dark:bg-[#2C2C2E]">
               <span className="text-[13px] text-text-sub">{actionSheetGroup.name}</span>
            </div>
            
            <div className="flex flex-col">
               <button 
                  onClick={() => {
                    navigate(`/community/${id}/profile/groups/edit/${actionSheetGroup.id}`);
                    setActionSheetGroup(null);
                  }}
                  className="bg-white dark:bg-[#2C2C2E] py-4 text-[16px] text-text-main border-b border-black/5 dark:border-white/5 active:bg-black/5 dark:active:bg-white/5 transition-colors"
               >{t('community.auto_3bf0041b', '编辑群组')}</button>
               <button 
                  onClick={() => {
                    handleDelete(actionSheetGroup.id);
                    setActionSheetGroup(null);
                  }}
                  className="bg-white dark:bg-[#2C2C2E] py-4 text-[16px] text-red-500 active:bg-black/5 dark:active:bg-white/5 transition-colors"
               >{t('community.auto_27997ae4', '删除群组')}</button>
            </div>
            
            <div className="mt-2">
               <button 
                  onClick={() => setActionSheetGroup(null)}
                  className="w-full bg-white dark:bg-[#2C2C2E] py-4 text-[16px] font-medium text-text-main active:bg-black/5 dark:active:bg-white/5 transition-colors"
               >{t('community.auto_a9472', '取消')}</button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

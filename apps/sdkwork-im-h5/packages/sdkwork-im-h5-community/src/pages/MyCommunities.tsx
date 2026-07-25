import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useNavigate, useSearchParams } from "react-router";
import { CommunityService } from "../services/CommunityService";
import { Community } from "../types";
import { cn, IconButton, Tabs, ActionSheet, showToast } from "@sdkwork/im-h5-commons";
import { ChevronLeft, Compass, Users, MessageSquare, MoreHorizontal, FileText, Share, Trash2, LogOut } from "lucide-react";
import { CommunityCard } from "../components/CommunityCard";

export const MyCommunities: React.FC = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const initialTab = searchParams.get('tab') || 'joined';
  
  const [activeTab, setActiveTab] = useState<string>(initialTab);
  const [communities, setCommunities] = useState<Community[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [actionSheetCommunity, setActionSheetCommunity] = useState<Community | null>(null);
  const [isLongPressed, setIsLongPressed] = useState(false);

  useEffect(() => {
    loadData();
  }, [activeTab]);

  const loadData = async () => {
    setIsLoading(true);
    try {
      const all = await CommunityService.getCommunities();
      if (activeTab === 'joined') {
        setCommunities(all.filter(c => c.isJoined));
      } else {
        setCommunities(all.slice(0, 2)); // Mocked
      }
    } catch {
    } finally {
      setIsLoading(false);
    }
  };

  const startLongPress = (community: Community) => {
  const handlePressStart = () => {
  setIsLongPressed(false);
      (window as any).longPressTimeout = setTimeout(() => {
        setIsLongPressed(true);
        setActionSheetCommunity(community);
      }, 500);
    };

    const handlePressEnd = () => {
  clearTimeout((window as any).longPressTimeout);
    };

    return {
      onPointerDown: handlePressStart,
      onPointerUp: handlePressEnd,
      onPointerLeave: () => {
        handlePressEnd();
        setIsLongPressed(false);
      },
      onPointerMove: () => {
         // optional: if they move a lot, cancel it
      },
      onContextMenu: (e: React.MouseEvent) => {
        e.preventDefault();
        handlePressStart(); // Trigger immediately on context menu (right click)
        setIsLongPressed(true);
        setActionSheetCommunity(community);
        handlePressEnd();
      }
    };
  };

  const handleActionSheetSelect = (action: string) => {
  if (!actionSheetCommunity) return;
    
    if (action === 'edit') {
       navigate(`/community/${actionSheetCommunity.id}/profile`);
    } else if (action === 'delete') {
       setCommunities(prev => prev.filter(c => c.id !== actionSheetCommunity.id));
       showToast(t('community.auto_fn_16b31b6', '已删除'));
    } else if (action === 'leave') {
       setCommunities(prev => prev.filter(c => c.id !== actionSheetCommunity.id));
       showToast(t('community.auto_fn_1726b6c', '已退出'));
    } else if (action === 'share') {
       showToast(t('community.auto_fn_352d1b75', '已分享邀请链接'));
    }
    setActionSheetCommunity(null);
  };

  return (
    <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black overflow-hidden relative">
      <header className="h-[56px] px-4 flex items-center justify-between sticky top-0 z-10 pt-safe bg-white dark:bg-[#1E1E1E] shrink-0 border-b border-black/5 dark:border-white/5">
        <IconButton
          icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
          className="bg-transparent w-10 h-10 -ml-2"
          onClick={() => navigate(-1)}
        />
        <h1 className="text-[17px] font-semibold text-text-main">{t('community.auto_2e5be31b', '我的圈子')}</h1>
        <div className="w-10" />
      </header>

      <div className="bg-white dark:bg-[#1E1E1E] shrink-0 border-b border-black/5 dark:border-white/5">
         <Tabs
            tabs={[
              { id: 'joined', name: '我加入的' },
              { id: 'created', name: '我创建的' },
            ]}
            activeTab={activeTab}
            onChange={setActiveTab}
            className="px-2"
         />
      </div>

      <div className="flex-1 overflow-y-auto w-full pb-safe bg-[#F2F2F7] dark:bg-black">
        {isLoading ? (
          <div className="flex flex-col h-40 items-center justify-center text-text-sub opacity-70">
            <div className="w-6 h-6 rounded-full border-2 border-text-sub border-t-transparent animate-spin mb-2"></div>
            <span className="text-[14px]">{t('community.auto_7f6f37e', '加载中...')}</span>
          </div>
        ) : communities.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-20 text-text-sub gap-3 opacity-80">
             <Compass className="w-12 h-12 opacity-30" />
             <span className="text-[14px]">{t('community.auto_30224566', '暂无圈子')}</span>
          </div>
        ) : (
           <div className="flex flex-col">
             {communities.map((community) => (
                <CommunityCard
                   key={community.id}
                   community={community}
                   onClick={() => {
                     if (isLongPressed) {
                        setIsLongPressed(false);
                        return;
                     }
                     navigate(`/community/${community.id}`);
                   }}
                   onLongPressProps={startLongPress(community)}
                   onMoreClick={(e) => {
                      e.stopPropagation();
                      setActionSheetCommunity(community);
                   }}
                />
             ))}
          </div>
        )}
      </div>

      {actionSheetCommunity && (
        <ActionSheet
          isOpen={true}
          title={`${actionSheetCommunity.name} - 操作`}
          options={
            activeTab === 'created' 
            ? [
                { label: '修改圈子信息', onClick: () => handleActionSheetSelect('edit') },
                { label: '分享圈子', onClick: () => handleActionSheetSelect('share') },
                { label: '删除圈子', danger: true, onClick: () => handleActionSheetSelect('delete') }
              ]
            : [
                { label: '分享圈子', onClick: () => handleActionSheetSelect('share') },
                { label: '退出圈子', danger: true, onClick: () => handleActionSheetSelect('leave') }
              ]
          }
          onClose={() => setActionSheetCommunity(null)}
        />
      )}
    </div>
  );
};



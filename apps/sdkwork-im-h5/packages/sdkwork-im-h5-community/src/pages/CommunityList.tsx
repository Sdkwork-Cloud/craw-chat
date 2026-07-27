import { useTranslation } from "react-i18next";
import React, { useState, useEffect, useRef } from "react";
import { useNavigate } from "react-router";
import { CommunityService } from "../services/CommunityService";
import { Community } from "../types";
import { cn, IconButton, showToast, Tabs, ActionSheet, useLongPress } from "@sdkwork/im-h5-commons";
import { ChevronLeft, Search, Users, MessageSquare, Compass, Check, X, Plus, MoreHorizontal } from "lucide-react";

import { PaymentSheet } from "../components/PaymentSheet";
import { SuccessModal } from "../components/SuccessModal";
import { CommunityCard } from "../components/CommunityCard";
import { CommunityHeader } from "../components/CommunityHeader";

export const CommunityList: React.FC = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const [communities, setCommunities] = useState<Community[]>([]);
  const [activeTab, setActiveTab] = useState<string>('recommend');
  const [isLoading, setIsLoading] = useState(true);
  const [isSearching, setIsSearching] = useState(false);
  const [searchText, setSearchText] = useState('');
  const [isPlusMenuOpen, setIsPlusMenuOpen] = useState(false);
  const [actionSheetCommunity, setActionSheetCommunity] = useState<Community | null>(null);
  const [isLongPressed, setIsLongPressed] = useState(false);

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    setIsLoading(true);
    try {
      const list = await CommunityService.getCommunities();
      setCommunities(list);
    } catch {
      showToast(t('community.auto_fn_afc180c', '获取圈子列表失败'));
    } finally {
      setIsLoading(false);
    }
  };

  const [isPaySheetOpen, setIsPaySheetOpen] = useState(false);
  const [selectedPaidCommunity, setSelectedPaidCommunity] = useState<Community | null>(null);
  const [showSuccessModal, setShowSuccessModal] = useState(false);
  const [selectedPayment, setSelectedPayment] = useState<'wechat'|'alipay'>('wechat');
  const [communityGroupsCache, setCommunityGroupsCache] = useState<any[]>([]);

  const handleJoin = async (e: React.MouseEvent, id: string) => {
    e.stopPropagation();
    const comm = communities.find(c => c.id === id);
    if (!comm) return;

    if (comm.isPaid) {
       setSelectedPaidCommunity(comm);
       setIsPaySheetOpen(true);
       return;
    }

    try {
      showToast(t('community.auto_fn_n630c7e9a', '加入中...'));
      await CommunityService.joinCommunity(id);
      
      const fetchedGroups = await CommunityService.getGroupsByCommunity(id);
      setCommunityGroupsCache(fetchedGroups);
      
      setCommunities(prev => prev.map(c => c.id === id ? { ...c, isJoined: true, memberCount: c.memberCount + 1 } : c));
      
      setSelectedPaidCommunity(comm);
      setShowSuccessModal(true);
    } catch {
      showToast(t('community.auto_fn_26cc0f99', '加入失败'));
    }
  };

  const handleConfirmPayJoin = async () => {
    if (!selectedPaidCommunity) return;
    try {
      showToast(t('community.auto_fn_1e02c86c', '支付处理中...'));
      await CommunityService.joinCommunity(selectedPaidCommunity.id);
      
      const fetchedGroups = await CommunityService.getGroupsByCommunity(selectedPaidCommunity.id);
      setCommunityGroupsCache(fetchedGroups);
      
      setCommunities(prev => prev.map(c => c.id === selectedPaidCommunity.id ? { ...c, isJoined: true, memberCount: c.memberCount + 1 } : c));
      setIsPaySheetOpen(false);
      setShowSuccessModal(true);
    } catch {
      showToast(t('community.auto_fn_2f078e83', '操作失败'));
      setIsPaySheetOpen(false);
    }
  };

  const longPressItemRef = useRef<Community | null>(null);
  const longPressHandlers = useLongPress({
    delay: 500,
    onLongPress: () => {
      const community = longPressItemRef.current;
      if (community) {
        setIsLongPressed(true);
        setActionSheetCommunity(community);
      }
    },
  });

  const startLongPress = (community: Community) => ({
    onPointerDown: () => {
      longPressItemRef.current = community;
      setIsLongPressed(false);
      longPressHandlers.onPointerDown();
    },
    onPointerUp: longPressHandlers.onPointerUp,
    onPointerLeave: () => {
      longPressHandlers.onPointerLeave();
      setIsLongPressed(false);
    },
    onContextMenu: (e: React.MouseEvent) => {
      e.preventDefault();
      longPressHandlers.onPointerUp();
      setIsLongPressed(true);
      setActionSheetCommunity(community);
    },
  });

  const handleActionSheetSelect = (action: string) => {
  if (!actionSheetCommunity) return;
    if (action === 'share') {
      showToast(t('community.auto_fn_16ae6d7', '已分享'));
    } else if (action === 'leave') {
      setCommunities(prev => prev.map(c => c.id === actionSheetCommunity.id ? { ...c, isJoined: false, memberCount: Math.max(0, c.memberCount - 1) } : c));
      showToast(t('community.auto_fn_1726b6c', '已退出'));
    }
    setActionSheetCommunity(null);
  };

  const filteredCommunities = (activeTab === 'joined' 
    ? communities.filter(c => c.isJoined)
    : activeTab === 'ai'
    ? communities.filter(c => c.tags.includes('AI') || c.tags.includes('大模型') || c.tags.includes('AIGC'))
    : activeTab === 'career'
    ? communities.filter(c => c.tags.some(t => ['职场', '搞钱', '出海', '开发', '产品', '商业'].includes(t)))
    : communities).filter(c => {
      if (!searchText.trim()) return true;
      const lowerSearch = searchText.toLowerCase();
      return c.name.toLowerCase().includes(lowerSearch) || 
             c.description.toLowerCase().includes(lowerSearch) ||
             c.tags.some(t => t.toLowerCase().includes(lowerSearch));
    });

  return (
    <div className="flex flex-col h-full bg-[#f2f2f7] dark:bg-black overflow-hidden relative">
      <CommunityHeader
        isSearching={isSearching}
        setIsSearching={setIsSearching}
        searchText={searchText}
        setSearchText={setSearchText}
        isPlusMenuOpen={isPlusMenuOpen}
        setIsPlusMenuOpen={setIsPlusMenuOpen}
      />

      <div className="bg-white dark:bg-[#1E1E1E] shrink-0 border-b border-black/5 dark:border-white/5">
         <Tabs
            tabs={[
              { id: 'recommend', name: '推荐' },
              { id: 'hot', name: '热门' },
              { id: 'ai', name: 'AI前沿' },
              { id: 'career', name: '搞钱出海' },
              { id: 'life', name: '数码生活' },
              { id: 'joined', name: '我加入的' },
              { id: 'news', name: '最新动态' },
              { id: 'tools', name: '实用工具' },
              { id: 'games', name: '游戏天地' }
            ]}
            activeTab={activeTab}
            onChange={setActiveTab}
            className="px-2"
            itemClassName="text-[15px] px-3 py-3 font-medium text-text-sub"
            activeItemClassName="text-[16px] font-bold text-blue-500"
         />
      </div>

      <div className="flex-1 overflow-y-auto pb-12 bg-[#F2F2F7] dark:bg-black">
        {isLoading ? (
          <div className="flex flex-col h-40 items-center justify-center text-text-sub opacity-70">
            <div className="w-6 h-6 rounded-full border-2 border-text-sub border-t-transparent animate-spin mb-2"></div>
            <span className="text-[14px]">{t('community.auto_7f6f37e', '加载中...')}</span>
          </div>
        ) : filteredCommunities.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-60 text-text-sub">
            <Compass className="w-16 h-16 opacity-30 mb-4" />
            <span className="text-[15px]">{t('community.auto_3983a45d', '空空如也')}</span>
          </div>
        ) : (
          <div className="flex flex-col">
            {filteredCommunities.map(community => (
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
                onJoinClick={(e) => handleJoin(e, community.id)}
              />
            ))}
          </div>
        )}
      </div>

      {isPaySheetOpen && selectedPaidCommunity && (
         <PaymentSheet
            communityName={selectedPaidCommunity.name}
            communityPrice={selectedPaidCommunity.price}
            communityCoverImage={selectedPaidCommunity.coverImage}
            onClose={() => {
              setIsPaySheetOpen(false);
              setSelectedPaidCommunity(null);
            }}
            onConfirm={handleConfirmPayJoin}
         />
      )}

      {showSuccessModal && selectedPaidCommunity && (
         <SuccessModal
            isPaid={selectedPaidCommunity.isPaid}
            communityName={selectedPaidCommunity.name}
            hasGroups={communityGroupsCache.length > 0}
            onClose={() => {
               setShowSuccessModal(false);
               setSelectedPaidCommunity(null);
            }}
            onEnterGroups={() => {
               setShowSuccessModal(false);
               navigate(`/community/${selectedPaidCommunity.id}/profile/tabs?tab=groups`);
            }}
            onEnterResources={() => {
               setShowSuccessModal(false);
               navigate(`/community/${selectedPaidCommunity.id}/profile/tabs?tab=resources`);
            }}
         />
      )}

      {actionSheetCommunity && (
        <ActionSheet
          isOpen={true}
          title={`${actionSheetCommunity.name} - 操作`}
          options={[
            { label: '分享圈子', onClick: () => handleActionSheetSelect('share') },
            ...(actionSheetCommunity.isJoined ? [{ label: '退出圈子', danger: true, onClick: () => handleActionSheetSelect('leave') }] : [])
          ]}
          onClose={() => setActionSheetCommunity(null)}
        />
      )}
    </div>
  );
};

import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router";
import { CommunityService } from "../services/CommunityService";
import { Community, Post, Resource, CommunityGroup } from "../types";
import { cn, IconButton, showToast, Tabs } from "@sdkwork/im-h5-commons";
import { ChevronLeft, Share2, Plus, Users, LayoutDashboard, FileText, Download, Check, Heart, MessageCircle, MessageSquare, QrCode, X, Edit2, Trash2, Newspaper, BookOpen, Github, Package, Settings2, Lock } from "lucide-react";

import { PostList } from "../components/PostList";
import { ResourceList } from "../components/ResourceList";
import { GroupList } from "../components/GroupList";
import { PaymentSheet } from "../components/PaymentSheet";
import { SuccessModal } from "../components/SuccessModal";

import { CommunityCover } from "../components/CommunityDetail/CommunityCover";
import { CommunityLockedView } from "../components/CommunityDetail/CommunityLockedView";
import { CommentInputOverlay } from "../components/CommunityDetail/CommentInputOverlay";
import { CommunityTabsContent } from "../components/CommunityDetail/CommunityTabsContent";

export const CommunityDetail: React.FC = () => {
  const { t } = useTranslation();
const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [community, setCommunity] = useState<Community | null>(null);
  const [posts, setPosts] = useState<Post[]>([]);
  const [resources, setResources] = useState<Resource[]>([]);
  const [groups, setGroups] = useState<CommunityGroup[]>([]);
  const [activeTab, setActiveTab] = useState<string>('feeds');
  const [isLoading, setIsLoading] = useState(true);
  const [activeCommentPostId, setActiveCommentPostId] = useState<string | null>(null);
  const [commentText, setCommentText] = useState("");

  useEffect(() => {
    loadData();
  }, [id]);

  const loadData = async () => {
    if (!id) return;
    setIsLoading(true);
    try {
      const [comm, fetchedPosts, fetchedResources, fetchedGroups] = await Promise.all([
        CommunityService.getCommunityById(id),
        CommunityService.getPostsByCommunity(id),
        CommunityService.getResourcesByCommunity(id),
        CommunityService.getGroupsByCommunity(id)
      ]);
      if (comm) setCommunity(comm);
      setPosts(fetchedPosts);
      setResources(fetchedResources);
      setGroups(fetchedGroups);
    } catch {
      showToast(t('community.auto_fn_n5e6a908e', '获取详情失败'));
    } finally {
      setIsLoading(false);
    }
  };

  const [isPaySheetOpen, setIsPaySheetOpen] = useState(false);
  const [showSuccessModal, setShowSuccessModal] = useState(false);
  const [selectedPayment, setSelectedPayment] = useState<'wechat'|'alipay'>('wechat');

  const handleJoin = async () => {
    if (!id || !community) return;
    if (community.isPaid) {
      if (!isPaySheetOpen) {
        setIsPaySheetOpen(true);
        return;
      }
    }
    
    try {
      showToast(community.isPaid ? "支付处理中..." : "加入中...");
      await CommunityService.joinCommunity(id);
      
      const [fetchedResources, fetchedGroups] = await Promise.all([
        CommunityService.getResourcesByCommunity(id),
        CommunityService.getGroupsByCommunity(id)
      ]);
      setResources(fetchedResources);
      setGroups(fetchedGroups);

      setCommunity({...community, isJoined: true, memberCount: community.memberCount + 1});
      setIsPaySheetOpen(false);

      setShowSuccessModal(true);
    } catch {
      showToast(t('community.auto_fn_2f078e83', '操作失败'));
      setIsPaySheetOpen(false);
    }
  };

  const handleLike = async (postId: string) => {
    if (!id) return;
    try {
      await CommunityService.toggleLikePost(id, postId);
      setPosts(prev => prev.map(p => {
        if (p.id === postId) {
          const isLiked = !p.isLiked;
          return {
            ...p,
            isLiked,
            likes: isLiked ? p.likes + 1 : Math.max(0, p.likes - 1)
          };
        }
        return p;
      }));
    } catch {
      showToast(t('community.auto_fn_2f078e83', '操作失败'));
    }
  };

  const handleComment = async () => {
    if (!commentText.trim() || !id || !activeCommentPostId) return;
    setIsLoading(true); // Can show loading indicator if needed
    try {
      await CommunityService.addComment(id, activeCommentPostId, commentText);
      setPosts(prev => prev.map(p => {
        if (p.id === activeCommentPostId) {
          return { 
            ...p, 
            comments: p.comments + 1,
            commentsList: [
              ...(p.commentsList || []),
              { id: `cmt_temp_${Date.now()}`, authorName: "我", content: commentText, createdAt: new Date().toISOString() }
            ]
          };
        }
        return p;
      }));
      setCommentText("");
      setActiveCommentPostId(null);
      showToast(t('community.auto_fn_41a16585', '评论成功'));
    } catch {
      showToast(t('community.auto_fn_41a08d0a', '评论失败'));
    } finally {
      setIsLoading(false);
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
      <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black">
         <header className="h-[56px] px-4 flex items-center justify-between sticky top-0 z-10 shrink-0 pt-safe bg-transparent">
            <IconButton icon={<ChevronLeft className="w-6 h-6 text-text-main" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
         </header>
         <div className="flex-1 flex items-center justify-center text-text-sub">{t('community.auto_7f6f37e', '加载中...')}</div>
      </div>
    );
  }

  if (!community) {
    return (
      <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black">
        <header className="h-[56px] px-4 flex items-center sticky top-0 z-10 pt-safe bg-bg-color">
            <IconButton icon={<ChevronLeft className="w-6 h-6 text-text-main" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
            <h1 className="text-[17px] font-semibold text-text-main ml-2">{t('community.auto_28f804e7', '圈子详情')}</h1>
        </header>
        <div className="flex-1 flex items-center justify-center text-text-sub">{t('community.auto_nadfe4ab', '圈子不存在')}</div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black relative">
       {/* Cover and Header */}
       <CommunityCover community={community} />

       {/* Content Area */}
       <div className="flex-1 flex flex-col bg-[#F2F2F7] dark:bg-black -mt-2 rounded-t-[24px] relative z-20 overflow-hidden shadow-[0_-4px_16px_rgba(0,0,0,0.1)]">
          {/* Join action bar if not joined */}
          {!community.isJoined && !community.isPaid && (
             <div className="px-4 py-4 bg-white dark:bg-[#1C1C1E] flex items-center justify-between shadow-[0_2px_10px_rgba(0,0,0,0.02)] z-10 shrink-0">
                <span className="text-[14px] text-text-sub pl-1">{t('community.auto_n2a576b36', '加入圈子，参与讨论并获取更多资源')}</span>
                <button 
                  className="px-5 py-2 bg-blue-500 text-white rounded-full font-medium text-[14px] shadow-md shadow-blue-500/20 active:scale-95 transition-transform"
                  onClick={handleJoin}
                >{t('community.auto_27118551', '免费加入')}</button>
             </div>
          )}

          {!community.isJoined && community.isPaid ? (
             <CommunityLockedView community={community} onJoin={handleJoin} />
          ) : (
            <>
              {/* Sticky Tabs */}
              <div className="bg-white dark:bg-[#1C1C1E] flex items-center shrink-0 border-b border-black/5 dark:border-white/5">
                <Tabs
                   tabs={[
                     { id: 'feeds', name: '动态' },
                     { id: 'resources', name: '资源' },
                     { id: 'groups', name: '群组' },
                     { id: 'news', name: '新闻' },
                     { id: 'docs', name: '文档' },
                     { id: 'repos', name: '开源' },
                     { id: 'software', name: '软件' }
                   ].filter(tab => {
                     const allowed = community.tabs || ['feeds', 'resources', 'groups'];
                     return allowed.includes(tab.id);
                   })}
                   activeTab={activeTab}
                   onChange={setActiveTab}
                   className="px-2"
                   itemClassName="text-[15px] px-3 py-3 font-medium text-text-sub"
                   activeItemClassName="text-[15px] font-bold text-blue-500"
                />
              </div>

              <CommunityTabsContent
                activeTab={activeTab}
                posts={posts}
                resources={resources}
                groups={groups}
                community={community}
                platformNameMap={platformNameMap}
                onLike={handleLike}
                onCommentClick={(postId) => {
                  setActiveCommentPostId(postId);
                  setTimeout(() => {
                    document.getElementById('commentInput')?.focus();
                  }, 100);
                }}
              />
          </>
          )}
       </div>

       {/* Sub-components below Content Area */}

       {/* Floating Action Button (if joined) */}
       {community.isJoined && activeTab === 'feeds' && (
         <button 
           className="absolute right-5 bottom-[4.5rem] w-14 h-14 bg-gradient-to-br from-blue-500 to-indigo-500 rounded-full shadow-lg shadow-blue-500/30 flex items-center justify-center text-white active:scale-95 transition-transform z-40"
           onClick={() => navigate(`/community/${community.id}/post`)}
         >
           <Plus className="w-7 h-7" />
         </button>
       )}
       {/* Comment Input Overlay */}
       <CommentInputOverlay
         activeCommentPostId={activeCommentPostId}
         commentText={commentText}
         setCommentText={setCommentText}
         onClose={() => setActiveCommentPostId(null)}
         onSend={handleComment}
       />

       {/* Pay Sheet Overlay */}
       {isPaySheetOpen && (
          <PaymentSheet
             communityName={community.name}
             communityPrice={community.price}
             communityCoverImage={community.coverImage}
             onClose={() => setIsPaySheetOpen(false)}
             onConfirm={handleJoin}
          />
       )}

       {/* Success Modal */}
       {showSuccessModal && (
          <SuccessModal
             isPaid={community.isPaid}
             communityName={community.name}
             hasGroups={groups.length > 0}
             onClose={() => setShowSuccessModal(false)}
             onEnterGroups={() => {
                setShowSuccessModal(false);
                setActiveTab('groups');
             }}
             onEnterResources={() => {
                setShowSuccessModal(false);
                setActiveTab('resources');
             }}
          />
       )}

     </div>
  );
};

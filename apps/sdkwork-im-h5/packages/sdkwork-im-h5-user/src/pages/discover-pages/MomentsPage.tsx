import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { Camera, Heart, MessageCircle, Send, MoreHorizontal, X, Image as ImageIcon, Trash2 } from "lucide-react";
import { MomentService, type Moment } from "../../services/MomentService";
import { Avatar, cn, showToast, PageLayout } from "@sdkwork/im-h5-commons";
import { motion, AnimatePresence } from "motion/react";
import { MomentItemCard } from "../../components/moments/MomentItemCard";
import { MomentPublishModal } from "../../components/moments/MomentPublishModal";
import { MomentMediaPreview } from "../../components/moments/MomentMediaPreview";
import { MomentsHeaderCover } from "../../components/moments/MomentsHeaderCover";

export const MomentsPage = () => {
  const { t } = useTranslation();
const [moments, setMoments] = useState<Moment[]>([]);
  const [activeCommentId, setActiveCommentId] = useState<string | null>(null);
  const [activePopoverId, setActivePopoverId] = useState<string | null>(null);
  const [commentText, setCommentText] = useState("");
  const [isLoading, setIsLoading] = useState(true);

  // Publish state
  const [showPublish, setShowPublish] = useState(false);
  const [publishContent, setPublishContent] = useState("");
  const [publishImages, setPublishImages] = useState<string[]>([]);

  // Preview state
  const [previewState, setPreviewState] = useState<
    { type: 'images'; images: string[]; index: number } | 
    { type: 'video'; url: string } | null
  >(null);

  const swipeConfidenceThreshold = 10000;
  const swipePower = (offset: number, velocity: number) => {
  return Math.abs(offset) * velocity;
  };

  const handleSwipe = (direction: 'left' | 'right') => {
  if (previewState?.type !== 'images') return;
    if (direction === 'left' && previewState.index < previewState.images.length - 1) {
      setPreviewState({ ...previewState, index: previewState.index + 1 });
    } else if (direction === 'right' && previewState.index > 0) {
      setPreviewState({ ...previewState, index: previewState.index - 1 });
    }
  };

  const loadMoments = async () => {
    setIsLoading(true);
    const data = await MomentService.getMoments();
    setMoments(data);
    setIsLoading(false);
  };

  useEffect(() => {
    loadMoments();
  }, []);

  const handleLike = async (id: string, e?: React.MouseEvent) => {
    if (e) e.stopPropagation();
    await MomentService.toggleLike(id, "u1");
    setActivePopoverId(null);
    loadMoments();
  };

  const submitComment = async (id: string) => {
    if (!commentText.trim()) return;
    await MomentService.addComment(id, "Alex Chen", commentText);
    setCommentText("");
    setActiveCommentId(null);
    loadMoments();
  };

  const handleDelete = async (id: string, e?: React.MouseEvent) => {
    if (e) e.stopPropagation();
    await MomentService.deleteMoment(id);
    showToast(t('user.auto_fn_16b31b6', '已删除'));
    loadMoments();
  };

  const openComment = (id: string, e: React.MouseEvent) => {
  e.stopPropagation();
    setActivePopoverId(null);
    setCommentText(""); // reset when normal comment
    setActiveCommentId(id);
  };

  const openReply = (momentId: string, commentAuthor: string, e: React.MouseEvent) => {
  e.stopPropagation();
    setActivePopoverId(null);
    setCommentText(`回复 ${commentAuthor}: `);
    setActiveCommentId(momentId);
  };

  const togglePopover = (id: string, e: React.MouseEvent) => {
  e.stopPropagation();
    if (activePopoverId === id) {
      setActivePopoverId(null);
    } else {
      setActivePopoverId(id);
    }
  };

  // Close popover when clicking anywhere else
  useEffect(() => {
    const closeAll = () => setActivePopoverId(null);
    window.addEventListener("click", closeAll);
    return () => window.removeEventListener("click", closeAll);
  }, []);

  const formatTime = (ts: number) => {
  const diff = Date.now() - ts;
    const mins = Math.floor(diff / 60000);
    if (mins < 1) return "刚刚";
    if (mins < 60) return `${mins}分钟前`;
    const hours = Math.floor(mins / 60);
    if (hours < 24) return `${hours}小时前`;
    return new Date(ts).toLocaleDateString();
  };

  const submitPublish = async () => {
    if (!publishContent.trim() && publishImages.length === 0) return;
    await MomentService.addMoment(publishContent, publishImages);
    setPublishContent("");
    setPublishImages([]);
    setShowPublish(false);
    showToast(t('user.auto_fn_2786ea61', '发布成功'));
    loadMoments();
  };

  const addFakeImage = () => {
  const seed = Math.random().toString(36).substring(7);
    setPublishImages([...publishImages, `https://picsum.photos/seed/${seed}/400/400`]);
  };

  return (
    <div className="flex flex-col h-full bg-bg-color relative overflow-hidden">
      <PageLayout 
        bgClass="bg-chat-other-bg"
        rightElement={
          <button 
            onClick={() => setShowPublish(true)}
            className="w-10 h-10 flex items-center justify-center active:bg-gray-100 dark:active:bg-white/10 rounded-full transition-colors"
          >
            <Camera className="w-5 h-5 text-text-main" strokeWidth={2} />
          </button>
        }
      >
        <div className="flex flex-col flex-1 w-full min-h-full pb-safe">
          <MomentsHeaderCover
            name="Alex Chen"
            avatarUrl="https://picsum.photos/seed/alex/200/200"
            coverUrl="https://picsum.photos/seed/cover/800/600"
          />

          {/* Moments List */}
          <div className="pb-safe w-full bg-chat-other-bg min-h-screen">
            {isLoading ? (
              <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
                <p className="text-[14px]">{t('user.auto_7f6f37e', '加载中...')}</p>
              </div>
            ) : moments.length === 0 ? (
              <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                <Camera className="w-12 h-12 mb-3 stroke-current opacity-40 animate-pulse" />
                <p className="text-[14px]">{t('user.auto_6b17c5ac', '去分享你的生活点滴吧')}</p>
              </div>
            ) : (
              moments.map((moment) => (
                <MomentItemCard
                  key={moment.id}
                  moment={moment}
                  activePopoverId={activePopoverId}
                  activeCommentId={activeCommentId}
                  commentText={commentText}
                  setCommentText={setCommentText}
                  formatTime={formatTime}
                  setPreviewState={setPreviewState}
                  handleDelete={handleDelete}
                  togglePopover={togglePopover}
                  handleLike={handleLike}
                  openComment={openComment}
                  openReply={openReply}
                  submitComment={submitComment}
                />
              ))
            )}
          </div>
        </div>
      </PageLayout>

      {/* Fullscreen Publish Modal */}
      <AnimatePresence>
        {showPublish && (
          <MomentPublishModal
            onClose={() => setShowPublish(false)}
            onSubmit={submitPublish}
            content={publishContent}
            setContent={setPublishContent}
            images={publishImages}
            setImages={setPublishImages}
            addFakeImage={addFakeImage}
          />
        )}
      </AnimatePresence>
      {/* Image / Video Preview Modal */}
      <AnimatePresence>
        {previewState && (
          <MomentMediaPreview
            previewState={previewState}
            onClose={() => setPreviewState(null)}
            onSwipe={handleSwipe}
          />
        )}
      </AnimatePresence>
    </div>
  );
};

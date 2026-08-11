import { useTranslation } from "react-i18next";
import React, { useState, useEffect, useCallback, useRef } from "react";
import { Camera } from "lucide-react";
import { MomentService } from "../services/MomentService";
import type { Moment, MomentCircle, MomentComment, MomentPreviewState } from "../types";
import { showToast, PageLayout } from "@sdkwork/im-h5-commons";
import { useAppStore } from "@sdkwork/im-h5-core";
import { motion, AnimatePresence } from "motion/react";
import { MomentItemCard } from "../components/moments/MomentItemCard";
import { MomentPublishModal } from "../components/moments/MomentPublishModal";
import { MomentMediaPreview } from "../components/moments/MomentMediaPreview";
import { MomentsHeaderCover } from "../components/moments/MomentsHeaderCover";

const swipeConfidenceThreshold = 10000;
const swipePower = (offset: number, velocity: number) => Math.abs(offset) * velocity;

export const MomentsPage = () => {
  const { t, i18n } = useTranslation();
  const currentUser = useAppStore((state) => state.currentUser);

  const [moments, setMoments] = useState<Moment[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [loadFailed, setLoadFailed] = useState(false);
  const [page, setPage] = useState(1);
  const [hasMore, setHasMore] = useState(true);
  const [isLoadingMore, setIsLoadingMore] = useState(false);
  const loadMoreSentinelRef = useRef<HTMLDivElement | null>(null);
  const [activeCommentId, setActiveCommentId] = useState<string | null>(null);
  const [activePopoverId, setActivePopoverId] = useState<string | null>(null);
  const [commentText, setCommentText] = useState("");
  const [previewState, setPreviewState] = useState<MomentPreviewState | null>(null);

  // Publish state
  const [showPublish, setShowPublish] = useState(false);
  const [publishContent, setPublishContent] = useState("");
  const [circles, setCircles] = useState<MomentCircle[]>([]);
  const [circlesLoading, setCirclesLoading] = useState(false);
  const [selectedCircleId, setSelectedCircleId] = useState("");

  const handleSwipe = (direction: 'left' | 'right') => {
    if (previewState?.type !== 'images') return;
    if (direction === 'left' && previewState.index < previewState.images.length - 1) {
      setPreviewState({ ...previewState, index: previewState.index + 1 });
    } else if (direction === 'right' && previewState.index > 0) {
      setPreviewState({ ...previewState, index: previewState.index - 1 });
    }
  };

  const loadMoments = useCallback(
    async (targetPage: number, append: boolean) => {
      if (!append) {
        setIsLoading(true);
        setLoadFailed(false);
      }
      try {
        const { moments: nextMoments, hasMore: nextHasMore } = await MomentService.getFeed(
          targetPage,
        );
        setMoments((prev) => (append ? [...prev, ...nextMoments] : nextMoments));
        setHasMore(nextHasMore);
        setPage(targetPage);
      } catch {
        if (!append) {
          setMoments([]);
          setLoadFailed(true);
        }
      } finally {
        if (!append) {
          setIsLoading(false);
        }
      }
    },
    [],
  );

  useEffect(() => {
    let cancelled = false;
    setIsLoading(true);
    MomentService.getFeed(1)
      .then(({ moments: nextMoments, hasMore: nextHasMore }) => {
        if (cancelled) return;
        setMoments(nextMoments);
        setHasMore(nextHasMore);
        setPage(1);
      })
      .catch(() => {
        if (!cancelled) {
          setMoments([]);
          setLoadFailed(true);
        }
      })
      .finally(() => {
        if (!cancelled) setIsLoading(false);
      });
    return () => {
      cancelled = true;
    };
  }, []);

  const loadMore = useCallback(async () => {
    if (isLoading || isLoadingMore || !hasMore) return;
    setIsLoadingMore(true);
    try {
      await loadMoments(page + 1, true);
    } finally {
      setIsLoadingMore(false);
    }
  }, [isLoading, isLoadingMore, hasMore, page, loadMoments]);

  // Keep the latest loadMore in a ref so the observer effect never needs to
  // re-subscribe when the feed grows.
  const loadMoreRef = useRef(loadMore);
  loadMoreRef.current = loadMore;

  useEffect(() => {
    const sentinel = loadMoreSentinelRef.current;
    if (!sentinel || !hasMore || isLoading) return;
    const observer = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting) {
          loadMoreRef.current();
        }
      },
      { rootMargin: "160px" },
    );
    observer.observe(sentinel);
    return () => observer.disconnect();
  }, [hasMore, isLoading]);

  const handleLike = async (id: string, e?: React.MouseEvent) => {
    if (e) e.stopPropagation();
    setActivePopoverId(null);
    try {
      const { isLiked, likeCount } = await MomentService.toggleLike(id);
      setMoments((prev) =>
        prev.map((moment) =>
          moment.id === id ? { ...moment, isLiked, likeCount } : moment,
        ),
      );
    } catch {
      showToast(t('moments.like_failed', '操作失败，请重试'));
    }
  };

  const loadCommentsIfNeeded = async (moment: Moment) => {
    if (moment.comments.length > 0 || moment.commentCount === 0) return;
    try {
      const comments = await MomentService.getComments(moment.id);
      setMoments((prev) =>
        prev.map((item) => {
          if (item.id !== moment.id) return item;
          // Merge server comments with any comment posted locally while the
          // fetch was in flight, so a fast submit is never overwritten.
          const merged = new Map<string, MomentComment>();
          for (const comment of comments) merged.set(comment.id, comment);
          for (const comment of item.comments) merged.set(comment.id, comment);
          return { ...item, comments: [...merged.values()] };
        }),
      );
    } catch {
      // Keep the inline comment input usable even when the list fetch fails.
    }
  };

  const openComment = (id: string, e: React.MouseEvent) => {
    e.stopPropagation();
    setActivePopoverId(null);
    setCommentText("");
    setActiveCommentId(id);
    const moment = moments.find((item) => item.id === id);
    if (moment) {
      loadCommentsIfNeeded(moment);
    }
  };

  const openReply = (momentId: string, commentAuthor: string, e: React.MouseEvent) => {
    e.stopPropagation();
    setActivePopoverId(null);
    setCommentText(t('moments.reply_prefix', { author: commentAuthor, defaultValue: `回复 ${commentAuthor}: ` }));
    setActiveCommentId(momentId);
    const moment = moments.find((item) => item.id === momentId);
    if (moment) {
      loadCommentsIfNeeded(moment);
    }
  };

  const submitComment = async (id: string) => {
    if (!commentText.trim()) return;
    try {
      const comment = await MomentService.addComment(id, commentText);
      setCommentText("");
      setActiveCommentId(null);
      setMoments((prev) =>
        prev.map((moment) =>
          moment.id === id
            ? {
                ...moment,
                comments: [...moment.comments, comment],
                commentCount: moment.commentCount + 1,
              }
            : moment,
        ),
      );
    } catch {
      showToast(t('moments.comment_failed', '评论失败，请重试'));
    }
  };

  const handleDelete = async (id: string, e?: React.MouseEvent) => {
    if (e) e.stopPropagation();
    try {
      await MomentService.deleteMoment(id);
      showToast(t('moments.delete_success', '已删除'));
      setMoments((prev) => prev.filter((moment) => moment.id !== id));
    } catch {
      showToast(t('moments.delete_failed', '删除失败，请重试'));
    }
  };

  const togglePopover = (id: string, e: React.MouseEvent) => {
    e.stopPropagation();
    setActivePopoverId((prev) => (prev === id ? null : id));
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
    if (mins < 1) return t('moments.time.just_now', '刚刚');
    if (mins < 60) return t('moments.time.minutes_ago', '{{count}}分钟前', { count: mins });
    const hours = Math.floor(mins / 60);
    if (hours < 24) return t('moments.time.hours_ago', '{{count}}小时前', { count: hours });
    return new Date(ts).toLocaleDateString(i18n.language);
  };

  const openPublish = () => {
    setShowPublish(true);
    setCirclesLoading(true);
    MomentService.getCircles()
      .then((nextCircles) => {
        setCircles(nextCircles);
        if (nextCircles.length > 0 && !selectedCircleId) {
          setSelectedCircleId(nextCircles[0].id);
        }
      })
      .catch(() => setCircles([]))
      .finally(() => setCirclesLoading(false));
  };

  const submitPublish = async () => {
    if (!publishContent.trim() || !selectedCircleId) {
      showToast(t('moments.select_circle_required', '请选择要发布的圈子'));
      return;
    }
    try {
      const published = await MomentService.publish({
        categoryId: selectedCircleId,
        content: publishContent,
      });
      setPublishContent("");
      setSelectedCircleId("");
      setShowPublish(false);
      showToast(t('moments.publish_success', '发布成功'));
      // Insert the fresh moment at the top instead of reloading the whole
      // feed, so the list does not flash a full-screen loading state.
      setMoments((prev) => [published, ...prev]);
    } catch {
      showToast(t('moments.publish_failed', '发布失败，请重试'));
    }
  };

  const likedByLabel = (moment: Moment) => {
    if (moment.isLiked) {
      const otherCount = Math.max(moment.likeCount - 1, 0);
      if (otherCount === 0) {
        return currentUser?.name ?? t('moments.liked_by_me', '我赞过');
      }
      return currentUser?.name
        ? `${currentUser.name}，${t('moments.liked_by', { count: otherCount })}`
        : t('moments.liked_by', { count: otherCount });
    }
    return t('moments.liked_by_count', '{{count}} 人赞过', { count: moment.likeCount });
  };

  return (
    <div className="flex flex-col h-full bg-bg-color relative overflow-hidden">
      <PageLayout
        bgClass="bg-chat-other-bg"
        rightElement={
          <button
            onClick={openPublish}
            className="w-10 h-10 flex items-center justify-center active:bg-gray-100 dark:active:bg-white/10 rounded-full transition-colors"
          >
            <Camera className="w-5 h-5 text-text-main" strokeWidth={2} />
          </button>
        }
      >
        <div className="flex flex-col flex-1 w-full min-h-full pb-safe">
          <MomentsHeaderCover
            name={currentUser?.name ?? t('moments.title', '朋友圈')}
            avatarUrl={currentUser?.avatar}
          />

          {/* Moments List */}
          <div className="pb-safe w-full bg-chat-other-bg min-h-screen">
            {isLoading ? (
              <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
                <p className="text-[14px]">{t('moments.loading', '加载中...')}</p>
              </div>
            ) : loadFailed ? (
              <button
                onClick={() => loadMoments(1, false)}
                className="w-full flex flex-col items-center justify-center py-20 text-text-sub opacity-70 active:opacity-50"
              >
                <Camera className="w-12 h-12 mb-3 stroke-current opacity-40" />
                <p className="text-[14px]">{t('moments.load_failed', '加载失败，点击重试')}</p>
              </button>
            ) : moments.length === 0 ? (
              <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                <Camera className="w-12 h-12 mb-3 stroke-current opacity-40 animate-pulse" />
                <p className="text-[14px]">{t('moments.empty', '去分享你的生活点滴吧')}</p>
              </div>
            ) : (
              <>
                {moments.map((moment) => (
                  <MomentItemCard
                    key={moment.id}
                    moment={moment}
                    currentUserId={currentUser?.id}
                    likedByLabel={likedByLabel(moment)}
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
                ))}
                <div
                  ref={loadMoreSentinelRef}
                  className="flex items-center justify-center py-4 text-text-sub opacity-60 text-[13px]"
                >
                  {isLoadingMore
                    ? t('moments.loading_more', '加载更多...')
                    : hasMore
                      ? t('moments.load_more', '上拉加载更多')
                      : t('moments.no_more', '没有更多了')}
                </div>
              </>
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
            circles={circles}
            circlesLoading={circlesLoading}
            selectedCircleId={selectedCircleId}
            setSelectedCircleId={setSelectedCircleId}
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

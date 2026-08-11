import { useTranslation } from "react-i18next";
import React from "react";
import { Image as ImageIcon } from "lucide-react";
import { motion } from "motion/react";
import { useNavigate } from "react-router";
import type { MomentCircle } from "../../types";

interface MomentPublishModalProps {
  onClose: () => void;
  onSubmit: () => void;
  content: string;
  setContent: (content: string) => void;
  circles: MomentCircle[];
  circlesLoading: boolean;
  selectedCircleId: string;
  setSelectedCircleId: (circleId: string) => void;
}

export const MomentPublishModal: React.FC<MomentPublishModalProps> = ({
  onClose,
  onSubmit,
  content,
  setContent,
  circles,
  circlesLoading,
  selectedCircleId,
  setSelectedCircleId,
}) => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const canSubmit = content.trim().length > 0 && selectedCircleId.length > 0;

  return (
    <motion.div
      initial={{ opacity: 0, y: "100%" }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: "100%" }}
      transition={{ type: "spring", damping: 25, stiffness: 200 }}
      className="fixed inset-0 z-50 bg-bg-color flex flex-col pt-safe"
    >
      {/* Header */}
      <div className="h-[56px] flex items-center justify-between px-4 bg-chat-other-bg shrink-0 shadow-sm relative z-10 border-b border-black/5 dark:border-white/5">
        <button
          onClick={onClose}
          className="text-[16px] text-text-main font-medium active:opacity-50"
        >{t('moments.cancel', 'Cancel')}</button>
        <button
          onClick={onSubmit}
          disabled={!canSubmit}
          className="bg-[#07C160] disabled:opacity-50 disabled:bg-[#07C160]/70 text-white text-[14px] font-medium px-4 py-1.5 rounded active:bg-[#06ad56] transition-colors"
        >{t('moments.publish', 'Publish')}</button>
      </div>

      <div className="flex-1 bg-chat-other-bg p-4 overflow-y-auto w-full">
        <textarea
          className="w-full h-32 bg-transparent outline-none text-[16px] text-text-main resize-none placeholder:text-text-sub"
          placeholder={t('moments.placeholder', 'Share a thought...')}
          value={content}
          onChange={(e) => setContent(e.target.value)}
          autoFocus
        />

        {/* Circle picker (publishing requires a target circle) */}
        <div className="mt-4">
          <div className="text-[13px] text-text-sub mb-2">{t('moments.select_circle', 'Post to')}</div>
          {circlesLoading ? (
            <div className="text-[13px] text-text-sub opacity-70">{t('moments.loading', 'Loading...')}</div>
          ) : circles.length === 0 ? (
            <div className="text-[13px] text-text-sub opacity-70">
              {t('moments.no_circles', 'No circles available yet')}
              <button
                className="text-[#07C160] font-medium ml-2 active:opacity-50"
                onClick={() => {
                  onClose();
                  navigate("/community");
                }}
              >
                {t('moments.go_to_circles', 'Go to Circles')}
              </button>
            </div>
          ) : (
            <div className="flex gap-2 overflow-x-auto pb-1">
              {circles.map((circle) => (
                <button
                  key={circle.id}
                  onClick={() => setSelectedCircleId(circle.id)}
                  className={[
                    "flex items-center gap-2 rounded-lg border px-3 py-2 shrink-0 transition-colors",
                    selectedCircleId === circle.id
                      ? "border-[#07C160] bg-[#07C160]/10"
                      : "border-black/10 dark:border-white/10 bg-hover-bg",
                  ].join(" ")}
                >
                  {circle.avatar ? (
                    <img src={circle.avatar} alt={circle.name} className="w-5 h-5 rounded" />
                  ) : null}
                  <span className="text-[13px] text-text-main whitespace-nowrap">{circle.name}</span>
                </button>
              ))}
            </div>
          )}
        </div>

        {/* Media upload is a deferred capability: no transport exists yet. */}
        <div className="mt-4">
          <button
            disabled
            className="aspect-square w-20 bg-hover-bg flex flex-col items-center justify-center rounded active:bg-active-bg dark:active:bg-white/10 transition-colors opacity-60"
          >
            <ImageIcon className="w-7 h-7 text-text-sub opacity-50" />
          </button>
          <div className="text-[12px] text-text-sub opacity-60 mt-2">{t('moments.image_coming_soon', 'Image upload coming soon')}</div>
        </div>
      </div>
    </motion.div>
  );
};

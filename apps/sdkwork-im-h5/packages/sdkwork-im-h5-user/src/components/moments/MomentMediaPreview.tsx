import { useTranslation } from "react-i18next";
import React from "react";
import { motion } from "motion/react";

interface MediaPreviewState {
  type: 'images' | 'video';
  images?: string[];
  index?: number;
  url?: string;
}

interface MomentMediaPreviewProps {
  previewState: MediaPreviewState;
  onClose: () => void;
  onSwipe: (direction: 'left' | 'right') => void;
}

export const MomentMediaPreview: React.FC<MomentMediaPreviewProps> = ({
  previewState,
  onClose,
  onSwipe
}) => {
  const { t } = useTranslation();
const swipeConfidenceThreshold = 10000;
  const swipePower = (offset: number, velocity: number) => Math.abs(offset) * velocity;

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      transition={{ duration: 0.2 }}
      className="fixed inset-0 z-[60] bg-black flex flex-col items-center justify-center touch-none"
      onClick={onClose}
    >
      {previewState.type === 'images' && previewState.images && previewState.images.length > 1 && (
        <div className="absolute top-safe right-4 z-10 text-white p-2 text-[14px]">
          {(previewState.index ?? 0) + 1} / {previewState.images.length}
        </div>
      )}
      
      {previewState.type === 'images' && previewState.images ? (
        <motion.img
          key={previewState.index}
          initial={{ opacity: 0, scale: 0.95 }}
          animate={{ opacity: 1, scale: 1 }}
          exit={{ opacity: 0, scale: 0.95 }}
          transition={{ duration: 0.2, type: "spring", damping: 25 }}
          src={previewState.images[previewState.index ?? 0]}
          alt="Preview"
          className="w-full max-h-[100dvh] object-contain" 
          drag="x"
          dragConstraints={{ left: 0, right: 0 }}
          dragElastic={0.2}
          onDragEnd={(e, { offset, velocity }) => {
            const swipe = swipePower(offset.x, velocity.x);
            if (swipe < -swipeConfidenceThreshold) {
              onSwipe('left');
            } else if (swipe > swipeConfidenceThreshold) {
              onSwipe('right');
            }
          }}
          onClick={(e) => { e.stopPropagation(); onClose(); }}
        />
      ) : previewState.type === 'video' && previewState.url ? (
        <motion.video
          initial={{ opacity: 0, scale: 0.95 }}
          animate={{ opacity: 1, scale: 1 }}
          exit={{ opacity: 0, scale: 0.95 }}
          transition={{ duration: 0.2, type: "spring", damping: 25 }}
          src={previewState.url}
          className="w-full max-h-[100dvh] object-contain"
          controls
          autoPlay
          playsInline
          onClick={(e) => e.stopPropagation()}
        />
      ) : null}
    </motion.div>
  );
};

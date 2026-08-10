import { useTranslation } from "react-i18next";
import React from "react";
import { motion, AnimatePresence } from "motion/react";
import { X, Download } from "lucide-react";

export interface MediaPreviewProps {
  media: {
    type: "image" | "video" | "pdf" | string;
    url: string;
    name?: string;
  } | null;
  onClose: () => void;
}

export const MediaPreview: React.FC<MediaPreviewProps> = ({
  media,
  onClose,
}) => {
  const { t } = useTranslation();
if (!media) return null;

  return (
    <AnimatePresence>
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
        transition={{ duration: 0.2 }}
        className="fixed inset-0 z-[500] bg-black/95 flex flex-col items-center justify-center backdrop-blur-sm"
      >
        {/* Header Controls */}
        <div className="absolute top-0 left-0 right-0 h-14 bg-gradient-to-b from-black/60 to-transparent flex items-center justify-between px-4 pt-safe z-50">
          <div
            className="w-10 h-10 flex items-center justify-center cursor-pointer text-white/80 active:text-white"
            onClick={onClose}
          >
            <X className="w-6 h-6" />
          </div>
          <span className="text-white text-[15px] font-medium truncate max-w-[200px]">
            {media.name}
          </span>
          <div className="w-10 h-10 flex items-center justify-center cursor-pointer text-white/80 active:text-white">
            <Download className="w-5 h-5" />
          </div>
        </div>

        {/* Media Content */}
        <div
          className="flex-1 flex items-center justify-center absolute inset-0 cursor-default px-4 pt-16 pb-[calc(env(safe-area-inset-bottom,0px)+1rem)]"
          onClick={onClose}
        >
          {media.type === "image" && (
            <motion.img
              initial={{ scale: 0.9, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              exit={{ scale: 0.9, opacity: 0 }}
              transition={{ type: "spring", damping: 25, stiffness: 300 }}
              src={media.url}
              alt="Preview"
              className="max-w-full max-h-full object-contain select-none"
              onClick={(e) => e.stopPropagation()}
            />
          )}
          {media.type === "video" && (
            <motion.div
              initial={{ scale: 0.95, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              exit={{ scale: 0.95, opacity: 0 }}
              transition={{ type: "spring", damping: 25, stiffness: 300 }}
              className="w-full h-full flex items-center justify-center"
              onClick={(e) => e.stopPropagation()}
            >
              <video
                src={media.url}
                controls
                autoPlay
                playsInline
                className="max-w-full max-h-full outline-none"
              />
            </motion.div>
          )}
          {media.type !== "image" && media.type !== "video" && (
            <div
              className="flex flex-col items-center justify-center text-white/60"
              onClick={(e) => e.stopPropagation()}
            >
              <span className="text-[14px]">{t('commons.auto_n59aa5386', '不支持该格式预览，请下载后查看')}</span>
            </div>
          )}
        </div>
      </motion.div>
    </AnimatePresence>
  );
};

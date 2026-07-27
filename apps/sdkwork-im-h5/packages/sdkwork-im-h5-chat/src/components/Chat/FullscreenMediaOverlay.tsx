import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { motion, AnimatePresence } from "motion/react";
import { X } from "lucide-react";

interface FullscreenMediaOverlayProps {
  media: {
    type: "image" | "video";
    url: string;
  } | null;
  mediaList?: {
    type: "image" | "video";
    url: string;
  }[];
  onClose: () => void;
}

export const FullscreenMediaOverlay: React.FC<FullscreenMediaOverlayProps> = ({
  media,
  mediaList = [],
  onClose,
}) => {
  const { t } = useTranslation();
const allMedia = mediaList.length > 0 ? mediaList : media ? [media] : [];
  const initialIdx = media ? allMedia.findIndex(m => m.url === media.url) : 0;
  const [currentIndex, setCurrentIndex] = useState(initialIdx >= 0 ? initialIdx : 0);

  useEffect(() => {
    if (media) {
      const idx = mediaList.findIndex((m) => m.url === media.url);
      setCurrentIndex(idx >= 0 ? idx : 0);
    }
  }, [media, mediaList]);

  const [direction, setDirection] = useState(0);

  const handleDragEnd = (e: any, { offset, velocity }: any) => {
  const swipe = offset.x;

    if (swipe < -50 && currentIndex < allMedia.length - 1) {
      setDirection(1);
      setCurrentIndex((prev) => prev + 1);
    } else if (swipe > 50 && currentIndex > 0) {
      setDirection(-1);
      setCurrentIndex((prev) => prev - 1);
    }
  };

  return (
    <AnimatePresence>
      {media && (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          className="fixed inset-0 z-[200] bg-black flex flex-col cursor-pointer overflow-hidden"
          onClick={onClose}
        >
          <div className="h-14 flex items-center justify-end px-4 pt-safe safe-area-top absolute top-0 inset-x-0 z-20 bg-gradient-to-b from-black/50 to-transparent pointer-events-none">
            <button
              className="text-white p-2 pointer-events-auto"
              onClick={(e) => {
                e.stopPropagation();
                onClose();
              }}
            >
              <X className="w-8 h-8 drop-shadow-md" />
            </button>
          </div>
          
          <div
            className="flex-1 flex items-center w-full h-full absolute inset-0 cursor-default bg-black"
            onClick={(e) => e.stopPropagation()}
          >
            <AnimatePresence initial={false} mode="wait">
              {allMedia[currentIndex] && (
                 <motion.div
                   key={currentIndex}
                   initial={{ x: direction > 0 ? 300 : direction < 0 ? -300 : 0, opacity: 0 }}
                   animate={{ x: 0, opacity: 1 }}
                   exit={{ x: direction > 0 ? -300 : direction < 0 ? 300 : 0, opacity: 0 }}
                   transition={{ type: "spring", stiffness: 300, damping: 30 }}
                   drag="x"
                   dragConstraints={{ left: 0, right: 0 }}
                   dragElastic={1}
                   onDragEnd={handleDragEnd}
                   className="w-full h-full flex items-center justify-center absolute inset-0"
                 >
                    {allMedia[currentIndex].type === "image" && (
                      <img
                        src={allMedia[currentIndex].url}
                        alt="Fullscreen Preview"
                        className="w-full h-full object-contain select-none"
                        draggable={false}
                      />
                    )}
                    {allMedia[currentIndex].type === "video" && (
                      <video
                        src={allMedia[currentIndex].url}
                        controls
                        autoPlay
                        playsInline
                        className="w-full h-full object-contain"
                      />
                    )}
                 </motion.div>
              )}
            </AnimatePresence>
          </div>
          
          {allMedia.length > 1 && (
            <div className="absolute bottom-8 inset-x-0 flex justify-center z-20">
               <span className="text-white/80 bg-black/50 px-3 py-1 rounded-full text-[13px]">
                  {currentIndex + 1} / {allMedia.length}
               </span>
            </div>
          )}
        </motion.div>
      )}
    </AnimatePresence>
  );
};

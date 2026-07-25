import { useTranslation } from "react-i18next";
import React from "react";
import { X, Image as ImageIcon } from "lucide-react";
import { motion } from "motion/react";

interface MomentPublishModalProps {
  onClose: () => void;
  onSubmit: () => void;
  content: string;
  setContent: (content: string) => void;
  images: string[];
  setImages: (images: string[]) => void;
  addFakeImage: () => void;
}

export const MomentPublishModal: React.FC<MomentPublishModalProps> = ({
  onClose,
  onSubmit,
  content,
  setContent,
  images,
  setImages,
  addFakeImage
}) => {
  const { t } = useTranslation();
return (
    <motion.div
      initial={{ opacity: 0, y: "100%" }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: "100%" }}
      transition={{ type: "spring", damping: 25, stiffness: 200 }}
      className="fixed inset-0 z-50 bg-[#F2F2F7] dark:bg-black flex flex-col pt-safe"
    >
      {/* Header */}
      <div className="h-[56px] flex items-center justify-between px-4 bg-white dark:bg-[#1C1C1E] shrink-0 shadow-sm relative z-10 border-b border-black/5 dark:border-white/5">
        <button 
          onClick={onClose}
          className="text-[16px] text-text-main font-medium active:opacity-50"
        >{t('user.auto_a9472', '取消')}</button>
        <button 
          onClick={onSubmit}
          disabled={!content.trim() && images.length === 0}
          className="bg-[#07C160] disabled:opacity-50 disabled:bg-[#07C160]/70 text-white text-[14px] font-medium px-4 py-1.5 rounded active:bg-[#06ad56] transition-colors"
        >{t('user.auto_aaeb7', '发表')}</button>
      </div>

      <div className="flex-1 bg-white dark:bg-[#1C1C1E] p-4 overflow-y-auto w-full">
        <textarea
          className="w-full h-32 bg-transparent outline-none text-[16px] text-text-main resize-none placeholder:text-text-sub"
          placeholder={t('user.auto_prop_387d57bc', '这一刻的想法...')}
          value={content}
          onChange={(e) => setContent(e.target.value)}
          autoFocus
        />

        {/* Image Grid */}
        <div className="grid grid-cols-3 gap-2 mt-2 w-full">
          {images.map((img, i) => (
            <div key={i} className="relative aspect-square rounded overflow-hidden bg-gray-100 dark:bg-gray-800">
              <img src={img} alt="upload" className="w-full h-full object-cover" />
              <button 
                className="absolute top-1 right-1 w-6 h-6 bg-black/50 text-white rounded-full flex items-center justify-center active:bg-black/70"
                onClick={() => setImages(images.filter((_, idx) => idx !== i))}
              >
                <X className="w-3.5 h-3.5" />
              </button>
            </div>
          ))}
          
          {images.length < 9 && (
            <button 
              onClick={addFakeImage}
              className="aspect-square bg-[#F5F5F5] dark:bg-[#2A2A2D] flex items-center justify-center rounded active:bg-[#EBEBEB] dark:active:bg-white/10 transition-colors"
            >
              <ImageIcon className="w-8 h-8 text-text-sub opacity-50" />
            </button>
          )}
        </div>
      </div>
    </motion.div>
  );
};

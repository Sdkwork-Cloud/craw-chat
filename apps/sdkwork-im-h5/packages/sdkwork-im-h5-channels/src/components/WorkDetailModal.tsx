import { useTranslation } from "react-i18next";
import React from "react";
import { motion, AnimatePresence } from "motion/react";
import { ChevronLeft, Share, MessageCircle, Heart, Star, Wand2, Play, Pause, Disc } from "lucide-react";
import { IconButton, Avatar, cn } from "@sdkwork/im-h5-commons";
import { CreativeWork } from "../types";

interface WorkDetailModalProps {
  work: CreativeWork | null;
  onClose: () => void;
  onRemix: (work: CreativeWork) => void;
}

export const WorkDetailModal: React.FC<WorkDetailModalProps> = ({ work, onClose, onRemix }) => {
  const { t } = useTranslation();
if (!work) return null;

  return (
    <AnimatePresence>
      <motion.div
        initial={{ opacity: 0, y: "100%" }}
        animate={{ opacity: 1, y: 0 }}
        exit={{ opacity: 0, y: "100%" }}
        transition={{ type: "spring", damping: 25, stiffness: 200 }}
        className="fixed inset-0 z-[60] bg-black text-white flex flex-col"
      >
        <div className="flex-1 relative bg-black">
          {/* Header */}
          <div className="absolute top-0 left-0 right-0 z-20 pt-safe flex items-center justify-between px-2 h-14 bg-gradient-to-b from-black/50 to-transparent">
            <IconButton 
              icon={<ChevronLeft className="w-6 h-6 text-white drop-shadow-md" />} 
              onClick={onClose}
              className="bg-black/20 hover:bg-black/40 backdrop-blur-sm"
            />
            <IconButton 
              icon={<Share className="w-5 h-5 text-white drop-shadow-md" />} 
              onClick={() => {}}
              className="bg-black/20 hover:bg-black/40 backdrop-blur-sm"
            />
          </div>

          {/* Media Content */}
          <div className="w-full h-[calc(100vh-100px)] flex items-center justify-center bg-black">
            {work.type === "video" ? (
              <video 
                src={work.mediaUrl} 
                className="w-full h-full object-cover" 
                controls 
                autoPlay 
                playsInline 
              />
            ) : (
              <img 
                src={work.mediaUrl} 
                alt={work.title} 
                className="w-full h-full object-contain" 
              />
            )}
          </div>

          <div className="absolute bottom-0 left-0 right-0 h-1/2 bg-gradient-to-t from-black/90 via-black/40 to-transparent pointer-events-none" />

          {/* Bottom Actions & Info */}
          <div className="absolute bottom-[24px] left-4 right-16 z-10 pointer-events-auto">
             <div className="flex items-center gap-2 mb-3">
               <Avatar src={work.avatar} className="w-8 h-8 border border-white/50" />
               <span className="font-medium text-[15px] drop-shadow-md">@{work.author}</span>
               <button className="px-3 py-1 bg-white text-black text-[12px] font-bold rounded-full ml-2 active:scale-95 transition-transform">{t('channels.auto_a49d5', '关注')}</button>
             </div>
             
             <h2 className="text-[16px] font-bold mb-1.5 drop-shadow-[0_1px_2px_rgba(0,0,0,0.8)] leading-tight">
               {work.title}
             </h2>
             <p className="text-[13px] text-white/80 drop-shadow-md line-clamp-2">{t('channels.auto_n4ec835e6', '在这里发现更多惊艳的创作内容！这件作品不仅展示了出色的视觉效果，更体现了创作者对大模型技术的深入理解和灵活应用。')}</p>
             
             <div className="flex items-center gap-2 mt-3 cursor-pointer">
                <Disc className="w-4 h-4 text-white/80 animate-spin-slow" />
                <span className="text-[12px] text-white/80 font-medium">{t('channels.auto_465ccae2', '@原声 - {work.author} 创作的原声')}</span>
             </div>
          </div>

          {/* Right Action Sidebar */}
          <div className="absolute right-3 bottom-[24px] z-10 flex flex-col items-center gap-5 pointer-events-auto">
            <ActionIcon icon={Heart} count={work.likes} />
            <ActionIcon icon={MessageCircle} count={work.comments} />
            <ActionIcon icon={Star} count={(work.likes / 2).toFixed(0)} />
            <ActionIcon icon={Wand2} count={work.remixes} onClick={() => onRemix(work)} />
            <div className="w-10 h-10 rounded-full bg-black/50 border border-white/20 flex items-center justify-center mt-3 animate-[spin_6s_linear_infinite]">
                 {work.type === 'video' ? <Play className="w-4 h-4 text-white ml-0.5" /> : <img src={work.avatar} className="w-full h-full rounded-full" />}
            </div>
          </div>
        </div>

        {/* Reply input simulation */}
        <div className="h-14 bg-[#121212] border-t border-white/10 flex items-center px-4 gap-3 shrink-0 pb-safe">
           <div className="flex-1 h-9 rounded-full bg-white/10 flex items-center px-4 text-[14px] text-white/50">{t('channels.auto_n65d4b670', '留下你的神评论...')}</div>
           <div className="flex items-center gap-4 text-white/80">
              <Heart className="w-6 h-6" />
              <Star className="w-6 h-6" />
           </div>
        </div>
      </motion.div>
    </AnimatePresence>
  );
};

const ActionIcon = ({ icon: Icon, count, onClick }: any) => {
  const { t } = useTranslation();
  
  return (
  <div className="flex flex-col items-center gap-1 cursor-pointer active:scale-90 transition-transform" onClick={onClick}>
    <Icon className="w-8 h-8 drop-shadow-[0_2px_4px_rgba(0,0,0,0.4)] text-white fill-transparent" strokeWidth={1.5} />
    <span className="text-[12px] font-semibold drop-shadow-[0_1px_2px_rgba(0,0,0,0.8)] text-white">
      {count > 10000 ? (count / 10000).toFixed(1) + "w" : count}
    </span>
  </div>
);
};


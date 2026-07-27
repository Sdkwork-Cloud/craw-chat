import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useNavigate, useParams } from "react-router";
import { PageLayout, showToast, IconButton } from "@sdkwork/im-h5-commons";
import { ChevronLeft, Share, MoreHorizontal, Wand2, Heart, MessageSquare } from "lucide-react";
import { WorkService, Work } from "../services/WorkService";

export const WorkDetailPage = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const { id } = useParams<{ id: string }>();
  const [work, setWork] = useState<Work | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (id) {
      loadWork(id);
    }
  }, [id]);

  const loadWork = async (workId: string) => {
    setLoading(true);
    try {
      const data = await WorkService.getMyWorks();
      const found = data.find(w => w.id === workId);
      if (found) {
        setWork(found);
      }
    } catch (e) {
      // ignore
    } finally {
      setLoading(false);
    }
  };

  const handleRemix = () => {
  showToast("正在准备创作环境...");
    setTimeout(() => {
      // In a real app, this would navigate to the editor with this work's data
      showToast("已进入二创模式");
    }, 1000);
  };

  if (loading) {
    return (
      <div className="flex flex-col h-full bg-bg-color justify-center items-center">
         <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
      </div>
    );
  }

  if (!work) {
    return (
      <div className="flex flex-col h-full bg-bg-color">
         <header className="h-[56px] flex items-center px-1 glass-header sticky top-0 z-10 shrink-0 pt-safe">
            <IconButton icon={<ChevronLeft className="w-6 h-6 text-text-main" />} onClick={() => navigate(-1)} />
         </header>
         <div className="flex-1 flex justify-center items-center text-text-sub text-[15px]">作品找不到了</div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full bg-black relative">
       {/* Background Image / Video Mock */}
       <div 
         className="absolute inset-0 bg-cover bg-center opacity-80 mix-blend-screen"
         style={{ backgroundImage: `url(${work.coverUrl})` }}
       >
          <div className="absolute inset-0 bg-gradient-to-b from-black/40 via-transparent to-black/90" />
       </div>

       {/* Header */}
       <header className="h-[56px] flex items-center justify-between px-1 sticky top-0 z-10 shrink-0 pt-safe text-white relative">
          <IconButton icon={<ChevronLeft className="w-6 h-6 text-white" strokeWidth={2.5} />} onClick={() => navigate(-1)} />
          <div className="flex-1" />
       </header>

       {/* Content */}
       <div className="flex-1 flex flex-col justify-end p-4 relative z-10 pb-6 pb-safe">
          <div className="flex justify-between items-end">
             <div className="flex-1 pr-12 text-white">
                <h1 className="text-[20px] font-bold mb-3 drop-shadow-md leading-snug">{work.title}</h1>
                <div className="flex items-center gap-2 mb-4">
                  <span className="text-[13px] bg-white/20 backdrop-blur-md px-2.5 py-1 rounded-full text-white/90">{work.type === 'video' ? '视频' : work.type === 'article' ? '图文' : work.type === 'audio' ? '音频' : 'AI作画'}</span>
                  <span className="text-[13px] text-white/70">
                    2026-05-26
                  </span>
                </div>
                <div className="text-[14px] text-white/80 leading-relaxed mb-4 line-clamp-3">这是该作品的详细描述内容。这里展示作品的相关信息，或者是AI创作时使用的提示词。点击可以查看完整信息。</div>

                {/* Remix Button */}
                <button 
                  onClick={handleRemix}
                  className="mt-2 flex items-center gap-2 bg-gradient-to-r from-blue-600 to-indigo-600 hover:from-blue-500 hover:to-indigo-500 text-white px-5 py-2.5 rounded-full font-medium active:scale-95 transition-transform shadow-lg shadow-blue-500/20 w-fit border border-white/10"
                >
                  <Wand2 className="w-[18px] h-[18px]" strokeWidth={2.5} />
                  <span>二创同款 (Remix)</span>
                </button>
             </div>

             {/* Right sidebar actions */}
             <div className="flex flex-col gap-6 items-center w-12 pb-2">
                <div className="flex flex-col items-center gap-1 active:scale-95 transition-transform cursor-pointer">
                   <div className="w-10 h-10 rounded-full bg-black/40 backdrop-blur-md flex items-center justify-center border border-white/10">
                     <Heart className="w-5 h-5 text-red-500 fill-red-500" strokeWidth={1.5} />
                   </div>
                   <span className="text-white font-medium text-[12px] drop-shadow-md">
                     {work.likes >= 10000 ? (work.likes / 10000).toFixed(1) + 'w' : work.likes}
                   </span>
                </div>
                <div className="flex flex-col items-center gap-1 active:scale-95 transition-transform cursor-pointer">
                   <div className="w-10 h-10 rounded-full bg-black/40 backdrop-blur-md flex items-center justify-center border border-white/10">
                     <MessageSquare className="w-5 h-5 text-white" strokeWidth={1.5} />
                   </div>
                   <span className="text-white font-medium text-[12px] drop-shadow-md">
                     {work.comments >= 10000 ? (work.comments / 10000).toFixed(1) + 'w' : work.comments}
                   </span>
                </div>
                <div className="flex flex-col items-center gap-1 active:scale-95 transition-transform cursor-pointer">
                   <div className="w-10 h-10 rounded-full bg-black/40 backdrop-blur-md flex items-center justify-center border border-white/10">
                     <Share className="w-5 h-5 text-white" strokeWidth={1.5} />
                   </div>
                   <span className="text-white font-medium text-[12px] drop-shadow-md">分享</span>
                </div>
                <div className="flex flex-col items-center gap-1 active:scale-95 transition-transform cursor-pointer">
                   <div className="w-10 h-10 rounded-full bg-black/40 backdrop-blur-md flex items-center justify-center border border-white/10">
                     <MoreHorizontal className="w-5 h-5 text-white" strokeWidth={1.5} />
                   </div>
                </div>
             </div>
          </div>
       </div>
    </div>
  );
};

import { useTranslation } from "react-i18next";
import React from "react";
import { Search, Heart, Video } from "lucide-react";
import { Avatar, cn } from "@sdkwork/im-h5-commons";
import { CreativeWork } from "../types";

interface WaterfallFeedProps {
  works: CreativeWork[];
  onWorkClick?: (work: CreativeWork) => void;
}

export const WaterfallFeed: React.FC<WaterfallFeedProps> = ({ works, onWorkClick }) => {
  const { t } = useTranslation();
// Multiply dummy items for waterfall demo
  const waterfallItems = [...works, ...works, ...works].map((w, i) => ({
    ...w,
    id: `wf-${i}`,
    heightClass: i % 2 === 0 ? "h-[260px]" : "h-[200px]"
  }));

  const leftCol = waterfallItems.filter((_, i) => i % 2 === 0);
  const rightCol = waterfallItems.filter((_, i) => i % 2 !== 0);

  return (
    <div className="w-full h-full bg-[#121212] overflow-y-auto pb-[60px]">
       <div className="pt-safe px-4 pb-2 mt-4 flex items-center justify-between sticky top-0 bg-[#121212]/90 backdrop-blur-md z-10">
          <h2 className="text-[18px] font-bold text-white tracking-wide mix-blend-difference ml-12">{t('channels.auto_n8535ee7', 'Explore the creators\' world')}</h2>
          <div className="w-8 h-8 rounded-full bg-white/10 flex items-center justify-center cursor-pointer active:opacity-70">
             <Search className="w-4 h-4 text-white" />
          </div>
       </div>

       <div className="px-2 pt-2 flex items-start gap-2">
         {/* Left Column */}
         <div className="flex-1 flex flex-col gap-2">
           {leftCol.map((item) => (
             <WaterfallCard key={item.id} item={item} onClick={() => onWorkClick?.(item)} />
           ))}
         </div>
         {/* Right Column */}
         <div className="flex-1 flex flex-col gap-2">
           {rightCol.map((item) => (
             <WaterfallCard key={item.id} item={item} onClick={() => onWorkClick?.(item)} />
           ))}
         </div>
       </div>
    </div>
  );
};

const WaterfallCard: React.FC<{ item: any, onClick: () => void }> = ({ item, onClick }) => (
  <div 
    className="w-full rounded-md overflow-hidden bg-[#1D1D1D] relative shadow-sm cursor-pointer active:opacity-90 transition-opacity"
    onClick={onClick}
  >
     <div className={cn("w-full relative", item.heightClass)}>
        <img src={item.mediaUrl} alt={item.title} className="w-full h-full object-cover" />
        {item.type === "video" && (
           <div className="absolute top-2 right-2 w-5 h-5 bg-black/40 rounded-full backdrop-blur-sm flex items-center justify-center">
              <Video className="w-3 h-3 text-white" />
           </div>
        )}
     </div>
     <div className="p-2.5">
        <p className="text-[13px] text-white/90 leading-snug font-medium mb-2.5 line-clamp-2">
           {item.title}
        </p>
        <div className="flex items-center justify-between text-[11px] text-white/50">
           <div className="flex items-center gap-1.5 min-w-0">
              <Avatar src={item.avatar} className="w-4 h-4 shrink-0" />
              <span className="truncate max-w-[80px] font-medium">{item.author}</span>
           </div>
           <div className="flex items-center gap-1 shrink-0">
              <Heart className="w-3.5 h-3.5" />
              <span className="font-medium">{item.likes > 1000 ? (item.likes/1000).toFixed(1) + 'k' : item.likes}</span>
           </div>
        </div>
     </div>
  </div>
);

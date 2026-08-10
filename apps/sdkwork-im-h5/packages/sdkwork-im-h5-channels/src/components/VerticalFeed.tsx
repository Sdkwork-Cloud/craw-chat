import { useTranslation } from "react-i18next";
import React from "react";
import { Search, Heart, MessageCircle, Send, Wand2, Video, Image as ImageIcon } from "lucide-react";
import { motion } from "motion/react";
import { Avatar, cn } from "@sdkwork/im-h5-commons";
import { CreativeWork } from "../types";

interface VerticalFeedProps {
  feedTab: string;
  setFeedTab: (v: any) => void;
  onRemix: (work: CreativeWork) => void;
  works: CreativeWork[];
}

export const VerticalFeed: React.FC<VerticalFeedProps> = ({ feedTab, setFeedTab, onRemix, works }) => {
  const { t } = useTranslation();
return (
    <>
      <div className="absolute top-0 left-0 right-0 z-40 flex justify-center items-center px-4 pt-safe mt-4 pointer-events-none">
        <div className="flex gap-6 text-[16px] font-medium drop-shadow-[0_2px_4px_rgba(0,0,0,0.5)] leading-none pointer-events-auto">{["关注", "朋友", "推荐"].map((tab) => (<div
              key={tab}
              className="relative cursor-pointer py-1 px-1"
              onClick={() => setFeedTab(tab)}
            >
              <span className={cn(
                "transition-colors",
                feedTab === tab ? "text-white drop-shadow-md font-semibold text-[17px]" : "text-white/70 font-medium"
              )}>
                {tab}
              </span>
              {feedTab === tab && (
                <div className="absolute -bottom-1.5 left-0 right-0 flex justify-center">
                  <motion.div layoutId="channelTabIndicator" className="w-5 h-0.5 bg-white rounded-full drop-shadow-md" />
                </div>
              )}
            </div>
          ))}
        </div>
        <div className="absolute right-4 top-safe pointer-events-auto">
           <Search className="w-[22px] h-[22px] text-white/90 drop-shadow-[0_2px_4px_rgba(0,0,0,0.5)] cursor-pointer active:opacity-50" strokeWidth={2.5} />
        </div>
      </div>

      <div className="flex-1 overflow-y-scroll snap-y snap-mandatory scroll-smooth no-scrollbar relative w-full h-[calc(100vh-52px)]">
        {works.map((work) => (
          <div
            key={work.id}
            className="w-full h-[calc(100vh-52px)] snap-start relative flex items-center justify-center bg-black overflow-hidden"
          >
            {/* Media Content */}
            {work.type === "video" ? (
              <video 
                src={work.mediaUrl} 
                className="w-full h-full object-cover" 
                muted 
                loop 
                controls={false}
                autoPlay
                playsInline
              />
            ) : (
              <img 
                src={work.mediaUrl}
                alt={work.title}
                className="w-full h-full object-cover"
              />
            )}
            
            {/* Dimming overlay gradient for readability */}
            <div className="absolute bottom-0 left-0 right-0 h-1/2 bg-gradient-to-t from-black/80 via-black/20 to-transparent pointer-events-none" />

            {/* Sidebar actions */}
            <div className="absolute right-3 bottom-0 flex flex-col items-center gap-5 z-10 w-12 pb-[68px]">
              <div className="relative mb-2">
                <Avatar
                  src={work.avatar}
                  className="w-11 h-11 border-2 border-white/80"
                />
                <div className="absolute -bottom-2 inset-x-0 mx-auto w-4 h-4 bg-red-500 rounded-full flex items-center justify-center text-white text-[12px] border border-black cursor-pointer leading-none">
                  +
                </div>
              </div>
              <ActionIcon icon={Heart} count={work.likes} />
              <ActionIcon icon={MessageCircle} count={work.comments} />
              <ActionIcon icon={Send} count={work.shares} />
              <ActionIcon icon={Wand2} count={work.remixes} onClick={() => onRemix(work)} />
              
              <div className="w-10 h-10 rounded-full bg-black/50 border border-white/20 flex items-center justify-center mt-3 animate-[spin_6s_linear_infinite]">
                 {work.type === 'video' ? <Video className="w-5 h-5 text-white" /> : <ImageIcon className="w-5 h-5 text-white" />}
              </div>
            </div>

            {/* Bottom info */}
            <div className="absolute left-4 bottom-0 right-20 z-10 pb-[68px]">
              <h3 className="font-bold text-[17px] mb-2 drop-shadow-[0_1px_2px_rgba(0,0,0,0.8)] text-white">
                @{work.author}
              </h3>
              <p className="text-[14.5px] line-clamp-3 drop-shadow-[0_1px_2px_rgba(0,0,0,0.8)] text-white/95 leading-relaxed max-w-[95%] font-medium">
                {work.title}
              </p>
            </div>
          </div>
        ))}
      </div>
    </>
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


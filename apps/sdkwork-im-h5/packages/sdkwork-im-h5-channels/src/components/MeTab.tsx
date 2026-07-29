import { useTranslation } from "react-i18next";
import React, { useState } from "react";
import { Search, MapPin, Grid, Heart, Wand2, Settings, Lock } from "lucide-react";
import { Avatar, cn } from "@sdkwork/im-h5-commons";
import { CreativeWork } from "../types";

export const MeTab = ({ works }: { works: CreativeWork[] }) => {
  const { t } = useTranslation();
const [activeTab, setActiveTab] = useState<"works" | "prompts" | "favorites" | "private">("works");
  
  return (
    <div className="w-full h-full bg-[#121212] overflow-y-auto pb-[60px] relative">
       {/* Header / Profile Info */}
       <div className="w-full relative pt-safe bg-gradient-to-b from-[#2B2B2B] to-[#121212]">
          <div className="flex items-center justify-end px-4 py-2 text-white">
             <Settings className="w-6 h-6" />
          </div>
          
          <div className="px-4 pb-4 flex items-start gap-4">
             <Avatar fallback="?" className="w-20 h-20 rounded-full border-2 border-white/10" />
             <div className="flex-1 mt-1">
                <h1 className="text-[20px] font-bold text-white mb-1">{t('channels.auto_609c8ff', 'AI魔法师')}</h1>
                <p className="text-[12px] text-white/50 mb-3">{t('channels.auto_98ad7ce', '账号: sdkwork_ai_master')}</p>
                
                <div className="flex items-center gap-6">
                   <div className="flex flex-col">
                      <span className="font-bold text-white text-[16px]">128</span>
                      <span className="text-[12px] text-white/50">{t('channels.auto_n7e715800', '获赞与收藏')}</span>
                   </div>
                   <div className="flex flex-col">
                      <span className="font-bold text-white text-[16px]">42</span>
                      <span className="text-[12px] text-white/50">{t('channels.auto_a49d5', '关注')}</span>
                   </div>
                   <div className="flex flex-col">
                      <span className="font-bold text-white text-[16px]">1.2w</span>
                      <span className="text-[12px] text-white/50">{t('channels.auto_f62b4', '粉丝')}</span>
                   </div>
                </div>
             </div>
          </div>
          
          <div className="px-4 pb-4">
             <p className="text-[13px] text-white/80 leading-relaxed max-w-[90%]">{t('channels.auto_444be3cd', '用代码和提示词构建未来的世界。喜欢探索前沿AIGC技术，欢迎交流！')}<br />
               <span className="text-white/40 flex items-center gap-1 mt-1">
                 <MapPin className="w-3 h-3" />{t('channels.auto_30928e61', '星辰大海')}</span>
             </p>
             <div className="flex gap-2 mt-4">
                <button className="flex-1 h-9 rounded bg-white/10 text-white font-medium text-[14px]">{t('channels.auto_3bf19150', '编辑资料')}</button>
                <button className="flex-1 h-9 rounded bg-white/10 text-white font-medium text-[14px]">{t('channels.auto_334184e5', '添加朋友')}</button>
             </div>
          </div>
       </div>

       {/* Tabs */}
       <div className="sticky top-0 bg-[#121212]/95 backdrop-blur-md z-10 flex border-b border-white/5 px-2 pt-2">
          <TabItem icon={Grid} label={`作品 ${works.length}`} active={activeTab === 'works'} onClick={() => setActiveTab('works')} />
          <TabItem icon={Wand2} label={t('channels.auto_prop_3e900dbe', '提示词 12')} active={activeTab === 'prompts'} onClick={() => setActiveTab('prompts')} />
          <TabItem icon={Heart} label={t('channels.auto_prop_n30ed3358', '收藏 45')} active={activeTab === 'favorites'} onClick={() => setActiveTab('favorites')} />
          <TabItem icon={Lock} label={t('channels.auto_prop_f1a25', '私密')} active={activeTab === 'private'} onClick={() => setActiveTab('private')} />
       </div>

       {/* Sub-feed Grid Content */}
       <div className="grid grid-cols-3 gap-0.5 p-0.5">
          {works.map((work, i) => (
            <div key={i} className="aspect-[3/4] bg-[#1d1d1d] relative overflow-hidden">
               <img src={work.mediaUrl} className="w-full h-full object-cover" />
               <div className="absolute bottom-1 left-1 flex items-center gap-1 text-white text-[11px] font-medium drop-shadow-md">
                 <Heart className="w-3 h-3" /> {work.likes > 1000 ? (work.likes/1000).toFixed(1) + 'k' : work.likes}
               </div>
            </div>
          ))}
          {/* duplicate works for display */}
          {works.map((work, i) => (
            <div key={`d-${i}`} className="aspect-[3/4] bg-[#1d1d1d] relative overflow-hidden">
               <img src={work.mediaUrl} className="w-full h-full object-cover" />
               <div className="absolute bottom-1 left-1 flex items-center gap-1 text-white text-[11px] font-medium drop-shadow-md">
                 <Heart className="w-3 h-3" /> {work.likes > 1000 ? (work.likes/1000).toFixed(1) + 'k' : work.likes}
               </div>
            </div>
          ))}
       </div>
    </div>
  );
};

const TabItem = ({ icon: Icon, label, active, onClick }: any) => {
  const { t } = useTranslation();
  
  return (
  <div 
    className="flex-1 flex flex-col items-center justify-center gap-1.5 pb-2 relative cursor-pointer"
    onClick={onClick}
  >
    <span className={cn("text-[14px] font-medium transition-colors flex items-center gap-1.5", active ? "text-white" : "text-white/50")}>
       {label}
    </span>
    {active && <div className="absolute bottom-0 w-8 h-0.5 bg-white rounded-t-full" />}
  </div>
);
};

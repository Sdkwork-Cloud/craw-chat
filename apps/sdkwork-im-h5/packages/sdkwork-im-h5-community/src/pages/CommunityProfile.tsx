import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useParams, useNavigate, useLocation } from "react-router";
import { CommunityService } from "../services/CommunityService";
import { Community } from "../types";
import { cn, IconButton, showToast } from "@sdkwork/im-h5-commons";
import { ChevronLeft, ChevronRight, Camera, Image as ImageIcon, QrCode } from "lucide-react";
import { AVAILABLE_TABS } from "./CommunityEditTabs";

export const CommunityProfile: React.FC = () => {
  const { t } = useTranslation();
const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const location = useLocation();
  
  const [community, setCommunity] = useState<Community | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    loadData();
  }, [id, location]);

  const loadData = async () => {
    if (!id) return;
    setIsLoading(true);
    try {
      const comm = await CommunityService.getCommunityById(id);
      if (comm) {
        setCommunity(comm);
      }
    } catch {
      showToast("获取圈子配置失败");
    } finally {
      setIsLoading(false);
    }
  };

  const navigateToEditImage = (type: 'avatar' | 'coverImage') => {
  navigate(`/community/${id}/profile/image?field=${type}`);
  };

  if (isLoading && !community) {
    return (
      <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black text-text-main">
         <header className="h-[56px] px-4 flex items-center shrink-0 pt-safe bg-bg-color">
            <IconButton icon={<ChevronLeft className="w-6 h-6" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
         </header>
         <div className="flex-1 flex items-center justify-center">加载中...</div>
      </div>
    );
  }

  if (!community) return null;

  return (
    <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black relative text-text-main">
       <header className="h-[56px] px-4 flex items-center justify-between shrink-0 pt-safe bg-white dark:bg-[#1C1C1E] z-20 shadow-sm relative">
          <div className="absolute left-4 z-10">
            <IconButton icon={<ChevronLeft className="w-6 h-6 text-text-main" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
          </div>
          <h1 className="text-[17px] font-semibold flex-1 text-center">圈子信息</h1>
       </header>

       <div className="flex-1 overflow-y-auto pb-safe">
          
          <div className="bg-white dark:bg-[#1C1C1E] mt-4 border-y border-black/5 dark:border-white/5 pl-4">
             <div 
               className="flex items-center justify-between py-3 pr-4 border-b border-black/5 dark:border-white/5 cursor-pointer active:opacity-70 transition-opacity"
               onClick={() => navigateToEditImage('avatar')}
             >
               <span className="text-[16px]">圈子头像</span>
               <div className="flex items-center gap-2">
                 <div className="w-12 h-12 rounded-full bg-gray-100 dark:bg-[#2C2C2E] overflow-hidden flex items-center justify-center">
                    {community.avatar ? (
                       <img src={community.avatar} alt="Avatar" className="w-full h-full object-cover" />
                    ) : (
                       <ImageIcon className="w-5 h-5 text-text-sub opacity-50" />
                    )}
                 </div>
                 <ChevronRight className="w-5 h-5 text-text-sub opacity-50" />
               </div>
             </div>

             <div 
               className="flex items-center justify-between py-3 pr-4 cursor-pointer active:opacity-70 transition-opacity"
               onClick={() => navigateToEditImage('coverImage')}
             >
               <span className="text-[16px]">圈子背景</span>
               <div className="flex items-center gap-2">
                 <div className="w-20 h-12 rounded-md bg-gray-100 dark:bg-[#2C2C2E] overflow-hidden flex items-center justify-center">
                    {community.coverImage ? (
                       <img src={community.coverImage} alt="Cover" className="w-full h-full object-cover" />
                    ) : (
                       <ImageIcon className="w-5 h-5 text-text-sub opacity-50" />
                    )}
                 </div>
                 <ChevronRight className="w-5 h-5 text-text-sub opacity-50" />
               </div>
             </div>
          </div>

          <div className="bg-white dark:bg-[#1C1C1E] mt-4 border-y border-black/5 dark:border-white/5 pl-4">
             <div 
               className="flex items-center justify-between py-4 pr-4 border-b border-black/5 dark:border-white/5 cursor-pointer active:opacity-70 transition-opacity"
               onClick={() => navigate(`/community/${id}/profile/edit?field=name`)}
             >
               <span className="text-[16px] whitespace-nowrap">圈子名称</span>
               <div className="flex items-center gap-2 flex-1 justify-end overflow-hidden pl-4">
                 <span className="text-[15px] text-text-sub truncate">{community.name}</span>
                 <ChevronRight className="w-5 h-5 text-text-sub opacity-50 shrink-0" />
               </div>
             </div>

             <div 
               className="flex items-center justify-between py-4 pr-4 border-b border-black/5 dark:border-white/5 cursor-pointer active:opacity-70 transition-opacity"
               onClick={() => navigate(`/community/${id}/profile/edit?field=description`)}
             >
               <span className="text-[16px] whitespace-nowrap">圈子简介</span>
               <div className="flex items-center gap-2 flex-1 justify-end overflow-hidden pl-4">
                 <span className="text-[15px] text-text-sub truncate">{community.description || '未设置'}</span>
                 <ChevronRight className="w-5 h-5 text-text-sub opacity-50 shrink-0" />
               </div>
             </div>

             <div 
               className="flex items-center justify-between py-4 pr-4 border-b border-black/5 dark:border-white/5 cursor-pointer active:opacity-70 transition-opacity"
               onClick={() => navigate(`/community/${id}/profile/edit?field=tags`)}
             >
               <span className="text-[16px] whitespace-nowrap">圈子标签</span>
               <div className="flex items-center gap-2 flex-1 justify-end overflow-hidden pl-4">
                 <span className="text-[15px] text-text-sub truncate">{community.tags.join(' ') || '未设置'}</span>
                 <ChevronRight className="w-5 h-5 text-text-sub opacity-50 shrink-0" />
               </div>
             </div>

             <div 
               className="flex items-center justify-between py-4 pr-4 border-b border-black/5 dark:border-white/5 cursor-pointer active:opacity-70 transition-opacity"
               onClick={() => navigate(`/community/${id}/profile/tabs`)}
             >
               <span className="text-[16px] whitespace-nowrap">展示模块</span>
               <div className="flex items-center gap-2 flex-1 justify-end overflow-hidden pl-4">
                 <span className="text-[15px] text-text-sub truncate">{community.tabs?.length ? `${community.tabs.length}个模块已选` : '3个模块已选'}</span>
                 <ChevronRight className="w-5 h-5 text-text-sub opacity-50 shrink-0" />
               </div>
             </div>

             <div 
               className="flex items-center justify-between py-4 pr-4 cursor-pointer active:opacity-70 transition-opacity"
               onClick={() => navigate(`/community/${id}/profile/qrcode`)}
             >
               <span className="text-[16px] whitespace-nowrap">圈子二维码</span>
               <div className="flex items-center gap-2 flex-1 justify-end overflow-hidden pl-4">
                 <QrCode className="w-4 h-4 text-text-sub opacity-50 shrink-0" />
                 <ChevronRight className="w-5 h-5 text-text-sub opacity-50 shrink-0" />
               </div>
             </div>
          </div>

          <div className="bg-white dark:bg-[#1C1C1E] mt-4 border-y border-black/5 dark:border-white/5 pl-4">
             <div 
               className="flex items-center justify-between py-4 pr-4 border-b border-black/5 dark:border-white/5 cursor-pointer active:opacity-70 transition-opacity"
               onClick={() => navigate(`/community/${id}/profile/groups`)}
             >
               <span className="text-[16px]">群组管理</span>
               <div className="flex items-center gap-2 flex-1 justify-end">
                 <ChevronRight className="w-5 h-5 text-text-sub opacity-50 shrink-0" />
               </div>
             </div>

             <div 
               className="flex items-center justify-between py-4 pr-4 cursor-pointer active:opacity-70 transition-opacity"
               onClick={() => navigate(`/community/${id}/profile/members`)}
             >
               <span className="text-[16px]">成员管理</span>
               <div className="flex items-center gap-2 flex-1 justify-end">
                 <span className="text-[15px] text-text-sub">{community.memberCount} 人</span>
                 <ChevronRight className="w-5 h-5 text-text-sub opacity-50 shrink-0" />
               </div>
             </div>
          </div>

       </div>
    </div>
  );
};

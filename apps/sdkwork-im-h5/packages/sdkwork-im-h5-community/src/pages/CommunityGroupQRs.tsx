import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router";
import { CommunityService } from "../services/CommunityService";
import { CommunityGroup } from "../types";
import { IconButton, showToast } from "@sdkwork/im-h5-commons";
import { ChevronLeft, MessageSquare, Download, Share2 } from "lucide-react";

export const CommunityGroupQRs: React.FC = () => {
  const { t } = useTranslation();
const { id, groupId } = useParams<{ id: string, groupId: string }>();
  const navigate = useNavigate();
  const [group, setGroup] = useState<CommunityGroup | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  
  useEffect(() => {
    loadData();
  }, [id, groupId]);

  const loadData = async () => {
    if (!id || !groupId) return;
    setIsLoading(true);
    try {
      const groups = await CommunityService.getGroupsByCommunity(id);
      const found = groups.find(g => g.id === groupId);
      if (found) setGroup(found);
    } catch {
      showToast("获取群组详情失败");
    } finally {
      setIsLoading(false);
    }
  };

  const platformNameMap: Record<string, string> = {
    wechat: '微信',
    qq: 'QQ',
    feishu: '飞书',
    dingtalk: '钉钉',
    telegram: 'Telegram',
    discord: 'Discord',
    whatsapp: 'WhatsApp',
    other: '其他'
  };

  const handleSaveImage = (url: string) => {
  try {
      const link = document.createElement('a');
      link.href = url;
      link.download = `qrcode_${Date.now()}.png`;
      link.click();
      showToast("已保存到相册");
    } catch {
      showToast("保存失败");
    }
  };

  if (isLoading) {
    return (
      <div className="flex flex-col h-full bg-[#1C1C1E] dark:bg-black text-white">
         <header className="h-[56px] px-4 flex items-center justify-between sticky top-0 z-10 shrink-0 pt-safe bg-transparent">
            <IconButton icon={<ChevronLeft className="w-6 h-6 text-white" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
         </header>
         <div className="flex-1 flex items-center justify-center text-white/50">加载中...</div>
      </div>
    );
  }

  if (!group) {
    return (
      <div className="flex flex-col h-full bg-[#1C1C1E] dark:bg-black text-white">
        <header className="h-[56px] px-4 flex items-center sticky top-0 z-10 pt-safe bg-[#1C1C1E]">
            <IconButton icon={<ChevronLeft className="w-6 h-6 text-white" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
        </header>
        <div className="flex-1 flex items-center justify-center text-white/50">群组不存在</div>
      </div>
    );
  }

  // Handle both legacy (qrCodeUrl) and new (qrCodes) data forms
  const allQrs = group.qrCodes || (group.qrCodeUrl ? [{ url: group.qrCodeUrl, description: '' }] : []);

  return (
    <div className="flex flex-col h-full bg-[#111111] dark:bg-black text-white relative">
       <header className="h-[56px] px-4 flex items-center justify-between sticky top-0 z-20 shrink-0 pt-safe bg-black/50 backdrop-blur-md">
          <IconButton icon={<ChevronLeft className="w-6 h-6 text-white" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
          <h1 className="text-[17px] font-semibold flex-1 text-center truncate px-2">{group.name}</h1>
          <IconButton icon={<Share2 className="w-5 h-5 text-white" />} className="bg-transparent w-10 h-10 p-0" onClick={() => showToast("请截图分享或者保存二维码")} /></header>

       <div className="flex-1 overflow-y-auto no-scrollbar pt-6 pb-20 px-4">
          <div className="flex flex-col gap-6 items-center">
             {allQrs.map((qrItem, idx) => (
                <div key={idx} className="bg-[#222222] w-full max-w-[320px] rounded-[24px] p-6 flex flex-col items-center shadow-lg border border-white/5">
                   {qrItem.description && (
                      <div className="w-full text-center text-[15px] font-medium text-white/90 mb-4 bg-white/5 py-2 px-4 rounded-xl">
                        {qrItem.description}
                      </div>
                   )}
                   <div className="w-full aspect-square bg-white rounded-[16px] overflow-hidden p-2 relative flex items-center justify-center mx-auto max-w-[240px]">
                      <div className="absolute inset-1 border-2 border-dashed border-black/10 rounded-xl" />
                      <img src={qrItem.url} alt="QR Code" className="w-[90%] h-[90%] object-contain relative z-10" />
                   </div>
                   
                   <p className="text-[13px] text-white/50 text-center font-medium mt-6 leading-relaxed">长按识别或者保存二维码<br />打开对应 App 进行扫码</p>
                   
                  <button 
                     onClick={() => handleSaveImage(qrItem.url)}
                     className="flex items-center gap-1.5 mt-5 text-[14px] font-medium text-blue-400 active:opacity-70 transition-opacity px-4 py-2 bg-blue-500/10 rounded-full"
                   >
                      <Download className="w-4 h-4" />保存到相册</button>
                </div>
             ))}
             
             {allQrs.length === 0 && (
               <div className="text-white/40 text-[14px]">暂无可用的加群二维码</div>
             )}
          </div>
       </div>
    </div>
  );
};

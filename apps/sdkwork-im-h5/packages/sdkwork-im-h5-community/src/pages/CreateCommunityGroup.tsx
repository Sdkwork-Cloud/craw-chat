import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router";
import { CommunityService } from "../services/CommunityService";
import { CommunityGroup, QRCodeItem } from "../types";
import { cn, IconButton, showToast } from "@sdkwork/im-h5-commons";
import { ChevronLeft, Plus, X, UploadCloud, MessageSquare } from "lucide-react";

export const CreateCommunityGroup: React.FC = () => {
  const { t } = useTranslation();
const { id, groupId } = useParams<{ id: string, groupId?: string }>();
  const navigate = useNavigate();
  const [name, setName] = useState("");
  const [platform, setPlatform] = useState<CommunityGroup['platform']>('wechat');
  const [description, setDescription] = useState("");
  const [qrCodes, setQrCodes] = useState<QRCodeItem[]>([]);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isLoading, setIsLoading] = useState(!!groupId);

  const isEditMode = !!groupId;

  useEffect(() => {
    if (groupId && id) {
      loadGroup();
    }
  }, [id, groupId]);

  const loadGroup = async () => {
    try {
      const groups = await CommunityService.getGroupsByCommunity(id!);
      const group = groups.find(g => g.id === groupId);
      if (group) {
        setName(group.name);
        setPlatform(group.platform);
        setDescription(group.description || "");
        setQrCodes(group.qrCodes || (group.qrCodeUrl ? [{ url: group.qrCodeUrl, description: '' }] : []));
      }
    } catch {
      showToast("获取群组失败");
    } finally {
      setIsLoading(false);
    }
  };

  const handleAddQr = () => {
  const input = document.createElement('input');
    input.type = 'file';
    input.accept = 'image/*';
    input.onchange = (e) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (file) {
        const reader = new FileReader();
        reader.onload = (e) => {
          if (e.target?.result) {
            setQrCodes(prev => [...prev, { url: e.target!.result as string, description: '' }]);
          }
        };
        reader.readAsDataURL(file);
      }
    };
    input.click();
  };

  const handleUpdateQrDescription = (index: number, value: string) => {
  const updated = [...qrCodes];
    updated[index].description = value;
    setQrCodes(updated);
  };

  const handleRemoveQr = (index: number) => {
  setQrCodes(qrCodes.filter((_, i) => i !== index));
  };

  const handleSubmit = async () => {
    if (!id) return;
    if (!name.trim()) return showToast("请输入群组名称");
    if (qrCodes.length === 0) return showToast("请至少上传一张二维码");
    
    setIsSubmitting(true);
    try {
      const payload = {
        name,
        platform,
        description,
        memberCount: isEditMode ? undefined : 0,
        qrCodes
      };

      if (isEditMode && groupId) {
        await CommunityService.updateGroup(id, groupId, payload);
        showToast("群组修改成功");
      } else {
        await CommunityService.createGroup(id, payload as any);
        showToast("群组创建成功");
      }
      navigate(-1);
    } catch {
      showToast(isEditMode ? "修改失败" : "创建失败");
    } finally {
      setIsSubmitting(false);
    }
  };

  if (isLoading) {
    return (
      <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black">
        <header className="h-[56px] px-4 flex items-center sticky top-0 z-10 pt-safe bg-bg-color shrink-0">
            <IconButton icon={<ChevronLeft className="w-6 h-6 text-text-main" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
        </header>
        <div className="flex-1 flex items-center justify-center text-text-sub">加载中...</div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black">
      <header className="h-[56px] px-4 flex items-center justify-between sticky top-0 z-10 pt-safe bg-bg-color shrink-0 shadow-sm border-b border-black/5 dark:border-white/5">
         <div className="flex items-center gap-2">
            <IconButton icon={<ChevronLeft className="w-6 h-6 text-text-main" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
            <h1 className="text-[17px] font-semibold text-text-main">{isEditMode ? "编辑群组" : "添加群组"}</h1>
         </div>
      </header>
      
      <div className="flex-1 flex flex-col pt-4 overflow-y-auto w-full gap-4 pb-safe bg-white dark:bg-[#1E1E1E]">
        
        <div className="px-4">
           <label className="text-[14px] font-medium text-text-main mb-2 block">群组名称 *</label>
           <input 
             type="text"
             value={name}
             onChange={e => setName(e.target.value)}
             className="w-full bg-[#f8f9fa] dark:bg-[#2C2C2E] px-4 py-3 rounded-2xl outline-none text-[15px] placeholder:text-text-sub focus:ring-1 focus:ring-blue-500 transition-shadow"
             placeholder="如: AI 开发者微信1群"
           />
        </div>

        <div className="px-4">
           <label className="text-[14px] font-medium text-text-main mb-2 block">平台 *</label>
           <select 
             value={platform}
             onChange={e => setPlatform(e.target.value as any)}
             className="w-full bg-[#f8f9fa] dark:bg-[#2C2C2E] px-4 py-3 rounded-2xl outline-none text-[15px] focus:ring-1 focus:ring-blue-500 transition-shadow appearance-none"
           >
             <option value="wechat">微信</option>
             <option value="qq">QQ</option>
             <option value="telegram">Telegram</option>
             <option value="discord">Discord</option>
             <option value="feishu">飞书</option>
             <option value="dingtalk">钉钉</option>
             <option value="whatsapp">WhatsApp</option>
             <option value="other">其他</option>
           </select>
        </div>

        <div className="px-4">
           <label className="text-[14px] font-medium text-text-main mb-2 block">描述 (选填)</label>
           <textarea 
             value={description}
             onChange={e => setDescription(e.target.value)}
             className="w-full bg-[#f8f9fa] dark:bg-[#2C2C2E] px-4 py-3 rounded-2xl outline-none text-[15px] placeholder:text-text-sub focus:ring-1 focus:ring-blue-500 transition-shadow resize-none h-24"
             placeholder="群组规则或介绍..."
           />
        </div>

        <div className="px-4 pt-2">
           <label className="text-[14px] font-medium text-text-main mb-3 flex items-center justify-between">
              <span>二维码 (可传多张) *</span>
              <button className="text-blue-500 text-[13px] flex items-center gap-1 active:opacity-70 transition-opacity" onClick={handleAddQr}>
                 <Plus className="w-4 h-4"/>增加一张</button>
           </label>
           
           <div className="flex flex-col gap-4">
              {qrCodes.map((qr, index) => (
                 <div key={index} className="flex gap-3 bg-[#f8f9fa] dark:bg-[#2C2C2E] p-3 rounded-2xl relative border border-transparent focus-within:border-blue-500 transition-colors">
                    <div className="w-[100px] h-[100px] relative shrink-0 rounded-xl overflow-hidden bg-black/5 dark:bg-white/5 border border-black/5 dark:border-white/5">
                       <img src={qr.url} alt="" className="w-full h-full object-cover" />
                       <button 
                         onClick={() => handleRemoveQr(index)}
                         className="absolute right-1 top-1 w-6 h-6 bg-black/40 text-white rounded-full flex items-center justify-center backdrop-blur-md"
                       >
                         <X className="w-4 h-4" />
                       </button>
                    </div>
                    <div className="flex-1 flex flex-col justify-between">
                       <textarea
                         value={qr.description}
                         onChange={e => handleUpdateQrDescription(index, e.target.value)}
                         placeholder={t('community.auto_prop_n51fc90ee', '添加提示文案，例如: 群1已满，请加群2...')}
                         className="w-full h-full bg-transparent outline-none text-[14px] resize-none text-text-main placeholder:text-text-sub inline-block pt-1"
                       />
                    </div>
                 </div>
              ))}
              
              {qrCodes.length === 0 && (
                <div 
                  className="h-28 border-2 border-dashed border-black/20 dark:border-white/20 rounded-2xl flex flex-col items-center justify-center text-text-sub cursor-pointer active:bg-black/5 transition-colors gap-2"
                  onClick={handleAddQr}
                >
                  <UploadCloud className="w-8 h-8 opacity-50" />
                  <span className="text-[14px] font-medium opacity-80">点击上传第一张二维码</span>
                </div>
              )}
           </div>
        </div>

        <div className="px-4 py-8 mt-auto">
          <button 
            className={cn(
              "w-full h-12 rounded-full font-bold text-[16px] text-white flex items-center justify-center active:scale-[0.98] transition-all",
              isSubmitting || qrCodes.length === 0 || !name.trim() ? "bg-blue-300 pointer-events-none" : "bg-blue-500 shadow-md shadow-blue-500/20"
            )}
            onClick={handleSubmit}
          >{isSubmitting ? "保存中..." : (isEditMode ? "保存修改" : "立即创建")}</button>
        </div>

      </div>
    </div>
  );
};

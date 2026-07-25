import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router";
import { CommunityService } from "../services/CommunityService";
import { Community } from "../types";
import { cn, IconButton, showToast } from "@sdkwork/im-h5-commons";
import { ChevronLeft, Check } from "lucide-react";

export const AVAILABLE_TABS = [
  { id: 'feeds', name: '动态' },
  { id: 'resources', name: '资源' },
  { id: 'groups', name: '群组' },
  { id: 'news', name: '新闻' },
  { id: 'docs', name: '文档' },
  { id: 'repos', name: '开源' },
  { id: 'software', name: '软件' }
];

export const CommunityEditTabs: React.FC = () => {
  const { t } = useTranslation();
const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();

  const [community, setCommunity] = useState<Community | null>(null);
  const [selectedTabs, setSelectedTabs] = useState<string[]>([]);
  const [isSaving, setIsSaving] = useState(false);

  useEffect(() => {
    if (id) {
      CommunityService.getCommunityById(id).then(c => {
        if (c) {
          setCommunity(c);
          setSelectedTabs(c.tabs || ['feeds', 'resources', 'groups']);
        }
      });
    }
  }, [id]);

  const toggleTab = (tabId: string) => {
  if (selectedTabs.includes(tabId)) {
      setSelectedTabs(selectedTabs.filter(t => t !== tabId));
    } else {
      setSelectedTabs([...selectedTabs, tabId]);
    }
  };

  const handleSave = async () => {
    if (!id || !community) return;
    if (selectedTabs.length === 0) return showToast(t('community.auto_fn_7fd03543', '至少选择一个展示模块'));

    setIsSaving(true);
    try {
       await CommunityService.updateCommunity(id, { tabs: selectedTabs });
       showToast(t('community.auto_fn_25b0deea', '保存成功'));
       navigate(-1);
    } catch {
       showToast(t('community.auto_fn_25b0066f', '保存失败'));
    } finally {
       setIsSaving(false);
    }
  };

  return (
    <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black relative text-text-main">
       <header className="h-[56px] px-4 flex items-center justify-between shrink-0 pt-safe bg-white dark:bg-[#1C1C1E] z-20 shadow-sm relative">
          <div className="absolute left-4 z-10">
             <IconButton icon={<ChevronLeft className="w-6 h-6 text-text-main" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
          </div>
          <h1 className="text-[17px] font-semibold flex-1 text-center">{t('community.auto_2bcd0b9b', '展示模块')}</h1>
          <div className="absolute right-4 z-10">
             <button onClick={handleSave} disabled={isSaving || !community} className="text-blue-500 font-medium text-[15px] active:opacity-70 disabled:opacity-50">{t('community.auto_a071b', '保存')}</button>
          </div>
       </header>

       <div className="flex-1 overflow-y-auto pb-safe pt-4">
          <div className="bg-white dark:bg-[#1C1C1E] pl-4 border-y border-black/5 dark:border-white/5">
            {AVAILABLE_TABS.map((tab, index) => {
              const checked = selectedTabs.includes(tab.id);
              return (
                <div 
                  key={tab.id}
                  onClick={() => toggleTab(tab.id)}
                  className={cn(
                    "flex items-center justify-between py-3 pr-4 cursor-pointer active:opacity-70 transition-opacity",
                    index < AVAILABLE_TABS.length - 1 && "border-b border-black/5 dark:border-white/5"
                  )}
                >
                  <span className="text-[16px]">{tab.name}</span>
                  <div className={cn(
                    "w-5 h-5 rounded-full flex items-center justify-center transition-colors border",
                    checked
                      ? "bg-blue-500 border-blue-500"
                      : "border-text-sub bg-transparent"
                  )}>
                    {checked && <Check className="w-3.5 h-3.5 text-white" />}
                  </div>
                </div>
              );
            })}
          </div>
          <p className="mt-2 text-[13px] text-text-sub px-4">{t('community.auto_n6f4676f8', '勾选的模块将会出现在圈子主页的顶部导航中。')}</p>
       </div>
    </div>
  );
};

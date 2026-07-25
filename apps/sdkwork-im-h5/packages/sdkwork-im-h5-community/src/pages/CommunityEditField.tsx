import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useParams, useNavigate, useSearchParams } from "react-router";
import { CommunityService } from "../services/CommunityService";
import { Community } from "../types";
import { cn, IconButton, showToast } from "@sdkwork/im-h5-commons";
import { ChevronLeft } from "lucide-react";

export const CommunityEditField: React.FC = () => {
  const { t } = useTranslation();
const { id } = useParams<{ id: string }>();
  const [searchParams] = useSearchParams();
  const field = searchParams.get('field') as 'name' | 'description' | 'tags' || 'name';
  const navigate = useNavigate();

  const [community, setCommunity] = useState<Community | null>(null);
  const [value, setValue] = useState("");
  const [isSaving, setIsSaving] = useState(false);

  useEffect(() => {
    if (id) {
      CommunityService.getCommunityById(id).then(c => {
        if (c) {
          setCommunity(c);
          if (field === 'tags') {
            setValue(c.tags.join(" "));
          } else {
            setValue(c[field] || "");
          }
        }
      });
    }
  }, [id, field]);

  const handleSave = async () => {
    if (!id || !community) return;
    if (field === 'name' && !value.trim()) return showToast(t('community.auto_fn_48fe3213', '名称不能为空'));

    setIsSaving(true);
    try {
       const updates: Partial<Community> = {};
       if (field === 'tags') {
         updates.tags = value.split(" ").filter(t => t.trim());
       } else {
         updates[field] = value.trim();
       }
       await CommunityService.updateCommunity(id, updates);
       showToast(t('community.auto_fn_25b0deea', '保存成功'));
       navigate(-1);
    } catch {
       showToast(t('community.auto_fn_25b0066f', '保存失败'));
    } finally {
       setIsSaving(false);
    }
  };

  const titles = {
    name: '圈子名称',
    description: '圈子简介',
    tags: '圈子标签'
  };

  return (
    <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black relative text-text-main">
       <header className="h-[56px] px-4 flex items-center justify-between shrink-0 pt-safe bg-white dark:bg-[#1C1C1E] z-20 shadow-sm relative">
          <div className="absolute left-4 z-10">
             <IconButton icon={<ChevronLeft className="w-6 h-6 text-text-main" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
          </div>
          <h1 className="text-[17px] font-semibold flex-1 text-center">{titles[field]}</h1>
          <div className="absolute right-4 z-10">
             <button onClick={handleSave} disabled={isSaving || !community} className="text-blue-500 font-medium text-[15px] active:opacity-70 disabled:opacity-50">{t('community.auto_a071b', '保存')}</button>
          </div>
       </header>

       <div className="flex-1 overflow-y-auto pb-safe pt-4">
          <div className="bg-white dark:bg-[#1C1C1E] px-4 py-2 border-y border-black/5 dark:border-white/5">
            {field === 'description' ? (
              <textarea
                value={value}
                onChange={e => setValue(e.target.value)}
                className="w-full bg-transparent py-2 outline-none text-[16px] resize-none h-32 text-text-main"
                placeholder={`请输入${titles[field]}...`}
                autoFocus
              />
            ) : (
              <input
                value={value}
                onChange={e => setValue(e.target.value)}
                className="w-full bg-transparent py-2 outline-none text-[16px] text-text-main"
                placeholder={field === 'tags' ? "标签用空格分隔" : `请输入${titles[field]}`}
                autoFocus
              />
            )}
          </div>
          {field === 'tags' && (
            <div className="px-4 py-2 text-[13px] text-text-sub">{t('community.auto_n27f80d1e', '多个标签请用空格隔开，例如：科技 创业 互联网')}</div>
          )}
       </div>
    </div>
  );
};

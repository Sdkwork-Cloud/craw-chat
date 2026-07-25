import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useNavigate, useParams } from "react-router";
import { WorkService, type Work } from "../services/WorkService";
import { cn, showToast } from "@sdkwork/im-h5-commons";
import { PageLayout } from "../components/PageLayout";

export const WorkEditPage = () => {
  const { t } = useTranslation();
const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [work, setWork] = useState<Work | null>(null);
  const [title, setTitle] = useState("");
  const [coverUrl, setCoverUrl] = useState("");
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (id) {
      WorkService.getMyWorks().then(works => {
        const found = works.find(w => w.id === id);
        if (found) {
          setWork(found);
          setTitle(found.title);
          setCoverUrl(found.coverUrl);
        }
        setLoading(false);
      });
    }
  }, [id]);

  const handleSave = async () => {
    if (!title.trim()) {
      showToast(t('user.auto_fn_be5b58b', '提请填写作品标题'));
      return;
    }
    if (work) {
      try {
        await WorkService.updateWork(work.id, { title, coverUrl });
        showToast(t('user.auto_fn_25ddaeda', '修改成功'));
        navigate(-1);
      } catch {
         showToast(t('user.auto_fn_25dcd65f', '修改失败'));
      }
    }
  };

  if (loading) return null;
  if (!work) return (
     <PageLayout title={t('user.auto_prop_3bea0160', '编辑作品')}>
       <div className="flex-1 flex items-center justify-center text-text-sub">{t('user.auto_n243721ff', '未找到作品')}</div>
     </PageLayout>
  );

  return (
    <PageLayout 
       title={t('user.auto_prop_3bea0160', '编辑作品')}
       rightElement={
          <button 
             onClick={handleSave}
             className="px-4 py-1.5 bg-primary-blue text-white rounded-full text-[13px] font-medium"
          >{t('user.auto_a071b', '保存')}</button>
       }
    >
      <div className="p-4 flex flex-col gap-6">
        <div className="flex flex-col gap-2">
           <label className="text-[14px] text-text-sub font-medium">{t('user.auto_255d0726', '作品封面')}</label>
           <div 
             className="w-full aspect-video bg-chat-other-bg rounded-xl border border-border-color overflow-hidden relative cursor-pointer"
             onClick={() => {
                const newUrl = prompt("输入新的封面图片地址", coverUrl);
                if (newUrl) setCoverUrl(newUrl);
             }}
           >
             {coverUrl ? (
                <img src={coverUrl} className="w-full h-full object-cover" />
             ) : (
                <div className="flex-1 h-full flex items-center justify-center text-text-sub text-[13px]">{t('user.auto_1f270253', '点击设置封面')}</div>
             )}
             <div className="absolute inset-0 bg-black/30 flex items-center justify-center opacity-0 hover:opacity-100 transition-opacity">
                <span className="text-white text-[13px] font-medium">{t('user.auto_304d92ef', '更换封面')}</span>
             </div>
           </div>
        </div>
        
        <div className="flex flex-col gap-2">
           <label className="text-[14px] text-text-sub font-medium">{t('user.auto_255e7d16', '作品标题')}</label>
           <input 
              value={title}
              onChange={e => setTitle(e.target.value)}
              className="w-full bg-chat-other-bg border border-border-color rounded-xl px-4 py-3 outline-none focus:border-primary-blue transition-colors text-[15px] text-text-main"
              placeholder={t('user.auto_prop_69a34051', '为你的作品起个好名字')}
           />
        </div>
      </div>
    </PageLayout>
  );
};

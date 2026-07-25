import { useTranslation } from "react-i18next";
import React from "react";
import { Resource } from "../types";
import { IconButton, showToast } from "@sdkwork/im-h5-commons";
import { FileText, Download } from "lucide-react";

interface ResourceListProps {
  resources: Resource[];
}

export const ResourceList: React.FC<ResourceListProps> = ({ resources }) => {
  const { t } = useTranslation();
return (
    <div className="pb-24 flex flex-col bg-white dark:bg-[#1C1C1E]">
      {resources.map(res => (
        <div key={res.id} className="bg-white dark:bg-[#1C1C1E] p-4 flex gap-3 items-center border-b border-black/5 dark:border-white/5 active:bg-black/5 dark:active:bg-white/5 transition-colors">
          <div className="w-12 h-12 rounded-xl bg-orange-500/10 flex items-center justify-center shrink-0">
            <FileText className="w-6 h-6 text-orange-500" />
          </div>
          <div className="flex-1 overflow-hidden">
            <div className="text-[15px] font-bold text-text-main truncate mb-1">{res.title}</div>
            <div className="flex items-center gap-2 text-[12px] text-text-sub">
              <span className="uppercase px-1 border border-black/10 rounded">{res.type}</span>
              {res.size && <span>{res.size}</span>}
              <span>{t('community.auto_4e0f68b', '· {res.uploadedBy}分享')}</span>
            </div>
          </div>
          <IconButton 
            icon={<Download className="w-5 h-5 text-blue-500" />}
            className="bg-blue-500/10 shrink-0 w-10 h-10"
            onClick={() => showToast(t('community.auto_fn_n40cce478', '资源保存中...'))}
          />
        </div>
      ))}
      {resources.length === 0 && (
        <div className="h-40 flex items-center justify-center text-text-sub">{t('community.auto_3028e9ea', '暂无资源')}</div>
      )}
    </div>
  );
};

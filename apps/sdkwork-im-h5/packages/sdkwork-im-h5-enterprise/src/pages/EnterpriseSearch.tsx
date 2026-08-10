import { useTranslation } from "react-i18next";
import React, { useState } from "react";
import { PageLayout } from "@sdkwork/im-h5-commons";
import { Search, X } from "lucide-react";

export const EnterpriseSearch = () => {
  const { t } = useTranslation();
const [query, setQuery] = useState("");

  const TRENDING = ["科技", "电商服务", "大米批发", "新媒体代运营", "物流", "招聘前端开发"];

  return (
    <PageLayout title={t('enterprise.auto_prop_c9c86', '搜索')} bgClass="bg-chat-other-bg">
      <div className="p-4 flex flex-col items-center">
        <div className="w-full bg-input-bg rounded-xl flex items-center px-4 py-3 border border-transparent focus-within:border-primary-blue/30 transition-colors">
          <Search className="w-5 h-5 text-text-sub shrink-0" />
          <input 
            className="flex-1 bg-transparent border-none outline-none text-[15px] text-text-main ml-2" 
            placeholder={t('enterprise.auto_prop_66466fbc', '搜索企业、供应或职位')}
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            autoFocus
          />
          {query && (
            <div className="w-6 h-6 flex items-center justify-center shrink-0 cursor-pointer text-text-sub bg-black/5 dark:bg-white/5 rounded-full ml-2" onClick={() => setQuery("")}>
              <X className="w-3.5 h-3.5" />
            </div>
          )}
        </div>

        {!query && (
          <div className="w-full mt-6 flex flex-col self-start">
             <h3 className="text-[14px] font-bold text-text-main mb-3">{t('enterprise.auto_3594a461', '热门搜索')}</h3>
             <div className="flex flex-wrap gap-3">
                {TRENDING.map((tag) => (
                  <span 
                    key={tag} 
                    className="px-3 py-1.5 bg-input-bg rounded-full text-[13px] text-text-sub cursor-pointer active:scale-95 transition-transform"
                    onClick={() => setQuery(tag)}
                  >
                    {tag}
                  </span>
                ))}
             </div>
          </div>
        )}

        {query && (
          <div className="w-full mt-12 flex flex-col items-center justify-center text-text-sub gap-2">
             <Search className="w-10 h-10 opacity-30" />
             <span className="text-[14px]">{t('enterprise.auto_6a2c51b5', '搜索 "{query}" 相关内容')}</span>
          </div>
        )}
      </div>
    </PageLayout>
  );
};

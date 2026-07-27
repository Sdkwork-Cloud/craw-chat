import React from "react";

interface GlobalSearchQuickTagsProps {
  t: (key: string) => string;
}

export const GlobalSearchQuickTags: React.FC<GlobalSearchQuickTagsProps> = ({ t }) => {
  return (
    <div className="p-6">
      <h3 className="text-center text-[13px] text-text-sub mb-6">
        {t('chat.search.search_specific')}
      </h3>
      <div className="grid grid-cols-3 gap-y-6 text-center">
        <span className="text-[15px] text-primary-blue active:opacity-70 cursor-pointer">
          {t('chat.search.moments')}
        </span>
        <span className="text-[15px] text-primary-blue active:opacity-70 cursor-pointer border-x border-border-color">
          {t('chat.search.article')}
        </span>
        <span className="text-[15px] text-primary-blue active:opacity-70 cursor-pointer">
          {t('chat.search.official_account')}
        </span>
        <span className="text-[15px] text-primary-blue active:opacity-70 cursor-pointer">
          {t('chat.search.miniapp')}
        </span>
        <span className="text-[15px] text-primary-blue active:opacity-70 cursor-pointer border-x border-border-color">
          {t('chat.search.music')}
        </span>
        <span className="text-[15px] text-primary-blue active:opacity-70 cursor-pointer">
          {t('chat.search.sticker')}
        </span>
      </div>
    </div>
  );
};

import React from "react";
import { useTranslation } from "react-i18next";

export const CourseBanner: React.FC = () => {
  const { t } = useTranslation();

  return (
    <div className="p-4 bg-white dark:bg-[#1C1C1E]">
      <div className="bg-gradient-to-r from-blue-600 to-indigo-600 rounded-2xl p-6 text-white flex flex-col justify-end shadow-sm relative overflow-hidden h-[160px] cursor-pointer">
        <div className="absolute inset-0 bg-[url('https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/cb/800x400.png')] opacity-40 mix-blend-overlay object-cover" />
        <div className="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent" />
        <div className="absolute top-3 right-3 bg-white/20 backdrop-blur-md px-2 py-0.5 rounded text-[10px] font-medium border border-white/10">
          {t('course.auto_250d7220', '主打推荐')}
        </div>
        <div className="relative z-10">
          <h2 className="text-[20px] font-bold mb-1 leading-tight">
            {t('course.auto_n22f09a3c', '2026 年度跨端技术合集')}
          </h2>
          <p className="text-[13px] text-white/80 line-clamp-1">
            {t('course.auto_43bffece', '掌握前沿开发趋势，提升核心竞争力')}
          </p>
        </div>
      </div>
      {/* Dots */}
      <div className="flex justify-center gap-1.5 mt-3">
        <div className="w-1.5 h-1.5 rounded-full bg-blue-500" />
        <div className="w-1.5 h-1.5 rounded-full bg-black/10 dark:bg-white/20" />
        <div className="w-1.5 h-1.5 rounded-full bg-black/10 dark:bg-white/20" />
      </div>
    </div>
  );
};

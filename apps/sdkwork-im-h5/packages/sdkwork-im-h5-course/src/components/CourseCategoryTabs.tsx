import React from "react";

export interface CourseCategory {
  id: string;
  name: string;
}

interface CourseCategoryTabsProps {
  categories: CourseCategory[];
  activeTab: string;
  onTabChange: (id: string) => void;
}

export const CourseCategoryTabs: React.FC<CourseCategoryTabsProps> = ({
  categories,
  activeTab,
  onTabChange,
}) => {
  return (
    <div className="bg-white dark:bg-[#1C1C1E] px-4 py-3 border-b border-black/5 dark:border-white/5 sticky top-0 z-10 flex overflow-x-auto hide-scrollbar gap-4">
      {categories.map((cat) => (
        <div
          key={cat.id}
          onClick={() => onTabChange(cat.id)}
          className={`whitespace-nowrap pb-2 text-[15px] font-medium transition-colors cursor-pointer border-b-2 ${
            activeTab === cat.id
              ? "text-blue-500 border-blue-500"
              : "text-text-sub border-transparent"
          }`}
        >
          {cat.name}
        </div>
      ))}
    </div>
  );
};

import { useTranslation } from "react-i18next";
import React, { useRef, useEffect, ReactNode } from 'react';
import { cn } from '../utils/cn';

export interface TabItem {
  id: string;
  name?: string;
  label?: string; // allow both name or label
}

export interface TabsProps {
  tabs: TabItem[];
  activeTab: string;
  onChange: (tabId: string) => void;
  className?: string; // Optional wrapper class
  itemClassName?: string; // Optional class for individual tab
  activeItemClassName?: string; // Optional class when active
}

export const Tabs: React.FC<TabsProps> = ({
  tabs,
  activeTab,
  onChange,
  className,
  itemClassName,
  activeItemClassName
}) => {
  const { t } = useTranslation();
const scrollRef = useRef<HTMLDivElement>(null);
  
  useEffect(() => {
    if (scrollRef.current) {
       const activeNode = scrollRef.current.querySelector('[data-active="true"]') as HTMLElement;
       if (activeNode) {
         // Use a more intelligent scroll behavior
         activeNode.scrollIntoView({ behavior: 'smooth', block: 'nearest', inline: 'center' });
       }
    }
  }, [activeTab]);

  return (
    <div className={cn("flex w-full overflow-x-auto no-scrollbar relative whitespace-nowrap scroll-smooth", className)} ref={scrollRef}>
      {tabs.map((tab) => {
        const isActive = activeTab === tab.id;
        const labelText = tab.label || tab.name;
        
        return (
          <div
            key={tab.id}
            data-active={isActive ? "true" : "false"}
            onClick={() => onChange(tab.id)}
            className={cn(
              "relative px-1 py-3 text-[15px] cursor-pointer transition-colors shrink-0",
              isActive ? "font-semibold text-text-main" : "text-text-sub",
              itemClassName,
              isActive && activeItemClassName
            )}
          >
            {labelText}
            {isActive && (
              <div className="absolute bottom-0 inset-x-0 mx-auto w-4 h-[3px] bg-blue-500 rounded-t-full" />
            )}
          </div>
        );
      })}
    </div>
  );
};

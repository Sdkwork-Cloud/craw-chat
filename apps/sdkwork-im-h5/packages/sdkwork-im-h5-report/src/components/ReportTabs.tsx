import React from 'react';
import { useTranslation } from 'react-i18next';
import { cn } from '@sdkwork/im-h5-commons';

interface ReportTabsProps {
  activeTab: string;
  setActiveTab: (tab: string) => void;
}

export const ReportTabs: React.FC<ReportTabsProps> = ({ activeTab, setActiveTab }) => {
  const { t } = useTranslation();
const tabs = [
    { id: '待我查阅', label: t('report.tabs.pending') },
    { id: '我发出的', label: t('report.tabs.sent') },
    { id: '抄送我的', label: t('report.tabs.cc') },
  ];

  return (
    <div className="flex bg-chat-other-bg rounded-xl shadow-sm mb-4 px-2 py-1">
      {tabs.map((tab) => (
        <button
          key={tab.id}
          className={cn(
            "flex-1 text-[15px] py-2.5 relative text-center transition-colors rounded-lg",
            activeTab === tab.id
              ? "text-primary-blue font-medium bg-primary-blue/5"
              : "text-text-sub",
          )}
          onClick={() => setActiveTab(tab.id)}
        >
          {tab.label}
        </button>
      ))}
    </div>
  );
};

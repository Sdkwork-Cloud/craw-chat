import React from 'react';
import { cn } from '@sdkwork/im-h5-commons';
import { useTranslation } from 'react-i18next';

export const ApprovalTabs: React.FC<{
  activeTab: string;
  setActiveTab: (tab: string) => void;
}> = ({ activeTab, setActiveTab }) => {
  const { t } = useTranslation();
  
  const tabs = [
    { key: 'pending', label: t('approval.tabs.pending') },
    { key: 'initiated', label: t('approval.tabs.initiated') },
    { key: 'cc', label: t('approval.tabs.cc') }
  ];

  return (
    <div className="flex bg-chat-other-bg rounded-xl shadow-sm mb-4 px-2 py-1">
      {tabs.map((tab) => (
        <button
          key={tab.key}
          className={cn(
            "flex-1 text-[15px] py-2.5 relative text-center transition-colors rounded-lg",
            activeTab === tab.key
              ? "text-primary-blue font-medium bg-primary-blue/5"
              : "text-text-sub"
          )}
          onClick={() => setActiveTab(tab.key)}
        >
          {tab.label}
        </button>
      ))}
    </div>
  );
};

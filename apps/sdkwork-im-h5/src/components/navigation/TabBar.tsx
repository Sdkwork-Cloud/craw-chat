import React from 'react';
import { useNavigate, useLocation } from 'react-router';
import { MessageCircle, Bot, LayoutGrid, Compass, UserRound } from 'lucide-react';
import { cn } from '@sdkwork/im-h5-commons';
import { useTranslation } from 'react-i18next';

// Custom filled SVG variants for the active tabs
const TabSolidMessage = ({ className }: { className?: string; strokeWidth?: number | string }) => (
  <svg viewBox="0 0 24 24" className={className} stroke="none">
    <path d="M7.9 20A9 9 0 1 0 4 16.1L2 22Z" fill="currentColor" />
  </svg>
);

const TabSolidBot = ({ className }: { className?: string; strokeWidth?: number | string }) => (
  <svg viewBox="0 0 24 24" className={className} fill="none">
    <path d="M12 2v6" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
    <path d="M8 8V6a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
    <rect x="3" y="10" width="18" height="12" rx="2" fill="currentColor" stroke="currentColor" strokeWidth="2" />
    <circle cx="8.5" cy="15.5" r="1.5" fill="white" />
    <circle cx="15.5" cy="15.5" r="1.5" fill="white" />
    <path d="M8 15.5h.01M16 15.5h.01" stroke="currentColor" strokeWidth="0" />
  </svg>
);

const TabSolidWorkspace = ({ className }: { className?: string; strokeWidth?: number | string }) => (
  <svg viewBox="0 0 24 24" className={className} stroke="none">
    <rect x="3" y="3" width="7" height="7" rx="1" fill="currentColor" />
    <rect x="14" y="3" width="7" height="7" rx="1" fill="currentColor" />
    <rect x="14" y="14" width="7" height="7" rx="1" fill="currentColor" />
    <rect x="3" y="14" width="7" height="7" rx="1" fill="currentColor" />
  </svg>
);

const TabSolidDiscover = ({ className }: { className?: string; strokeWidth?: number | string }) => (
  <svg viewBox="0 0 24 24" className={className} fill="none">
    <circle cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="2" fill="transparent" />
    <polygon points="16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76" fill="currentColor" />
  </svg>
);

const TabSolidUser = ({ className }: { className?: string; strokeWidth?: number | string }) => (
  <svg viewBox="0 0 24 24" className={className} fill="none">
    <circle cx="12" cy="7" r="5" fill="currentColor" />
    <path d="M20 21a8 8 0 0 0-16 0" fill="currentColor" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
  </svg>
);

export const TabBar: React.FC = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const { t } = useTranslation();

  const tabs = [
    { id: 'chat', outline: MessageCircle, solid: TabSolidMessage, label: t('common.tabs.chat', '聊天'), path: '/' },
    { id: 'agents', outline: Bot, solid: TabSolidBot, label: t('common.tabs.agents', '智能体'), path: '/agents' },
    { id: 'workspace', outline: LayoutGrid, solid: TabSolidWorkspace, label: t('common.tabs.workspace', '工作台'), path: '/workspace' },
    { id: 'discover', outline: Compass, solid: TabSolidDiscover, label: t('common.tabs.discover', '发现'), path: '/discover' },
    { id: 'me', outline: UserRound, solid: TabSolidUser, label: t('common.tabs.me', '我'), path: '/me' },
  ];

  const mainPaths = tabs.map(t => t.path);
  
  // Only show tab bar on main root pages
  if (!mainPaths.includes(location.pathname)) return null;

  return (
    <nav className="w-full pb-safe pt-2 flex justify-around items-start glass-tab-bar z-40 shrink-0 absolute bottom-0 left-0">
      {tabs.map((tab) => {
        const isActive = location.pathname === tab.path || (tab.path === '/' && location.pathname === '');
        const Icon = isActive ? tab.solid : tab.outline;
        return (
          <div
            key={tab.id}
            onClick={() => navigate(tab.path)}
            className={cn(
              "flex flex-col items-center gap-1 text-[10px] cursor-pointer transition-colors mb-1",
              isActive ? "text-primary-blue" : "text-text-sub"
            )}
          >
            <Icon 
              className={cn("w-6 h-6 transition-all", isActive ? "opacity-100 scale-110" : "opacity-50 scale-100")} 
              strokeWidth={isActive ? undefined : 1.5}
            />
            <span>{tab.label}</span>
          </div>
        );
      })}
    </nav>
  );
};

import { AdminDashboard } from '@sdkwork/im-admin-dashboard';
import React from 'react';
import { 
  Building2, Server, CreditCard, ShieldAlert,
  Activity, Settings, Users, Database,
  Globe, LogOut, BarChart3, Search, Bell
} from 'lucide-react';
import { cn, I18nProvider, useTranslation } from '@sdkwork/im-pc-commons';
import { PlatformTenants, AdminUsers } from '@sdkwork/im-admin-tenants';
import { InfrastructureStatus, AdminBilling } from '@sdkwork/im-admin-infrastructure';
import { AdminAnnouncements, AdminCompliance, AdminSettings } from '@sdkwork/im-admin-operations';
import { Routes, Route, useNavigate, useLocation, Navigate } from 'react-router-dom';

type AdminPage = 'overview' | 'tenants' | 'users' | 'infrastructure' | 'billing' | 'announcements' | 'compliance' | 'settings';

export const AdminLayoutInner: React.FC<{ onSwitchToClient?: () => void }> = ({ onSwitchToClient }) => {
  const navigate = useNavigate();
  const location = useLocation();
  const { t, language, setLanguage } = useTranslation();

  const navItems = [
    { id: 'overview', icon: Activity, label: t('admin.nav.overview'), path: '/admin/overview' },
    { id: 'tenants', icon: Building2, label: t('admin.nav.tenants'), path: '/admin/tenants' },
    { id: 'users', icon: Users, label: t('admin.nav.users'), path: '/admin/users' },
    { id: 'infrastructure', icon: Server, label: t('admin.nav.infrastructure'), path: '/admin/infrastructure' },
    { id: 'billing', icon: CreditCard, label: t('admin.nav.billing'), path: '/admin/billing' },
    { id: 'announcements', icon: Bell, label: t('admin.nav.announcements'), path: '/admin/announcements' },
    { id: 'compliance', icon: ShieldAlert, label: t('admin.nav.compliance'), path: '/admin/compliance' },
    { id: 'settings', icon: Settings, label: t('admin.nav.settings'), path: '/admin/settings' },
  ] as const;

  const currentPath = location.pathname.replace(/\/$/, '');
  const activeNavItem = navItems.find(item => item.path === currentPath) || navItems[0];

  return (
    <div className="flex h-screen w-full bg-admin-bg-root text-admin-text-main font-sans">
      {/* Dark Sidebar for Super Admin */}
      <div className="w-[260px] bg-admin-bg-panel border-r border-admin-border flex flex-col z-10 shrink-0">
        <div className="h-[64px] flex items-center px-6 border-b border-admin-border">
          <div className="w-8 h-8 bg-gradient-to-br from-indigo-500 to-purple-600 rounded-lg flex items-center justify-center text-white font-black text-lg mr-3 shadow-lg shadow-indigo-500/20">
            A
          </div>
          <div>
            <div className="font-bold text-[15px] text-white leading-tight tracking-wide">{t('admin.title')}</div>
            <div className="text-[11px] text-indigo-400 font-medium tracking-widest uppercase">{t('admin.subtitle')}</div>
          </div>
        </div>

        <div className="flex-1 overflow-y-auto py-6 px-3 flex flex-col gap-1 custom-scrollbar">
          <div className="text-[10px] font-bold text-gray-500 px-3 pb-2 uppercase tracking-widest">{t('admin.nav.operations')}</div>
          {navItems.map((item) => (
            <button
              key={item.id}
              onClick={() => navigate(item.path)}
              className={cn(
                "w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm transition-all focus:outline-none",
                activeNavItem.id === item.id 
                  ? "bg-white/10 text-white font-medium shadow-sm border border-white/5" 
                  : "text-gray-400 hover:bg-white/5 hover:text-gray-200"
              )}
            >
              <item.icon size={18} className={cn(activeNavItem.id === item.id ? "text-indigo-400" : "text-gray-500")} />
              {item.label}
            </button>
          ))}
        </div>

        <div className="p-4 border-t border-admin-border bg-admin-bg-root/50">
          <button 
            className="w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm text-admin-text-muted hover:bg-admin-bg-hover hover:text-admin-text-main transition-all focus:outline-none border border-transparent hover:border-admin-border-subtle"
            onClick={onSwitchToClient}
          >
            <LogOut size={18} className="text-admin-text-muted" />
            {t('admin.nav.switch')}
          </button>
        </div>
      </div>

      {/* Main Content Area */}
      <div className="flex-1 flex flex-col min-w-0 bg-admin-bg-root">
        {/* Top Navbar */}
        <header className="h-[64px] bg-admin-bg-panel border-b border-admin-border flex items-center justify-between px-8 shrink-0 shadow-sm">
          <h1 className="text-lg font-semibold text-admin-text-main tracking-wide">
            {activeNavItem.label}
          </h1>

          <div className="flex items-center gap-6">
            <div className="relative group">
              <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500 group-focus-within:text-indigo-400 transition-colors" />
              <input 
                type="text" 
                placeholder={t('admin.header.search')}
                className="w-64 bg-admin-input-bg border border-admin-border rounded-lg py-1.5 pl-9 pr-4 text-sm text-admin-text-main focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-all placeholder:text-admin-text-muted"
              />
            </div>
            
            <button 
              onClick={() => setLanguage(language === 'zh' ? 'en' : 'zh')}
              className="px-3 py-1 text-xs font-medium border border-admin-border rounded-md hover:bg-admin-bg-hover text-admin-text-main transition-colors uppercase"
            >
              {language === 'zh' ? 'EN' : '中'}
            </button>

            <button className="relative p-2 text-admin-text-muted hover:text-admin-text-main hover:bg-admin-bg-hover rounded-full transition-colors focus:outline-none">
              <Bell size={18} />
              <span className="absolute top-1.5 right-1.5 w-2 h-2 bg-rose-500 border-2 border-admin-bg-panel rounded-full shadow-[0_0_8px_rgba(244,63,94,0.6)]"></span>
            </button>

            <div className="flex items-center gap-3 pl-6 border-l border-admin-border cursor-pointer group">
              <div className="text-right hidden md:block">
                <div className="text-sm font-medium text-gray-200 group-hover:text-white transition-colors">{t('admin.header.root')}</div>
                <div className="text-[11px] text-gray-500 uppercase tracking-wider">{t('admin.header.system')}</div>
              </div>
              <div className="w-9 h-9 rounded-full bg-gradient-to-tr from-gray-800 to-gray-700 border border-white/20 flex items-center justify-center shadow-inner overflow-hidden">
                <ShieldAlert size={16} className="text-gray-300" />
              </div>
            </div>
          </div>
        </header>

        {/* Dynamic Page Content */}
        <main className="flex-1 overflow-x-hidden overflow-y-auto p-4 sm:p-6 custom-scrollbar relative">
          {/* Subtle background glow */}
          <div className="absolute top-0 left-1/2 -translate-x-1/2 w-full max-w-4xl h-96 bg-indigo-600/5 blur-[120px] rounded-full pointer-events-none" />
          
          <div className="w-full max-w-full mx-auto relative z-10 h-full flex flex-col">
            <Routes>
              <Route path="/" element={<Navigate to="/admin/overview" replace />} />
              <Route path="overview" element={<AdminDashboard />} />
              <Route path="tenants" element={<PlatformTenants />} />
              <Route path="users" element={<AdminUsers />} />
              <Route path="infrastructure" element={<InfrastructureStatus />} />
              <Route path="billing" element={<AdminBilling />} />
              <Route path="announcements" element={<AdminAnnouncements />} />
              <Route path="compliance" element={<AdminCompliance />} />
              <Route path="settings" element={<AdminSettings />} />
              <Route path="*" element={
                <div className="w-full h-64 border border-admin-border border-dashed rounded-2xl flex flex-col items-center justify-center text-admin-text-muted bg-admin-bg-panel/50 backdrop-blur-sm">
                  <Settings size={32} className="text-gray-600 mb-3" />
                  <p className="tracking-wide">{t('admin.under_construction')}</p>
                </div>
              } />
            </Routes>
          </div>
        </main>
      </div>
    </div>
  );
};

export const AdminLayout: React.FC<{ onSwitchToClient?: () => void }> = (props) => (
  <I18nProvider>
    <AdminLayoutInner {...props} />
  </I18nProvider>
);

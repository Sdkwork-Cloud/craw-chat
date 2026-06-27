import React, { useState, useEffect } from 'react';
import { Users, Search, Ban, Shield, Download, Mail, Activity, Monitor } from 'lucide-react';
import { cn, useTranslation } from '@sdkwork/im-pc-commons';
import { globalUserService, GlobalUser } from './services/GlobalUserService';

export const AdminUsers = () => {
  const [searchTerm, setSearchTerm] = useState('');
  const [statusFilter, setStatusFilter] = useState('All Global Statuses');
  const [users, setUsers] = useState<GlobalUser[]>([]);
  const [loading, setLoading] = useState(false);
  const [total, setTotal] = useState(0);
  const { t } = useTranslation();

  const fetchUsers = async () => {
    setLoading(true);
    try {
      const res = await globalUserService.getGlobalUsers({ search: searchTerm, status: statusFilter });
      setUsers(res.data);
      setTotal(res.total);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchUsers();
  }, [searchTerm, statusFilter]);

  const handleToggleBan = async (id: string, currentStatus: string) => {
    const newStatus = currentStatus === 'banned' ? 'active' : 'banned';
    await globalUserService.updateUserStatus(id, newStatus as any);
    fetchUsers();
  };

  return (
    <div className="bg-admin-bg-panel border border-admin-border rounded-2xl shadow-xl flex flex-col flex-1 min-h-0 h-full relative overflow-hidden">
      <div className="absolute top-0 left-0 w-full h-full overflow-hidden pointer-events-none">
        <div className="absolute top-0 right-1/3 w-[500px] h-[500px] bg-indigo-500/5 blur-[120px] rounded-full" />
      </div>

      {/* Header */}
      <div className="relative z-10 flex items-center justify-between p-6 border-b border-admin-border bg-admin-bg-root/50">
        <div>
          <h2 className="text-xl font-bold text-admin-text-main tracking-wide">{t('admin.users.title')}</h2>
          <p className="text-sm text-admin-text-muted mt-1">{t('admin.users.subtitle')}</p>
        </div>
        <button className="bg-admin-bg-hover hover:bg-admin-border-subtle border border-admin-border text-admin-text-main px-4 py-2 rounded-lg text-sm font-medium transition-colors flex items-center gap-2">
          <Download size={16} />
          {t('admin.users.export')}
        </button>
      </div>

      {/* Toolbar */}
      <div className="relative z-10 p-5 flex items-center justify-between border-b border-admin-border bg-admin-bg-root/30">
        <div className="flex items-center gap-4">
          <div className="relative group">
            <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-admin-text-muted group-focus-within:text-indigo-400 transition-colors" />
            <input 
              type="text" 
              placeholder={t('admin.users.search')} 
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="w-80 bg-admin-input-bg border border-admin-border rounded-lg py-2 pl-9 pr-4 text-sm text-admin-text-main focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-all placeholder:text-admin-text-muted"
            />
          </div>
          <select 
            className="bg-admin-bg-panel border border-admin-border text-sm text-admin-text-main rounded-lg px-3 py-2 outline-none cursor-pointer"
            value={statusFilter}
            onChange={(e) => setStatusFilter(e.target.value)}
          >
            <option value="All Global Statuses">{t('admin.users.filter.all')}</option>
            <option value="Active Accounts">{t('admin.users.filter.active')}</option>
            <option value="Banned Globally">{t('admin.users.filter.banned')}</option>
            <option value="Pending Verification">{t('admin.users.filter.pending')}</option>
          </select>
        </div>
      </div>

      {/* Global User Table */}
      <div className="relative z-10 flex-1 overflow-auto custom-scrollbar">
        <table className="w-full text-left border-collapse">
          <thead>
            <tr className="border-b border-admin-border text-[11px] uppercase tracking-widest text-admin-text-muted bg-admin-bg-root/80 backdrop-blur-sm sticky top-0 z-20">
              <th className="px-6 py-4 font-semibold w-16 text-center">
                <input type="checkbox" className="rounded border-admin-border bg-transparent text-indigo-500 focus:ring-indigo-500/50" />
              </th>
              <th className="px-6 py-4 font-semibold">{t('admin.users.col.identity')}</th>
              <th className="px-6 py-4 font-semibold">{t('admin.users.col.tenant')}</th>
              <th className="px-6 py-4 font-semibold">{t('admin.users.col.security')}</th>
              <th className="px-6 py-4 font-semibold">{t('admin.users.col.status')}</th>
              <th className="px-6 py-4 font-semibold text-right">{t('admin.users.col.actions')}</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-admin-border text-sm">
            {loading ? (
              <tr><td colSpan={6} className="px-6 py-8 text-center text-admin-text-muted">加载中...</td></tr>
            ) : users.length === 0 ? (
              <tr><td colSpan={6} className="px-6 py-8 text-center text-admin-text-muted">未找到相关人员</td></tr>
            ) : users.map(user => (
              <GlobalUserRow 
                key={user.id}
                id={user.id}
                uin={user.uin} name={user.name} email={user.email}
                tenant={user.tenant} security={user.security} status={user.status} 
                onToggleBan={() => handleToggleBan(user.id, user.status)}
              />
            ))}
          </tbody>
        </table>
      </div>
      
      {/* Footer Nav */}
      <div className="relative z-10 p-4 border-t border-admin-border flex items-center justify-between text-xs text-admin-text-muted bg-admin-bg-root/50">
        <div>{t('admin.users.footer')}</div>
      </div>
    </div>
  );
};

const GlobalUserRow = ({ id, uin, name, email, tenant, security, status, onToggleBan }: any) => {
  const { t } = useTranslation();

  const statusConfig = {
    active: { label: t('admin.users.status.active'), classes: 'text-emerald-400 bg-emerald-500/10 border-emerald-500/20' },
    banned: { label: t('admin.users.status.banned'), classes: 'text-rose-400 bg-rose-500/10 border-rose-500/20' },
    warning: { label: t('admin.users.status.warning'), classes: 'text-amber-400 bg-amber-500/10 border-amber-500/20' }
  };
  const currentStatus = statusConfig[status as keyof typeof statusConfig] || statusConfig.active;

  return (
    <tr className="hover:bg-admin-bg-hover transition-colors group">
      <td className="px-6 py-4 text-center">
        <input type="checkbox" className="rounded border-admin-border bg-transparent text-indigo-500 focus:ring-indigo-500/50" />
      </td>
      <td className="px-6 py-4">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-xl bg-admin-bg-root border border-admin-border flex items-center justify-center text-admin-text-main shadow-inner">
            <Users size={18} />
          </div>
          <div>
            <div className="font-semibold text-admin-text-main flex items-center gap-2">
              {name}
            </div>
            <div className="text-[11px] font-mono text-admin-text-muted mt-0.5 flex gap-2">
              <span>{uin}</span>
              <span className="opacity-50">|</span>
              <span>{email}</span>
            </div>
          </div>
        </div>
      </td>
      <td className="px-6 py-4 text-xs font-medium text-admin-text-main">{tenant}</td>
      <td className="px-6 py-4">
        <span className="flex items-center gap-1.5 text-xs text-admin-text-muted">
          <Shield size={12} className={security.includes('MFA') ? "text-emerald-500" : "text-amber-500"} />
          {security}
        </span>
      </td>
      <td className="px-6 py-4">
         <span className={cn("px-2.5 py-1 rounded-md text-[10px] font-mono uppercase tracking-wider border", currentStatus.classes)}>
          {currentStatus.label}
        </span>
      </td>
      <td className="px-6 py-4 text-right">
        <div className="flex items-center justify-end gap-2">
          <button className="p-1.5 text-admin-text-muted hover:text-indigo-400 hover:bg-admin-bg-root rounded-md transition-colors" title={t('admin.users.action.audit') || "Audit Trails"}>
            <Activity size={16} />
          </button>
          <button 
            className="p-1.5 text-admin-text-muted hover:text-rose-400 hover:bg-rose-500/10 rounded-md transition-colors" 
            title={t('admin.users.action.ban') || "Ban Platform Network"}
            onClick={onToggleBan}
          >
            <Ban size={16} className={status === 'banned' ? 'text-rose-500' : ''} />
          </button>
        </div>
      </td>
    </tr>
  );
};

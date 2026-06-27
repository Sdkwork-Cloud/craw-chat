import React, { useState, useEffect } from 'react';
import { Puzzle, Bot, Webhook, Key, ExternalLink, Plus, MoreHorizontal } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';
import { integrationService, IntegrationApp } from './services/IntegrationService';

export const ConsoleIntegrations = () => {
  const [searchTerm, setSearchTerm] = useState('');
  const [statusFilter, setStatusFilter] = useState('all');
  const [apps, setApps] = useState<IntegrationApp[]>([]);
  const [loading, setLoading] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  useEffect(() => {
    const fetchApps = async () => {
      setLoading(true);
      setErrorMessage(null);
      try {
        const res = await integrationService.getApps({ search: searchTerm, status: statusFilter });
        setApps(res.data);
      } catch (error) {
        setApps([]);
        setErrorMessage(error instanceof Error ? error.message : String(error));
      } finally {
        setLoading(false);
      }
    };
    fetchApps();
  }, [searchTerm, statusFilter]);

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <h2 className="text-lg font-bold text-console-text-main">应用与机器人</h2>
          <p className="text-sm text-console-text-muted mt-1">管理内部自建应用、第三方集成以及自定义机器人的访问权限</p>
        </div>
        <div className="flex gap-2">
          <button className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors flex items-center gap-2 shadow-sm">
            <Plus size={16} />
            创建应用 / 机器人
          </button>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
        {/* Left Sidebar Category */}
        <div className="lg:col-span-1 space-y-2">
          <button className="w-full text-left px-4 py-2.5 rounded-lg text-sm font-medium transition-colors bg-console-active-bg text-blue-600 border border-blue-200 dark:border-blue-500/20">
            内部直连应用 <span className="float-right text-xs bg-blue-100 dark:bg-blue-500/20 px-2 py-0.5 rounded-full">{apps.length}</span>
          </button>
          <button className="w-full text-left px-4 py-2.5 rounded-lg text-sm font-medium transition-colors text-console-text-main hover:bg-console-bg-hover">
            第三方集成
          </button>
          <button className="w-full text-left px-4 py-2.5 rounded-lg text-sm font-medium transition-colors text-console-text-main hover:bg-console-bg-hover">
            Webhook & API接入
          </button>
          <button className="w-full text-left px-4 py-2.5 rounded-lg text-sm font-medium transition-colors text-console-text-main hover:bg-console-bg-hover">
            应用审批流管理
          </button>
        </div>

        {/* Right Content */}
        <div className="lg:col-span-3 space-y-4">
          <div className="flex gap-4">
            <input 
              type="text" 
              placeholder="搜索应用或机器人名称..." 
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="flex-1 bg-console-input-bg border border-console-border rounded-lg py-2 px-3 text-sm text-console-text-main focus:ring-2 focus:ring-blue-500/50 outline-none transition-shadow"
            />
            <select 
              className="bg-console-bg-panel border border-console-border rounded-lg py-2 px-3 text-sm text-console-text-main outline-none"
              value={statusFilter}
              onChange={(e) => setStatusFilter(e.target.value)}
            >
              <option value="all">状态：全部</option>
              <option value="active">已启用</option>
              <option value="disabled">已禁用</option>
            </select>
          </div>

          <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
            {loading ? (
              <div className="col-span-1 sm:col-span-2 p-8 text-center text-console-text-muted text-sm">加载中...</div>
            ) : errorMessage ? (
              <div className="col-span-1 sm:col-span-2 p-8 text-center text-console-text-muted text-sm">{errorMessage}</div>
            ) : apps.length === 0 ? (
              <div className="col-span-1 sm:col-span-2 p-8 text-center text-console-text-muted text-sm">暂无应用及机器人</div>
            ) : apps.map(app => (
              <AppCard 
                key={app.id}
                name={app.name} 
                type={app.type} 
                desc={app.desc} 
                color={app.color}
                icon={app.iconType === 'Bot' ? Bot : app.iconType === 'Webhook' ? Webhook : Puzzle}
                status={app.status}
              />
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};

const AppCard = ({ name, type, desc, color, icon: Icon, status }: any) => {
  return (
    <div className="bg-console-bg-panel border border-console-border rounded-xl p-5 shadow-sm hover:border-blue-500/50 transition-colors group flex flex-col h-full">
      <div className="flex justify-between items-start mb-4">
        <div className="flex items-center gap-3">
          <div className={cn("w-10 h-10 rounded-xl flex items-center justify-center text-white shadow-sm", color)}>
            <Icon size={20} />
          </div>
          <div>
            <h4 className="font-semibold text-console-text-main">{name}</h4>
            <span className="text-[11px] font-medium text-console-text-muted bg-console-bg-root px-2 py-0.5 rounded-md border border-console-border">{type}</span>
          </div>
        </div>
        <button className="p-1 text-console-text-muted hover:text-blue-500 rounded-md transition-colors">
          <MoreHorizontal size={18} />
        </button>
      </div>
      
      <p className="text-xs text-console-text-muted mb-6 flex-1 line-clamp-2">
        {desc}
      </p>
      
      <div className="flex justify-between items-center pt-3 border-t border-console-border">
        {status === 'active' ? (
          <span className="flex items-center gap-1.5 text-[11px] font-medium text-emerald-600 dark:text-emerald-400">
            <div className="w-1.5 h-1.5 bg-emerald-500 rounded-full"></div> 已启用
          </span>
        ) : (
          <span className="flex items-center gap-1.5 text-[11px] font-medium text-console-text-muted">
            <div className="w-1.5 h-1.5 bg-gray-400 rounded-full"></div> 已停用
          </span>
        )}
        <button className="text-[11px] font-medium text-blue-600 hover:text-blue-700 transition-colors">配置</button>
      </div>
    </div>
  );
};

import React, { useState, useEffect } from 'react';
import { ShieldCheck, Plus, Search, Check, MoreHorizontal, Users, Key } from 'lucide-react';
import { cn, ConsoleContractEmptyState } from '@sdkwork/im-pc-commons';
import { roleService, Role } from './services/RoleService';

export const ConsoleRoles = () => {
  const [roles, setRoles] = useState<Role[]>([]);
  const [activeRole, setActiveRole] = useState<Role | null>(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const fetchRoles = async () => {
      setLoading(true);
      try {
        const res = await roleService.getRoles();
        setRoles(res.data);
        if (res.data.length > 0) {
          setActiveRole(res.data[0]);
        }
      } finally {
        setLoading(false);
      }
    };
    fetchRoles();
  }, []);

  if (!activeRole && loading) {
    return <div className="p-8 text-center text-console-text-muted">加载角色中...</div>;
  }

  if (!activeRole) {
    return (
      <div className="bg-console-bg-panel border border-console-border rounded-2xl shadow-sm flex flex-1 min-h-0 h-full overflow-hidden flex-col">
        <div className="p-6 border-b border-console-border">
          <h2 className="text-lg font-bold text-console-text-main">角色与权限</h2>
          <p className="text-sm text-console-text-muted mt-1">管理企业角色模板与功能权限分配</p>
        </div>
        <ConsoleContractEmptyState title="暂无可用角色" description="当前租户尚未配置角色绑定，或角色数据暂不可用。" />
      </div>
    );
  }

  return (
    <div className="bg-console-bg-panel border border-console-border rounded-2xl shadow-sm flex flex-1 min-h-0 h-full overflow-hidden">
      {/* Sidebar list of roles */}
      <div className="w-64 border-r border-console-border flex flex-col bg-console-bg-root/30">
        <div className="p-4 border-b border-console-border flex items-center justify-between">
          <h3 className="font-semibold text-console-text-main">角色列表</h3>
          <button className="text-console-text-muted hover:text-blue-600 transition-colors">
            <Plus size={18} />
          </button>
        </div>
        <div className="p-3">
          <div className="relative">
            <Search size={14} className="absolute left-2.5 top-1/2 -translate-y-1/2 text-console-text-muted" />
            <input 
              type="text" 
              placeholder="搜索角色..." 
              className="w-full bg-console-input-bg border border-console-border rounded-md py-1.5 pl-8 pr-3 text-xs text-console-text-main focus:ring-1 focus:ring-blue-500 outline-none"
            />
          </div>
        </div>
        <div className="flex-1 overflow-y-auto">
          {loading ? (
            <div className="p-4 text-center text-console-text-muted text-sm">加载中...</div>
          ) : roles.map(role => (
            <button 
              key={role.id}
              onClick={() => setActiveRole(role)}
              className={cn(
                "w-full text-left px-4 py-3 border-b border-console-border transition-colors hover:bg-console-bg-hover relative",
                activeRole.id === role.id && "bg-console-active-bg border-l-2 border-l-blue-600"
              )}
            >
              <div className="flex justify-between items-center mb-1">
                <span className={cn("text-sm font-medium", activeRole.id === role.id ? "text-blue-700 dark:text-blue-500" : "text-console-text-main")}>{role.name}</span>
                {role.system && <span className="text-[10px] text-emerald-600 bg-emerald-50 dark:bg-emerald-500/10 dark:text-emerald-400 px-1.5 py-[1px] rounded border border-emerald-200 dark:border-emerald-500/20">系统内置</span>}
              </div>
              <div className="text-xs text-console-text-muted flex items-center gap-1.5 mt-1">
                <Users size={12} /> {role.count} 人
              </div>
            </button>
          ))}
        </div>
      </div>

      {/* Role details & permissions */}
      <div className="flex-1 flex flex-col">
        <div className="p-6 border-b border-console-border flex justify-between items-start">
          <div>
            <h2 className="text-lg font-bold text-console-text-main">{activeRole.name}</h2>
            <p className="text-sm text-console-text-muted mt-1">{activeRole.desc}</p>
          </div>
          <button className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors">
            编辑角色属性
          </button>
        </div>

        <div className="flex-1 overflow-y-auto p-6">
          <h4 className="text-sm font-semibold text-console-text-main mb-4 flex items-center gap-2">
            <Key size={16} className="text-blue-500" />
            功能权限配置
          </h4>

          <div className="space-y-4">
            <PermissionGroup.Root title="组织架构管理">
              <PermissionGroup.Item label="查看组织架构树" checked={true} />
              <PermissionGroup.Item label="添加/移除部门成员" checked={activeRole.name === '超级管理员' || activeRole.name === '部门管理员'} />
              <PermissionGroup.Item label="管理部门属性" checked={activeRole.name === '超级管理员'} />
            </PermissionGroup.Root>

            <PermissionGroup.Root title="通信与群组">
              <PermissionGroup.Item label="创建公开群组" checked={true} />
              <PermissionGroup.Item label="解散任意群组" checked={activeRole.name === '超级管理员' || activeRole.name === '安全合规管理员'} />
              <PermissionGroup.Item label="设置全局消息保留策略" checked={activeRole.name === '超级管理员'} />
            </PermissionGroup.Root>

            <PermissionGroup.Root title="应用与集成">
              <PermissionGroup.Item label="使用已授权的三方应用" checked={true} />
              <PermissionGroup.Item label="配置 Webhook 订阅" checked={activeRole.name === '超级管理员' || activeRole.name === '开发集成者'} />
              <PermissionGroup.Item label="上架内部自建应用" checked={activeRole.name === '超级管理员' || activeRole.name === '发集成者'} />
            </PermissionGroup.Root>

            <PermissionGroup.Root title="安全与合规">
              <PermissionGroup.Item label="查看操作审计日志" checked={activeRole.name === '超级管理员' || activeRole.name === '安全合规管理员'} />
              <PermissionGroup.Item label="配置数据防泄漏 (DLP) 策略" checked={activeRole.name === '超级管理员' || activeRole.name === '安全合规管理员'} />
              <PermissionGroup.Item label="导出敏感审计报告" checked={activeRole.name === '超级管理员' || activeRole.name === '安全合规管理员'} />
            </PermissionGroup.Root>
          </div>
        </div>
      </div>
    </div>
  );
};

const PermissionGroup = {
  Root: ({ title, children }: any) => (
    <div className="border border-console-border rounded-xl overflow-hidden">
      <div className="bg-console-bg-root/50 px-4 py-2 border-b border-console-border font-medium text-sm text-console-text-main">
        {title}
      </div>
      <div className="divide-y divide-console-border bg-console-bg-panel">
        {children}
      </div>
    </div>
  ),
  Item: ({ label, checked }: any) => (
    <div className="flex items-center justify-between px-4 py-3 hover:bg-console-bg-hover transition-colors">
      <span className="text-sm text-console-text-main">{label}</span>
      <label className="relative inline-flex items-center cursor-pointer">
        <input type="checkbox" className="sr-only peer" checked={checked} readOnly />
        <div className="w-11 h-6 bg-console-border peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600 border border-console-border-light"></div>
      </label>
    </div>
  )
};

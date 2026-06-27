import React, { useState, useEffect } from 'react';
import { Search, Plus, MoreHorizontal, Shield, Mail, Filter } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';
import { userService, User } from './services/UserService';

export const TenantUsers = () => {
  const [searchTerm, setSearchTerm] = useState('');
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState(false);
  const [total, setTotal] = useState(0);

  useEffect(() => {
    const fetchUsers = async () => {
      setLoading(true);
      try {
        const res = await userService.getUsers({ page: 1, pageSize: 10, search: searchTerm });
        setUsers(res.data);
        setTotal(res.total);
      } finally {
        setLoading(false);
      }
    };
    fetchUsers();
  }, [searchTerm]);

  return (
    <div className="bg-console-bg-panel border border-console-border rounded-2xl shadow-sm flex flex-col flex-1 min-h-0 h-full overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between p-6 border-b border-console-border">
        <div>
          <h2 className="text-lg font-bold text-console-text-main">组织架构与成员</h2>
          <p className="text-sm text-console-text-muted mt-1">管理企业组织架构、部门划分及员工账号权限</p>
        </div>
        <button className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors flex items-center gap-2 shadow-sm shadow-blue-600/20">
          <Plus size={16} />
          添加成员
        </button>
      </div>

      {/* Toolbar */}
      <div className="p-4 flex items-center justify-between bg-console-bg-root border-b border-console-border">
        <div className="flex items-center gap-3">
          <div className="relative">
            <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-console-text-muted" />
            <input 
              type="text" 
              placeholder="搜索姓名、邮箱、部门..." 
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="w-72 bg-console-input-bg border border-console-border rounded-lg py-1.5 pl-9 pr-4 text-sm text-console-text-main focus:ring-2 focus:ring-blue-500/50 focus:border-blue-500 outline-none transition-all shadow-sm"
            />
          </div>
          <button className="bg-console-bg-panel border border-console-border text-console-text-main px-3 py-1.5 rounded-lg text-sm flex items-center gap-2 hover:bg-console-bg-hover transition-colors shadow-sm">
            <Filter size={14} />
            筛选
          </button>
        </div>
        
        <div className="flex gap-2">
          <select className="bg-console-bg-panel border border-console-border text-sm text-console-text-main rounded-lg px-3 py-1.5 outline-none shadow-sm cursor-pointer hover:bg-console-bg-hover transition-colors">
            <option>批量操作</option>
            <option>导出数据</option>
            <option>禁用账号</option>
          </select>
        </div>
      </div>

      {/* Table */}
      <div className="flex-1 overflow-auto">
        <table className="w-full text-left border-collapse">
          <thead>
            <tr className="bg-console-bg-root text-console-text-muted text-xs uppercase tracking-wider border-b border-console-border">
              <th className="px-6 py-4 font-semibold w-12 text-center">
                <input type="checkbox" className="rounded border-console-border text-blue-600 focus:ring-blue-500" />
              </th>
              <th className="px-6 py-4 font-semibold">成员信息</th>
              <th className="px-6 py-4 font-semibold">部门/职位</th>
              <th className="px-6 py-4 font-semibold">角色权限</th>
              <th className="px-6 py-4 font-semibold">状态</th>
              <th className="px-6 py-4 font-semibold">最近活跃</th>
              <th className="px-6 py-4 font-semibold text-right">操作</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-console-border text-sm">
            {loading ? (
              <tr><td colSpan={7} className="px-6 py-8 text-center text-console-text-muted">加载中...</td></tr>
            ) : users.length === 0 ? (
              <tr><td colSpan={7} className="px-6 py-8 text-center text-console-text-muted">暂无数据</td></tr>
            ) : users.map((user) => (
              <tr key={user.id} className="hover:bg-console-bg-hover transition-colors group">
                <td className="px-6 py-4 text-center">
                  <input type="checkbox" className="rounded border-console-border text-blue-600 focus:ring-blue-500" />
                </td>
                <td className="px-6 py-4">
                  <div className="flex items-center gap-3">
                    <div className="w-10 h-10 rounded-full bg-blue-100/50 text-blue-700 flex items-center justify-center font-bold">
                      {user.name.charAt(0)}
                    </div>
                    <div>
                      <div className="font-semibold text-console-text-main group-hover:text-blue-600 transition-colors cursor-pointer">{user.name}</div>
                      <div className="text-xs text-console-text-muted flex items-center gap-1 mt-0.5">
                        <Mail size={10} /> {user.email}
                      </div>
                    </div>
                  </div>
                </td>
                <td className="px-6 py-4">
                  <div className="text-console-text-main font-medium">{user.department}</div>
                  <div className="text-xs text-console-text-muted mt-0.5">常规成员</div>
                </td>
                <td className="px-6 py-4">
                  {user.role === 'admin' ? (
                    <span className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-md text-xs font-medium bg-purple-500/10 text-purple-600 border border-purple-500/20">
                      <Shield size={12} /> 管理员
                    </span>
                  ) : (
                    <span className="inline-flex items-center px-2.5 py-1 rounded-md text-xs font-medium bg-console-bg-hover text-console-text-muted border border-console-border">
                      普通成员
                    </span>
                  )}
                </td>
                <td className="px-6 py-4">
                  {user.status === 'active' && <span className="flex items-center gap-1.5 text-emerald-600 text-xs font-medium"><div className="w-1.5 h-1.5 rounded-full bg-emerald-500"></div> 在线</span>}
                  {user.status === 'offline' && <span className="flex items-center gap-1.5 text-console-text-muted text-xs font-medium"><div className="w-1.5 h-1.5 rounded-full bg-gray-400"></div> 离线</span>}
                  {user.status === 'disabled' && <span className="flex items-center gap-1.5 text-rose-600 text-xs font-medium"><div className="w-1.5 h-1.5 rounded-full bg-rose-500"></div> 禁用</span>}
                </td>
                <td className="px-6 py-4 text-console-text-muted text-xs">{user.lastLogin}</td>
                <td className="px-6 py-4 text-right">
                  <button className="p-1.5 text-console-text-muted hover:text-blue-600 hover:bg-console-bg-root rounded-md transition-colors">
                    <MoreHorizontal size={18} />
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {/* Pagination */}
      <div className="p-4 border-t border-console-border flex items-center justify-between text-sm text-console-text-muted bg-console-bg-root/50">
        <div>显示 1 到 5 条，共 1,240 条记录</div>
        <div className="flex gap-1">
          <button className="px-3 py-1 border border-console-border rounded text-console-text-muted cursor-not-allowed bg-console-bg-root">上一页</button>
          <button className="px-3 py-1 border border-blue-600 rounded bg-blue-600 text-white font-medium">1</button>
          <button className="px-3 py-1 border border-console-border rounded text-console-text-main hover:bg-console-bg-hover transition-colors">2</button>
          <button className="px-3 py-1 border border-console-border rounded text-console-text-main hover:bg-console-bg-hover transition-colors">3</button>
          <span className="px-2 py-1">...</span>
          <button className="px-3 py-1 border border-console-border rounded text-console-text-main hover:bg-console-bg-hover transition-colors">下一页</button>
        </div>
      </div>
    </div>
  );
};

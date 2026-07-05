import React, { useCallback, useEffect, useState } from 'react';
import { Search, Plus, MoreHorizontal, Shield, Users, MessageCircle, Settings, Filter, Lock, Globe } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';
import { groupService, Group } from './services/GroupService';

export const ConsoleGroups = () => {
  const [searchTerm, setSearchTerm] = useState('');
  const [groups, setGroups] = useState<Group[]>([]);
  const [loading, setLoading] = useState(false);
  const [nextCursor, setNextCursor] = useState<string | undefined>();
  const [hasMore, setHasMore] = useState(false);

  const loadGroups = useCallback(async (cursor?: string, append = false) => {
    setLoading(true);
    try {
      const page = await groupService.listGroupsPage({
        pageSize: 10,
        cursor,
        search: searchTerm,
      });
      setGroups((current) => (append ? [...current, ...page.data] : page.data));
      setNextCursor(page.nextCursor);
      setHasMore(page.hasMore);
    } finally {
      setLoading(false);
    }
  }, [searchTerm]);

  useEffect(() => {
    void loadGroups();
  }, [loadGroups]);

  return (
    <div className="bg-console-bg-panel border border-console-border rounded-2xl shadow-sm flex flex-col flex-1 min-h-0 overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between p-6 border-b border-console-border">
        <div>
          <h2 className="text-lg font-bold text-console-text-main">群组与通信管理</h2>
          <p className="text-sm text-console-text-muted mt-1">管理企业内的所有聊天群组及全局通信策略</p>
        </div>
        <div className="flex gap-3">
          <button className="bg-console-bg-hover hover:bg-console-border text-console-text-main px-4 py-2 rounded-lg text-sm font-medium transition-colors flex items-center gap-2 border border-console-border">
            <Settings size={16} />
            全局策略设置
          </button>
          <button className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors flex items-center gap-2 shadow-sm">
            <Plus size={16} />
            新建群组
          </button>
        </div>
      </div>

      {/* Metrics Row */}
      <div className="grid grid-cols-4 divide-x divide-console-border border-b border-console-border bg-console-bg-root/50">
        <div className="p-4 flex flex-col">
          <span className="text-xs text-console-text-muted mb-1">已加载群组</span>
          <span className="text-xl font-bold text-console-text-main">{groups.length}</span>
        </div>
        <div className="p-4 flex flex-col">
          <span className="text-xs text-console-text-muted mb-1">分页状态</span>
          <span className="text-xl font-bold text-emerald-500">{hasMore ? '更多可用' : '已到底'}</span>
        </div>
        <div className="p-4 flex flex-col">
          <span className="text-xs text-console-text-muted mb-1">数据来源</span>
          <span className="text-xl font-bold text-console-text-main">Inbox SDK</span>
        </div>
        <div className="p-4 flex flex-col">
          <span className="text-xs text-console-text-muted mb-1">分页模式</span>
          <span className="text-xl font-bold text-console-text-main">Cursor</span>
        </div>
      </div>

      {/* Toolbar */}
      <div className="p-4 flex items-center justify-between bg-console-bg-root border-b border-console-border">
        <div className="flex items-center gap-3">
          <div className="relative">
            <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-console-text-muted" />
            <input 
              type="text" 
              placeholder="搜索群ID、群名称、群主..." 
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="w-72 bg-console-input-bg border border-console-border rounded-lg py-1.5 pl-9 pr-4 text-sm text-console-text-main focus:ring-2 focus:ring-blue-500/50 focus:border-blue-500 outline-none transition-all"
            />
          </div>
          <button className="bg-console-bg-panel border border-console-border text-console-text-main px-3 py-1.5 rounded-lg text-sm flex items-center gap-2 hover:bg-console-bg-hover transition-colors">
            <Filter size={14} />
            筛选
          </button>
        </div>
        
        <div className="flex gap-2">
          <select className="bg-console-bg-panel border border-console-border text-sm text-console-text-main rounded-lg px-3 py-1.5 outline-none cursor-pointer hover:bg-console-bg-hover transition-colors">
            <option>批量操作</option>
            <option>解散群组</option>
            <option>转移群主</option>
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
              <th className="px-6 py-4 font-semibold">群组信息</th>
              <th className="px-6 py-4 font-semibold">类型</th>
              <th className="px-6 py-4 font-semibold">群主</th>
              <th className="px-6 py-4 font-semibold">成员数</th>
              <th className="px-6 py-4 font-semibold">今日消息数</th>
              <th className="px-6 py-4 font-semibold">状态</th>
              <th className="px-6 py-4 font-semibold text-right">操作</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-console-border text-sm">
            {loading && groups.length === 0 ? (
              <tr><td colSpan={8} className="px-6 py-8 text-center text-console-text-muted">加载中...</td></tr>
            ) : groups.length === 0 ? (
              <tr><td colSpan={8} className="px-6 py-8 text-center text-console-text-muted">暂无数据</td></tr>
            ) : groups.map((group) => (
              <tr key={group.id} className="hover:bg-console-bg-hover transition-colors group">
                <td className="px-6 py-4 text-center">
                  <input type="checkbox" className="rounded border-console-border text-blue-600 focus:ring-blue-500" />
                </td>
                <td className="px-6 py-4">
                  <div className="flex items-center gap-3">
                    <div className="w-10 h-10 rounded-xl bg-blue-100/50 text-blue-600 flex items-center justify-center">
                      <Users size={18} />
                    </div>
                    <div>
                      <div className="font-semibold text-console-text-main group-hover:text-blue-600 transition-colors cursor-pointer">{group.name}</div>
                      <div className="text-xs text-console-text-muted mt-0.5 font-mono">{group.id}</div>
                    </div>
                  </div>
                </td>
                <td className="px-6 py-4">
                  {group.type === 'public' ? (
                    <span className="inline-flex items-center gap-1 text-xs text-emerald-600 font-medium">
                      <Globe size={12} /> 公开群
                    </span>
                  ) : (
                    <span className="inline-flex items-center gap-1 text-xs text-amber-600 font-medium">
                      <Lock size={12} /> 私密群
                    </span>
                  )}
                </td>
                <td className="px-6 py-4 text-console-text-main">{group.owner}</td>
                <td className="px-6 py-4 text-console-text-main font-medium">{group.members}</td>
                <td className="px-6 py-4">
                  <div className="flex items-center gap-1.5 text-console-text-muted">
                    <MessageCircle size={14} />
                    <span>{group.messagesToDay}</span>
                  </div>
                </td>
                <td className="px-6 py-4">
                  {group.status === 'active' ? (
                    <span className="px-2.5 py-1 rounded-md text-[11px] font-medium bg-emerald-500/10 text-emerald-600 border border-emerald-500/20">正常</span>
                  ) : (
                    <span className="px-2.5 py-1 rounded-md text-[11px] font-medium bg-console-bg-hover text-console-text-muted border border-console-border">已归档</span>
                  )}
                </td>
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
      <div className="p-4 border-t border-console-border flex items-center justify-between text-xs text-console-text-muted bg-console-bg-root/50">
        <div>已加载 {groups.length} 条群组记录</div>
        <div className="flex gap-1">
          <button
            type="button"
            disabled={loading || !hasMore || !nextCursor}
            onClick={() => void loadGroups(nextCursor, true)}
            className={cn(
              'px-3 py-1.5 border border-console-border rounded text-console-text-main transition-colors',
              loading || !hasMore || !nextCursor
                ? 'opacity-50 cursor-not-allowed'
                : 'hover:bg-console-bg-hover',
            )}
          >
            {loading ? '加载中...' : '加载更多'}
          </button>
        </div>
      </div>
    </div>
  );
};

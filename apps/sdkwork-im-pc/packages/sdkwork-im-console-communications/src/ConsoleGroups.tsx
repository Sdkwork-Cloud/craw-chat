import React, { useCallback, useEffect, useRef, useState } from 'react';
import { Search, Users, Clock, RefreshCw } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';
import { groupService, Group } from './services/GroupService';

export const ConsoleGroups = () => {
  const [searchTerm, setSearchTerm] = useState('');
  const [groups, setGroups] = useState<Group[]>([]);
  const [loading, setLoading] = useState(false);
  const [loadingMore, setLoadingMore] = useState(false);
  const [loadError, setLoadError] = useState(false);
  const [nextCursor, setNextCursor] = useState<string | undefined>();
  const [hasMore, setHasMore] = useState(false);
  const requestSequenceRef = useRef(0);

  const loadGroups = useCallback(async ({
    append = false,
    cursor,
    requestId,
  }: {
    append?: boolean;
    cursor?: string;
    requestId: number;
  }) => {
    if (append) {
      setLoadingMore(true);
    } else {
      setLoading(true);
    }
    setLoadError(false);
    try {
      const page = await groupService.listGroupsPage({
        pageSize: 20,
        cursor,
        q: searchTerm.trim() || undefined,
      });
      if (requestSequenceRef.current !== requestId) {
        return;
      }
      setGroups((current) => {
        if (!append) {
          return page.data;
        }
        const byId = new Map(current.map((group) => [group.id, group]));
        for (const group of page.data) {
          byId.set(group.id, group);
        }
        return [...byId.values()];
      });
      setNextCursor(page.nextCursor);
      setHasMore(page.hasMore);
    } catch {
      if (requestSequenceRef.current !== requestId) {
        return;
      }
      if (!append) {
        setGroups([]);
        setNextCursor(undefined);
        setHasMore(false);
      }
      setLoadError(true);
    } finally {
      if (requestSequenceRef.current === requestId) {
        if (append) {
          setLoadingMore(false);
        } else {
          setLoading(false);
        }
      }
    }
  }, [searchTerm]);

  useEffect(() => {
    const requestId = ++requestSequenceRef.current;
    setGroups([]);
    setLoading(true);
    setLoadingMore(false);
    setLoadError(false);
    setNextCursor(undefined);
    setHasMore(false);
    const timer = window.setTimeout(() => {
      void loadGroups({ requestId });
    }, 250);
    return () => window.clearTimeout(timer);
  }, [loadGroups]);

  const retryLoad = useCallback(() => {
    const requestId = ++requestSequenceRef.current;
    void loadGroups({ requestId });
  }, [loadGroups]);

  const loadMoreGroups = useCallback(() => {
    if (!hasMore || !nextCursor || loadingMore) {
      return;
    }
    const requestId = ++requestSequenceRef.current;
    void loadGroups({ append: true, cursor: nextCursor, requestId });
  }, [hasMore, loadGroups, loadingMore, nextCursor]);

  return (
    <div className="bg-console-bg-panel border border-console-border rounded-2xl shadow-sm flex flex-col flex-1 min-h-0 overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between p-6 border-b border-console-border">
        <div>
          <h2 className="text-lg font-bold text-console-text-main">群组与通信管理</h2>
          <p className="text-sm text-console-text-muted mt-1">查看当前账号可见的群聊收件箱与最近活动</p>
        </div>
        <div className="text-xs text-console-text-muted">数据来自当前用户会话的群聊收件箱</div>
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
              placeholder="搜索群 ID 或群名称..."
              aria-label="搜索群组"
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="w-72 bg-console-input-bg border border-console-border rounded-lg py-1.5 pl-9 pr-4 text-sm text-console-text-main focus:ring-2 focus:ring-blue-500/50 focus:border-blue-500 outline-none transition-all"
            />
          </div>
          <span className="text-xs text-console-text-muted">搜索覆盖当前账号可见的全部群组</span>
        </div>
        
        <div />
      </div>

      {/* Table */}
      <div className="flex-1 overflow-auto">
        <table className="w-full text-left border-collapse">
          <thead>
            <tr className="bg-console-bg-root text-console-text-muted text-xs uppercase tracking-wider border-b border-console-border">
              <th className="px-6 py-4 font-semibold">群组信息</th>
              <th className="px-6 py-4 font-semibold">最近活动</th>
              <th className="px-6 py-4 font-semibold">当前用户未读</th>
              <th className="px-6 py-4 font-semibold">数据范围</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-console-border text-sm">
            {loading ? (
              <tr><td colSpan={4} className="px-6 py-8 text-center text-console-text-muted">加载中...</td></tr>
            ) : loadError && groups.length === 0 ? (
              <tr>
                <td colSpan={4} className="px-6 py-8 text-center text-console-text-muted">
                  <button
                    type="button"
                    onClick={retryLoad}
                    className="inline-flex items-center gap-2 rounded-lg border border-console-border px-3 py-2 text-console-text-main hover:bg-console-bg-hover"
                  >
                    <RefreshCw size={14} /> 加载失败，重试
                  </button>
                </td>
              </tr>
            ) : groups.length === 0 ? (
              <tr><td colSpan={4} className="px-6 py-8 text-center text-console-text-muted">暂无数据</td></tr>
            ) : groups.map((group) => (
              <tr key={group.id} className="hover:bg-console-bg-hover transition-colors group">
                <td className="px-6 py-4">
                  <div className="flex items-center gap-3">
                    <div className="w-10 h-10 rounded-xl bg-blue-100/50 text-blue-600 flex items-center justify-center">
                      <Users size={18} />
                    </div>
                    <div>
                      <div className="font-semibold text-console-text-main group-hover:text-blue-600 transition-colors">{group.name || '未命名群组'}</div>
                      <div className="text-xs text-console-text-muted mt-0.5 font-mono">{group.id}</div>
                    </div>
                  </div>
                </td>
                <td className="px-6 py-4">
                  <div className="flex items-center gap-1.5 text-console-text-muted">
                    <Clock size={14} />
                    <span>{group.lastActivityAt ? new Date(group.lastActivityAt).toLocaleString() : '暂无数据'}</span>
                  </div>
                </td>
                <td className="px-6 py-4 text-console-text-main font-medium">{group.unreadCount}</td>
                <td className="px-6 py-4 text-xs text-console-text-muted">当前会话收件箱</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      
      {/* Pagination */}
      <div className="p-4 border-t border-console-border flex items-center justify-between text-xs text-console-text-muted bg-console-bg-root/50">
        <div>{loadError && groups.length > 0 ? '加载更多失败，可重试' : `已加载 ${groups.length} 条群组记录`}</div>
        <div className="flex gap-1">
          <button
            type="button"
            disabled={loading || loadingMore || (!loadError && (!hasMore || !nextCursor))}
            onClick={loadError ? retryLoad : loadMoreGroups}
            className={cn(
              'px-3 py-1.5 border border-console-border rounded text-console-text-main transition-colors',
              loading || loadingMore || (!loadError && (!hasMore || !nextCursor))
                ? 'opacity-50 cursor-not-allowed'
                : 'hover:bg-console-bg-hover',
            )}
          >
            {loadError ? '重试' : loadingMore ? '加载中...' : '加载更多'}
          </button>
        </div>
      </div>
    </div>
  );
};

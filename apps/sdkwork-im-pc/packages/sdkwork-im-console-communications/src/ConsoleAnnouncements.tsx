import React, { useState, useEffect } from 'react';
import { Bell, Plus, Calendar, Eye, MoreHorizontal, Send, Users } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';
import { announcementService, Announcement } from './services/AnnouncementService';

export const ConsoleAnnouncements = () => {
  const [announcements, setAnnouncements] = useState<Announcement[]>([]);
  const [stats, setStats] = useState({ publishedCount: 0, viewsCount: 0, draftCount: 0 });
  const [loading, setLoading] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  useEffect(() => {
    const fetchAnnouncements = async () => {
      setLoading(true);
      setErrorMessage(null);
      try {
        const res = await announcementService.getAnnouncements();
        setAnnouncements(res.data);
        setStats({
          publishedCount: res.publishedCount,
          viewsCount: res.viewsCount,
          draftCount: res.draftCount
        });
      } catch (error) {
        setAnnouncements([]);
        setStats({ publishedCount: 0, viewsCount: 0, draftCount: 0 });
        setErrorMessage(error instanceof Error ? error.message : String(error));
      } finally {
        setLoading(false);
      }
    };
    fetchAnnouncements();
  }, []);
  return (
    <div className="bg-console-bg-panel border border-console-border rounded-2xl shadow-sm flex flex-col min-h-[600px] overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between p-6 border-b border-console-border bg-gradient-to-r from-console-bg-panel to-console-bg-root">
        <div>
          <h2 className="text-lg font-bold text-console-text-main">租户通知与公告</h2>
          <p className="text-sm text-console-text-muted mt-1">全局发送系统广播、企业公告，支持富文本与定时推送</p>
        </div>
        <button className="bg-blue-600 hover:bg-blue-700 text-white px-5 py-2.5 rounded-lg text-sm font-medium transition-all shadow-sm flex items-center gap-2 hover:shadow-blue-500/20 border border-transparent">
          <Send size={16} />
          发布新公告
        </button>
      </div>

      {/* Metrics */}
      <div className="grid grid-cols-3 divide-x divide-console-border border-b border-console-border bg-console-bg-root/50">
        <div className="p-5">
          <div className="text-xs text-console-text-muted mb-1 font-medium">累计发布公告</div>
          <div className="text-2xl font-bold text-console-text-main font-mono">{stats.publishedCount}</div>
        </div>
        <div className="p-5">
          <div className="text-xs text-console-text-muted mb-1 font-medium">近 30 天触达人次</div>
          <div className="text-2xl font-bold text-blue-600 font-mono">{stats.viewsCount.toLocaleString()}</div>
        </div>
        <div className="p-5">
          <div className="text-xs text-console-text-muted mb-1 font-medium">草稿箱</div>
          <div className="text-2xl font-bold text-amber-500 font-mono">{stats.draftCount}</div>
        </div>
      </div>

      {/* List */}
      <div className="flex-1 overflow-auto p-6 bg-console-bg-root/30">
        <div className="flex items-center justify-between mb-4">
          <h3 className="font-semibold text-console-text-main">历史与草稿列表</h3>
          <select className="bg-console-input-bg border border-console-border rounded-md px-3 py-1.5 text-sm text-console-text-main outline-none">
            <option>全部状态</option>
            <option>已发布</option>
            <option>草稿</option>
          </select>
        </div>
        
        <div className="space-y-3">
          {loading ? (
            <div className="p-4 text-center text-console-text-muted text-sm">加载中...</div>
          ) : errorMessage ? (
            <div className="p-4 text-center text-console-text-muted text-sm">{errorMessage}</div>
          ) : announcements.length === 0 ? (
            <div className="p-4 text-center text-console-text-muted text-sm">暂无公告</div>
          ) : announcements.map((item) => (
            <div key={item.id} className="bg-console-bg-panel border border-console-border rounded-xl p-4 hover:border-blue-500/30 transition-all hover:shadow-sm group">
              <div className="flex justify-between items-start">
                <div className="flex-1">
                  <div className="flex items-center gap-2 mb-2">
                    {item.status === 'published' ? (
                      <span className="px-2 py-0.5 rounded text-[10px] font-medium bg-emerald-50 text-emerald-600 dark:bg-emerald-500/10 dark:text-emerald-400 border border-emerald-200 dark:border-emerald-500/20">已发布</span>
                    ) : (
                      <span className="px-2 py-0.5 rounded text-[10px] font-medium bg-amber-50 text-amber-600 dark:bg-amber-500/10 dark:text-amber-400 border border-amber-200 dark:border-amber-500/20">草稿</span>
                    )}
                    <h4 className="text-base font-semibold text-console-text-main group-hover:text-blue-600 transition-colors cursor-pointer">{item.title}</h4>
                  </div>
                  <div className="flex flex-wrap items-center gap-4 text-xs text-console-text-muted">
                    <span className="flex items-center gap-1.5"><Calendar size={14} /> {item.date}</span>
                    <span className="flex items-center gap-1.5"><Users size={14} /> 发件人: {item.sender}</span>
                    {item.status === 'published' && (
                      <span className="flex items-center gap-1.5"><Eye size={14} /> {item.views} 次阅读</span>
                    )}
                  </div>
                </div>
                <button className="p-1.5 text-console-text-muted hover:text-blue-600 hover:bg-console-bg-root rounded-md transition-colors">
                  <MoreHorizontal size={18} />
                </button>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

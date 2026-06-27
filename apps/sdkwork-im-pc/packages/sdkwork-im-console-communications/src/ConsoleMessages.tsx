import React, { useState, useEffect } from 'react';
import { MessageSquare, Search, Trash2, Download, AlertTriangle, Settings, FileText, Database, ShieldCheck } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';
import { messageAuditService, AuditMessage } from './services/MessageAuditService';

export const ConsoleMessages = () => {
  const [searchTerm, setSearchTerm] = useState('');
  const [messages, setMessages] = useState<AuditMessage[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const fetchMessages = async () => {
      setLoading(true);
      try {
        const res = await messageAuditService.getMessages({ page: 1, pageSize: 10, search: searchTerm });
        setMessages(res.data);
      } finally {
        setLoading(false);
      }
    };
    fetchMessages();
  }, [searchTerm]);
  return (
    <div className="h-full flex flex-col gap-6">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <h2 className="text-lg font-bold text-console-text-main">消息与内容管理</h2>
          <p className="text-sm text-console-text-muted mt-1">全局消息保留策略、DLP 防泄漏及审计历史查询</p>
        </div>
      </div>

      {/* Storage & Policies */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="bg-console-bg-panel border border-console-border rounded-2xl p-6 shadow-sm flex flex-col relative overflow-hidden">
          <Database className="absolute -right-4 -bottom-4 w-32 h-32 text-console-text-muted opacity-5" />
          <h3 className="font-semibold text-console-text-main mb-4 flex items-center gap-2">
            <Settings size={18} className="text-blue-500" />
            消息生命周期 (云端保留策略)
          </h3>
          <div className="space-y-4 relative z-10 font-medium">
            <div className="flex justify-between items-center p-3 rounded-xl border border-console-border min-h-[52px]">
              <span className="text-sm text-console-text-main">单聊消息保留时长</span>
              <select className="bg-console-bg-root border border-console-border text-sm text-console-text-main rounded-md px-2 py-1 outline-none">
                <option>永久保留</option>
                <option>7 天</option>
                <option>30 天</option>
                <option>180 天</option>
                <option>365 天</option>
              </select>
            </div>
            <div className="flex justify-between items-center p-3 rounded-xl border border-console-border min-h-[52px]">
              <span className="text-sm text-console-text-main">群组消息保留时长</span>
              <select className="bg-console-bg-root border border-console-border text-sm text-console-text-main rounded-md px-2 py-1 outline-none">
                <option>永久保留</option>
                <option>7 天</option>
                <option>30 天</option>
                <option>180 天</option>
                <option>365 天</option>
              </select>
            </div>
            <div className="flex justify-between items-center p-3 rounded-xl border border-console-border min-h-[52px]">
              <span className="text-sm text-console-text-main">文件附件保留时长</span>
              <select className="bg-console-bg-root border border-console-border text-sm text-console-text-main rounded-md px-2 py-1 outline-none">
                <option>180 天</option>
                <option>365 天</option>
                <option>永久保留</option>
              </select>
            </div>
            <button className="w-full py-2 bg-blue-50 dark:bg-blue-500/10 text-blue-600 rounded-lg text-sm transition-colors border border-blue-200 dark:border-blue-500/20 hover:bg-blue-100 dark:hover:bg-blue-500/20">
              更新保留策略
            </button>
          </div>
        </div>

        <div className="bg-console-bg-panel border border-console-border rounded-2xl p-6 shadow-sm flex flex-col relative overflow-hidden">
          <AlertTriangle className="absolute -right-4 -bottom-4 w-32 h-32 text-rose-500 opacity-5" />
          <h3 className="font-semibold text-console-text-main mb-4 flex items-center gap-2">
            <AlertTriangle size={18} className="text-rose-500" />
            数据防泄漏 (DLP)
          </h3>
          <p className="text-xs text-console-text-muted mb-4 opacity-80">开启 DLP 将实时拦截可能泄露企业核心资产的消息或文件传输。</p>
          
          <div className="space-y-3 relative z-10">
            <div className="flex items-center justify-between p-3 rounded-xl border border-console-border bg-console-bg-root/50 hover:bg-console-bg-hover transition-colors">
              <div>
                <div className="text-sm font-medium text-console-text-main">检测社会工程学凭证</div>
                <div className="text-[11px] text-console-text-muted">如身份证、信用卡号、各类证件号</div>
              </div>
              <input type="checkbox" className="toggle-checkbox" defaultChecked />
            </div>
            <div className="flex items-center justify-between p-3 rounded-xl border border-console-border bg-console-bg-root/50 hover:bg-console-bg-hover transition-colors">
              <div>
                <div className="text-sm font-medium text-console-text-main">检测代码片段与凭据</div>
                <div className="text-[11px] text-console-text-muted">如代码块、API Keys、Secret Token</div>
              </div>
              <input type="checkbox" className="toggle-checkbox" defaultChecked />
            </div>
            <div className="flex items-center justify-between p-3 rounded-xl border border-console-border bg-console-bg-root/50 hover:bg-console-bg-hover transition-colors">
              <div>
                <div className="text-sm font-medium text-console-text-main">自定义敏感词库检测</div>
                <div className="text-[11px] text-console-text-muted">共 243 个激活的敏感词规则</div>
              </div>
              <button className="text-xs text-blue-600 font-medium hover:text-blue-700">管理词库</button>
            </div>
          </div>
        </div>
      </div>

      {/* Message Content Audit Search */}
      <div className="bg-console-bg-panel border border-console-border rounded-2xl shadow-sm flex flex-col flex-1 min-h-0 overflow-hidden">
        <div className="p-5 border-b border-console-border flex items-center justify-between bg-console-bg-root/30">
          <h3 className="font-semibold text-console-text-main flex items-center gap-2">
            <Search size={18} className="text-console-text-muted" />
            全局消息历史审计
          </h3>
          <button className="text-xs bg-console-bg-panel border border-console-border text-console-text-main px-3 py-1.5 rounded-md hover:bg-console-bg-hover transition-colors">
            导出审计文档
          </button>
        </div>
        
        <div className="p-4 border-b border-console-border flex gap-3 flex-wrap">
          <input 
            type="text" 
            placeholder="搜索关键词、发件人..." 
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="w-64 bg-console-input-bg border border-console-border rounded-lg py-1.5 px-3 text-sm text-console-text-main focus:ring-1 focus:ring-blue-500 outline-none"
          />
          <input type="date" className="bg-console-input-bg border border-console-border rounded-lg py-1.5 px-3 text-sm text-console-text-main outline-none" />
          <select className="bg-console-input-bg border border-console-border rounded-lg py-1.5 px-3 text-sm text-console-text-main outline-none">
            <option>所有类型</option>
            <option>仅文件</option>
            <option>含有敏感词</option>
          </select>
          <button className="bg-blue-600 text-white px-4 py-1.5 rounded-lg text-sm font-medium">查询记录</button>
        </div>

        <div className="flex-1 overflow-auto custom-scrollbar">
          <table className="w-full text-left border-collapse">
            <thead>
              <tr className="bg-console-bg-root text-console-text-muted text-xs uppercase tracking-wider border-b border-console-border">
                <th className="px-6 py-3 font-semibold">发送时间</th>
                <th className="px-6 py-3 font-semibold">发件人</th>
                <th className="px-6 py-3 font-semibold">接收方 (群组/人)</th>
                <th className="px-6 py-3 font-semibold w-1/3">消息片段预览</th>
                <th className="px-6 py-3 font-semibold text-right">操作</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-console-border text-sm">
              {loading ? (
                <tr><td colSpan={5} className="px-6 py-8 text-center text-console-text-muted">查询中...</td></tr>
              ) : messages.length === 0 ? (
                <tr><td colSpan={5} className="px-6 py-8 text-center text-console-text-muted">暂无符合条件的记录</td></tr>
              ) : messages.map(msg => (
                <AuditMessageRow key={msg.id} time={msg.time} sender={msg.sender} receiver={msg.receiver} snippet={msg.snippet} alert={msg.alert} />
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
};

const AuditMessageRow = ({ time, sender, receiver, snippet, alert }: any) => (
  <tr className="hover:bg-console-bg-hover transition-colors group">
    <td className="px-6 py-3 text-xs text-console-text-muted font-mono">{time}</td>
    <td className="px-6 py-3 text-console-text-main font-medium">{sender}</td>
    <td className="px-6 py-3 text-console-text-muted text-xs">{receiver}</td>
    <td className="px-6 py-3">
      <div className="flex items-center gap-2">
        {alert && <ShieldCheck size={14} className="text-rose-500 shrink-0" />}
        <span className={cn("text-xs truncate max-w-xs", alert ? "text-rose-600 dark:text-rose-400 font-medium" : "text-console-text-main")}>{snippet}</span>
      </div>
    </td>
    <td className="px-6 py-3 text-right">
      <button className="text-xs text-blue-600 hover:text-blue-700 font-medium">查看上下文</button>
    </td>
  </tr>
);

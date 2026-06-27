import React, { useState, useEffect } from 'react';
import { 
  FileBox, Edit3, Send, Archive, Search, Filter, Calendar, BarChart3, Presentation, Plus, AlertCircle 
} from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';
import { toast } from '@sdkwork/im-pc-chat';
import { reportService, ReportItem } from './services/ReportService';

export const ReportsView: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'received' | 'sent'>('received');
  const [reports, setReports] = useState<ReportItem[]>([]);
  const [showEditor, setShowEditor] = useState(false);

  const [newReportType, setNewReportType] = useState<'daily'|'weekly'|'monthly'>('daily');
  const [newContent, setNewContent] = useState('');
  const [newPlan, setNewPlan] = useState('');

  useEffect(() => {
    const fetchReports = async () => {
      const data = await reportService.getReports();
      setReports(data);
    };
    fetchReports();
  }, []);

  const displayList = reports.filter(r => activeTab === 'received' ? r.author !== '我' : r.author === '我');

  const handleSubmit = async () => {
    if (!newContent.trim()) {
      toast('请填写汇报内容', 'error');
      return;
    }
    const report = await reportService.submitReport(newReportType, newContent, newPlan);
    setReports([report, ...reports]);
    setShowEditor(false);
    setNewContent('');
    setNewPlan('');
    toast('汇报发送成功', 'success');
  };

  const getTypeStr = (t: string) => t === 'daily' ? '日报' : t === 'weekly' ? '周报' : '月报';
  const getTypeColor = (t: string) => {
    if (t === 'daily') return 'bg-cyan-500/10 text-cyan-400 border-cyan-500/20';
    if (t === 'weekly') return 'bg-purple-500/10 text-purple-400 border-purple-500/20';
    return 'bg-pink-500/10 text-pink-400 border-pink-500/20';
  };
  const getTypeIcon = (t: string) => {
    if (t === 'daily') return <FileBox size={16} className="text-cyan-400" />;
    if (t === 'weekly') return <BarChart3 size={16} className="text-purple-400" />;
    return <Presentation size={16} className="text-pink-400" />;
  };

  return (
    <div className="flex-1 flex flex-col bg-[#1e1e1e] min-w-0 animate-in fade-in">
      {/* Header */}
      <div className="h-16 border-b border-white/5 flex items-center justify-between px-6 shrink-0 bg-[#1e1e1e]/80 backdrop-blur-md">
        <div className="flex items-center gap-6">
          <h2 className="text-lg font-medium text-gray-200">工作汇报</h2>
          <div className="flex items-center gap-1 bg-black/20 p-1 rounded-lg">
            <button 
              onClick={() => { setActiveTab('received'); setShowEditor(false); }}
              className={cn("px-4 py-1.5 rounded-md text-sm font-medium transition-colors", activeTab === 'received' && !showEditor ? "bg-[#3a3a3a] text-white shadow-sm" : "text-gray-400 hover:text-gray-200")}
            > 我收到的 </button>
            <button 
              onClick={() => { setActiveTab('sent'); setShowEditor(false); }}
              className={cn("px-4 py-1.5 rounded-md text-sm font-medium transition-colors", activeTab === 'sent' && !showEditor ? "bg-[#3a3a3a] text-white shadow-sm" : "text-gray-400 hover:text-gray-200")}
            > 我发出的 </button>
          </div>
        </div>
        <div className="flex items-center gap-3">
           <button onClick={() => setShowEditor(true)} className="flex items-center gap-2 bg-indigo-600 hover:bg-indigo-500 text-white px-4 py-2 rounded-lg text-sm font-medium transition-all shadow-lg shadow-indigo-500/20">
              <Edit3 size={16} /> 写汇报
           </button>
        </div>
      </div>

      {/* Main Content */}
      <div className="flex-1 overflow-y-auto custom-scrollbar p-6 bg-[#181818]">
        <div className="max-w-4xl mx-auto w-full">
           {showEditor ? (
             <div className="bg-[#1e1e1e] rounded-2xl border border-white/5 overflow-hidden shadow-lg animate-in fade-in slide-in-from-bottom-4">
                <div className="p-6 border-b border-white/5 flex items-center justify-between bg-black/10">
                   <h3 className="text-lg font-medium text-gray-200 flex items-center gap-2">
                     <Edit3 size={20} className="text-indigo-400" /> 新建汇报
                   </h3>
                   <div className="flex items-center gap-2">
                     {(['daily', 'weekly', 'monthly'] as const).map(t => (
                       <button
                         key={t}
                         onClick={() => setNewReportType(t)}
                         className={cn("px-4 py-1.5 rounded-full text-sm font-medium transition-colors border", newReportType === t ? getTypeColor(t) : "bg-transparent text-gray-400 border-white/10 hover:border-white/20")}
                       >
                         {getTypeStr(t)}
                       </button>
                     ))}
                   </div>
                </div>
                <div className="p-6 space-y-6">
                   <div className="space-y-2">
                     <label className="text-sm font-medium text-gray-400 flex items-center gap-2">本期工作内容 <span className="text-red-400">*</span></label>
                     <textarea 
                       value={newContent}
                       onChange={e => setNewContent(e.target.value)}
                       placeholder={`请填写您的${getTypeStr(newReportType)}内容...`}
                       className="w-full bg-[#2b2b2d] border border-white/5 focus:border-indigo-500/50 rounded-xl p-4 text-gray-200 outline-none resize-none h-40 transition-colors custom-scrollbar"
                     />
                   </div>
                   <div className="space-y-2">
                     <label className="text-sm font-medium text-gray-400">下期工作计划</label>
                     <textarea 
                       value={newPlan}
                       onChange={e => setNewPlan(e.target.value)}
                       placeholder={`请填写下期计划（可选）...`}
                       className="w-full bg-[#2b2b2d] border border-white/5 focus:border-indigo-500/50 rounded-xl p-4 text-gray-200 outline-none resize-none h-32 transition-colors custom-scrollbar"
                     />
                   </div>
                   
                   <div className="flex justify-end gap-3 pt-4 border-t border-white/5">
                     <button onClick={() => setShowEditor(false)} className="px-6 py-2 rounded-xl text-gray-400 hover:text-gray-200 hover:bg-white/5 transition-colors font-medium">取消</button>
                     <button onClick={handleSubmit} className="px-6 py-2 rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white font-medium transition-all shadow-lg shadow-indigo-500/20 flex items-center gap-2">
                       <Send size={16} /> 提交汇报
                     </button>
                   </div>
                </div>
             </div>
           ) : (
             <div className="space-y-4">
                {displayList.map(item => (
                  <div key={item.id} className="bg-[#1e1e1e] rounded-2xl border border-white/5 overflow-hidden hover:border-white/10 transition-all cursor-pointer group shadow-sm hover:shadow-md">
                     <div className="p-5 flex items-start gap-4">
                        <div className="w-12 h-12 rounded-full bg-indigo-500/10 flex items-center justify-center shrink-0 text-indigo-400 font-medium text-lg">
                           {item.author[0]}
                        </div>
                        <div className="flex-1 min-w-0">
                           <div className="flex items-center justify-between mb-1">
                              <div className="flex items-center gap-3">
                                <span className="font-medium text-gray-200 text-base">{item.author}的{getTypeStr(item.type)}</span>
                                <span className={cn("px-2.5 py-0.5 rounded-full text-xs border font-medium flex items-center gap-1", getTypeColor(item.type))}>
                                   {getTypeIcon(item.type)} {getTypeStr(item.type)}
                                </span>
                              </div>
                              <span className="text-sm text-gray-500">{item.date}</span>
                           </div>
                           <div className="mt-4 space-y-4">
                             <div className="bg-[#2b2b2d]/50 p-4 rounded-xl border border-white/5">
                                <h4 className="text-xs font-medium text-gray-500 mb-2">已完成工作</h4>
                                <p className="text-sm text-gray-300 whitespace-pre-wrap leading-relaxed">{item.content}</p>
                             </div>
                             <div className="bg-[#2b2b2d]/50 p-4 rounded-xl border border-white/5">
                                <h4 className="text-xs font-medium text-gray-500 mb-2">后续计划</h4>
                                <p className="text-sm text-gray-300 whitespace-pre-wrap leading-relaxed">{item.plan}</p>
                             </div>
                           </div>
                        </div>
                     </div>
                  </div>
                ))}
             </div>
           )}
        </div>
      </div>
    </div>
  );
};

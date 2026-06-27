import React from 'react';
import { cn } from '@sdkwork/im-pc-commons';
import { ArrowLeft, User, Paperclip, CheckCircle, Clock } from 'lucide-react';
import { ApprovalItem } from '../services/ApprovalsService';

interface ApprovalDetailPanelProps {
  selectedItem: ApprovalItem;
  setSelectedId: (id: string | null) => void;
  comment: string;
  setComment: (val: string) => void;
  handleAction: (action: 'approve' | 'reject') => void;
}

export const ApprovalDetailPanel: React.FC<ApprovalDetailPanelProps> = ({
  selectedItem,
  setSelectedId,
  comment,
  setComment,
  handleAction
}) => {
  return (
    <div className="flex-1 flex flex-col bg-[#1e1e1e] min-w-0 overflow-y-auto custom-scrollbar relative animate-in slide-in-from-right-4">
       <div className="p-6 max-w-3xl mx-auto w-full space-y-8 pb-32">
          <button onClick={() => setSelectedId(null)} className="lg:hidden flex items-center gap-1 text-sm text-gray-400 hover:text-gray-200 transition-colors mb-4">
            <ArrowLeft size={16} /> 返回
          </button>
          
          <div className="flex items-start justify-between">
             <div className="flex items-center gap-4">
               <div className="w-12 h-12 rounded-full bg-gradient-to-br from-indigo-500/30 to-purple-500/30 flex items-center justify-center">
                  <User size={24} className="text-indigo-400" />
               </div>
               <div>
                 <h2 className="text-xl font-medium text-gray-100">{selectedItem.applicant.name}的{selectedItem.title}</h2>
                 <div className="text-sm text-gray-500 mt-1 flex items-center gap-3">
                    <span>审批编号: {selectedItem.id}</span>
                    <span className="w-1 h-1 rounded-full bg-gray-600"></span>
                    <span>{selectedItem.submitTime}</span>
                 </div>
               </div>
             </div>
             <div className={cn("px-3 py-1 rounded-full text-xs font-medium border", selectedItem.status === 'pending' ? "bg-amber-500/10 text-amber-500 border-amber-500/20" : selectedItem.status === 'approved' ? "bg-green-500/10 text-green-500 border-green-500/20" : "bg-red-500/10 text-red-500 border-red-500/20")}>
                {selectedItem.status === 'pending' ? '待审批' : selectedItem.status === 'approved' ? '已同意' : '已拒绝'}
             </div>
          </div>

          <div className="bg-[#2b2b2d]/50 rounded-2xl p-6 border border-white/5 space-y-6">
             <div>
               <h3 className="text-xs font-medium text-gray-500 uppercase tracking-wider mb-2">详细说明</h3>
               <p className="text-sm text-gray-300 leading-relaxed">{selectedItem.description}</p>
             </div>
             {selectedItem.amount && (
               <div>
                 <h3 className="text-xs font-medium text-gray-500 uppercase tracking-wider mb-2">报销金额</h3>
                 <p className="text-lg font-medium text-emerald-400">¥{selectedItem.amount.toLocaleString('zh-CN', {minimumFractionDigits: 2})}</p>
               </div>
             )}
             {selectedItem.attachments && selectedItem.attachments.length > 0 && (
               <div>
                 <h3 className="text-xs font-medium text-gray-500 uppercase tracking-wider mb-3">附件 ({selectedItem.attachments.length})</h3>
                 <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
                    {selectedItem.attachments.map((file, idx) => (
                      <div key={idx} className="flex items-center gap-3 p-3 bg-[#1e1e1e] rounded-xl border border-white/5 hover:border-indigo-500/30 transition-colors cursor-pointer group">
                         <div className="w-10 h-10 rounded-lg bg-indigo-500/10 flex items-center justify-center text-indigo-400">
                            <Paperclip size={18} />
                         </div>
                         <div className="flex-1 min-w-0">
                            <div className="text-sm text-gray-300 truncate group-hover:text-indigo-400 transition-colors">{file.name}</div>
                            <div className="text-xs text-gray-500">{file.size}</div>
                         </div>
                      </div>
                    ))}
                 </div>
               </div>
             )}
          </div>

          {/* Workflow line */}
          <div>
             <h3 className="text-sm font-medium text-gray-200 mb-6 font-medium">审批流程</h3>
             <div className="space-y-6 relative before:absolute before:inset-0 before:ml-[19px] before:-translate-x-px md:before:mx-auto md:before:translate-x-0 before:h-full before:w-0.5 before:bg-gradient-to-b before:from-transparent before:via-white/10 before:to-transparent">
                <div className="relative flex items-center justify-between md:justify-normal md:odd:flex-row-reverse group is-active">
                   <div className="flex items-center justify-center w-10 h-10 rounded-full border-4 border-[#1e1e1e] bg-indigo-500 text-white shrink-0 md:order-1 md:group-odd:-translate-x-1/2 md:group-even:translate-x-1/2 shadow-lg z-10">
                      <CheckCircle size={16} />
                   </div>
                   <div className="w-[calc(100%-4rem)] md:w-[calc(50%-2.5rem)] bg-[#2b2b2d] p-4 rounded-xl border border-white/5 shadow-sm">
                      <div className="flex items-center justify-between mb-1">
                         <span className="font-medium text-sm text-gray-200">提交申请</span>
                         <span className="text-xs text-gray-500">刚刚</span>
                      </div>
                      <div className="text-xs text-gray-400">{selectedItem.applicant.name} (发起人)</div>
                   </div>
                </div>
                <div className="relative flex items-center justify-between md:justify-normal md:odd:flex-row-reverse group">
                   <div className="flex items-center justify-center w-10 h-10 rounded-full border-4 border-[#1e1e1e] bg-[#3a3a3a] text-gray-400 shrink-0 md:order-1 md:group-odd:-translate-x-1/2 md:group-even:translate-x-1/2 z-10">
                      <Clock size={16} />
                   </div>
                   <div className="w-[calc(100%-4rem)] md:w-[calc(50%-2.5rem)] bg-[#2b2b2d] p-4 rounded-xl border border-white/5 border-indigo-500/30">
                      <div className="flex items-center justify-between mb-1">
                         <span className="font-medium text-sm text-gray-200">直接主管审批</span>
                         <span className="text-xs text-amber-500">审批中</span>
                      </div>
                      <div className="text-xs text-gray-400">我 (当前处理人)</div>
                   </div>
                </div>
             </div>
          </div>
       </div>

       {/* Footer Actions */}
       {selectedItem.status === 'pending' && (
          <div className="absolute bottom-0 left-0 right-0 p-4 bg-[#1e1e1e]/90 backdrop-blur-md border-t border-white/10 shadow-[0_-10px_30px_rgba(0,0,0,0.2)]">
            <div className="max-w-3xl mx-auto flex flex-col sm:flex-row gap-4">
               <input 
                 type="text" 
                 value={comment}
                 onChange={e => setComment(e.target.value)}
                 placeholder="审批意见（选填）..." 
                 className="flex-1 bg-[#2b2b2d] border border-white/10 rounded-xl px-4 py-3 text-sm text-gray-200 outline-none focus:border-indigo-500 transition-colors"
               />
               <div className="flex gap-2 shrink-0">
                 <button onClick={() => handleAction('reject')} className="px-6 py-3 rounded-xl bg-red-500/10 hover:bg-red-500/20 text-red-500 text-sm font-medium transition-colors border border-red-500/20">
                    拒绝
                 </button>
                 <button onClick={() => handleAction('approve')} className="px-6 py-3 rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white text-sm font-medium transition-all shadow-lg shadow-indigo-500/20">
                    同意
                 </button>
               </div>
            </div>
          </div>
       )}
    </div>
  );
};

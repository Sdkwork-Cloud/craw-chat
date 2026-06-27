import React from 'react';
import { CheckCircle, XCircle } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';
import { ApprovalItem } from '../services/ApprovalsService';

interface CreateApprovalModalProps {
  show: boolean;
  onClose: () => void;
  newApprovalType: 'leave' | 'reimbursement' | 'purchase' | 'other';
  setNewApprovalType: (t: 'leave' | 'reimbursement' | 'purchase' | 'other') => void;
  newApprovalTitle: string;
  setNewApprovalTitle: (v: string) => void;
  newApprovalAmount: string;
  setNewApprovalAmount: (v: string) => void;
  newApprovalDesc: string;
  setNewApprovalDesc: (v: string) => void;
  onSubmit: () => void;
}

export const CreateApprovalModal: React.FC<CreateApprovalModalProps> = ({
  show, onClose, newApprovalType, setNewApprovalType,
  newApprovalTitle, setNewApprovalTitle,
  newApprovalAmount, setNewApprovalAmount,
  newApprovalDesc, setNewApprovalDesc,
  onSubmit
}) => {
  if (!show) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-md animate-in fade-in">
      <div className="bg-[#1e1e1e] border border-white/10 rounded-2xl w-full max-w-lg shadow-2xl flex flex-col overflow-hidden animate-in zoom-in-95">
        <div className="p-6 border-b border-white/10 flex items-center justify-between bg-black/20">
          <h3 className="text-lg font-medium text-gray-100 flex items-center gap-2">
             <CheckCircle size={20} className="text-indigo-400" /> 发起新审批
          </h3>
          <button onClick={onClose} className="text-gray-400 hover:text-gray-200 transition-colors">
            <XCircle size={20} />
          </button>
        </div>
        <div className="p-6 space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-400 mb-1.5">审批类型</label>
            <div className="flex gap-2">
               {['leave', 'reimbursement', 'purchase', 'other'].map(t => (
                 <button 
                   key={t}
                   onClick={() => setNewApprovalType(t as any)}
                   className={cn("px-4 py-2 rounded-lg text-sm font-medium transition-colors border", newApprovalType === t ? "bg-indigo-500/20 text-indigo-400 border-indigo-500/30" : "bg-[#2b2b2d] text-gray-400 border-transparent hover:border-white/10")}
                 >
                   {t === 'leave' ? '请假' : t === 'reimbursement' ? '报销' : t === 'purchase' ? '采购' : '其他'}
                 </button>
               ))}
            </div>
          </div>
          <div>
             <label className="block text-sm font-medium text-gray-400 mb-1.5">标题 <span className="text-red-400">*</span></label>
             <input 
               type="text" 
               value={newApprovalTitle}
               onChange={e => setNewApprovalTitle(e.target.value)}
               className="w-full bg-[#2b2b2d] border border-white/5 focus:border-indigo-500/50 rounded-xl px-4 py-2 text-gray-200 outline-none transition-all"
               placeholder="例如：年假申请-3天"
             />
          </div>
          {(newApprovalType === 'reimbursement' || newApprovalType === 'purchase') && (
            <div>
               <label className="block text-sm font-medium text-gray-400 mb-1.5">金额 (¥)</label>
               <input 
                 type="number" 
                 value={newApprovalAmount}
                 onChange={e => setNewApprovalAmount(e.target.value)}
                 className="w-full bg-[#2b2b2d] border border-white/5 focus:border-indigo-500/50 rounded-xl px-4 py-2 text-gray-200 outline-none transition-all"
                 placeholder="请填写金额"
               />
            </div>
          )}
          <div>
             <label className="block text-sm font-medium text-gray-400 mb-1.5">详细说明 <span className="text-red-400">*</span></label>
             <textarea 
               rows={4}
               value={newApprovalDesc}
               onChange={e => setNewApprovalDesc(e.target.value)}
               className="w-full bg-[#2b2b2d] border border-white/5 focus:border-indigo-500/50 rounded-xl px-4 py-2 text-gray-200 outline-none transition-all resize-none custom-scrollbar"
               placeholder="请填写详细的原因或说明..."
             />
          </div>
        </div>
        <div className="px-6 py-4 border-t border-white/10 flex justify-end gap-3 bg-black/20">
           <button onClick={onClose} className="px-5 py-2 rounded-xl text-gray-400 hover:text-gray-200 hover:bg-white/5 font-medium transition-colors">取消</button>
           <button onClick={onSubmit} className="px-5 py-2 rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white font-medium transition-all shadow-lg shadow-indigo-500/20">提交申请</button>
        </div>
      </div>
    </div>
  );
};

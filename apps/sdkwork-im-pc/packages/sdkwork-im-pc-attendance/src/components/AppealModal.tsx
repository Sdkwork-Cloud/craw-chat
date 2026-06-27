import React from 'react';
import { motion } from 'motion/react';

interface AppealModalProps {
  showAppealModal: boolean;
  setShowAppealModal: (v: boolean) => void;
  appealItem: {id: string, date: string, type: string} | null;
  appealReason: string;
  setAppealReason: (v: string) => void;
  submitAppeal: () => void;
}

export const AppealModal: React.FC<AppealModalProps> = ({
  showAppealModal, setShowAppealModal, appealItem, appealReason, setAppealReason, submitAppeal
}) => {
  if (!showAppealModal) return null;
  
  return (
    <motion.div initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }} className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-md">
       <motion.div initial={{ scale: 0.95, opacity: 0 }} animate={{ scale: 1, opacity: 1 }} exit={{ scale: 0.95, opacity: 0 }} transition={{ type: "spring", damping: 25, stiffness: 300 }} className="bg-[#1e1e1e] border border-white/10 rounded-2xl w-full max-w-sm shadow-2xl p-6 relative">
          <h3 className="text-lg font-medium text-white mb-4">异常申诉/补签</h3>
          <div className="mb-4 text-sm text-gray-400 bg-white/5 p-3 rounded-lg border border-white/5">
             <div>时间: <span className="text-gray-200">{appealItem?.date}</span></div>
             <div>类型: <span className="text-red-400">{appealItem?.type}</span></div>
          </div>
          <textarea
            value={appealReason}
            onChange={e => setAppealReason(e.target.value)}
            placeholder="请输入补签理由..."
            rows={3}
            className="w-full bg-[#181818] border border-white/10 rounded-xl px-4 py-2.5 text-sm text-gray-200 outline-none focus:border-indigo-500/50 resize-none mb-6"
          />
          <div className="flex justify-end gap-3">
             <button className="px-5 py-2 text-sm text-gray-300 hover:bg-white/5 rounded-xl transition-colors" onClick={() => setShowAppealModal(false)}>取消</button>
             <button 
               className="px-5 py-2 text-sm bg-indigo-600 hover:bg-indigo-500 text-white rounded-xl transition-colors font-medium disabled:opacity-50" 
               disabled={!appealReason.trim()}
               onClick={submitAppeal}
             >
               提交申请
             </button>
          </div>
       </motion.div>
    </motion.div>
  );
};

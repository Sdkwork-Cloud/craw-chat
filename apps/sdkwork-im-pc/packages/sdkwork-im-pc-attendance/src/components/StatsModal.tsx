import React from 'react';
import { motion } from 'motion/react';
import { History, Calendar } from 'lucide-react';
import { AttendanceStats, AttendanceRecord } from '../services/AttendanceService';

interface StatsModalProps {
  showStatsModal: boolean;
  setShowStatsModal: (v: boolean) => void;
  stats: AttendanceStats | null;
  records: AttendanceRecord[];
  setAppealItem: (item: any) => void;
  setShowAppealModal: (v: boolean) => void;
}

export const StatsModal: React.FC<StatsModalProps> = ({
  showStatsModal, setShowStatsModal, stats, records, setAppealItem, setShowAppealModal
}) => {
  if (!showStatsModal) return null;
  
  return (
    <motion.div initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }} className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-md">
      <motion.div initial={{ scale: 0.95, opacity: 0 }} animate={{ scale: 1, opacity: 1 }} exit={{ scale: 0.95, opacity: 0 }} transition={{ type: "spring", damping: 25, stiffness: 300 }} className="bg-[#1e1e1e] border border-white/10 rounded-2xl w-full max-w-2xl shadow-2xl flex flex-col overflow-hidden h-[600px]">
         <div className="p-6 border-b border-white/10 flex items-center justify-between bg-black/20 shrink-0">
           <h3 className="text-lg font-medium text-gray-100 flex items-center gap-2">
             <History size={20} className="text-indigo-400" /> 考勤统计与补签
           </h3>
           <button onClick={() => setShowStatsModal(false)} className="text-gray-400 hover:text-gray-200 transition-colors">
             <Calendar size={20} />
           </button>
         </div>
         
         <div className="flex-1 overflow-y-auto custom-scrollbar p-6 space-y-6">
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
               <div className="bg-[#2b2b2d] p-4 rounded-xl border border-white/5">
                  <div className="text-gray-500 text-xs mb-1">本月出勤</div>
                  <div className="text-2xl font-semibold text-gray-200">{stats?.presentDays || 0}<span className="text-sm text-gray-500 font-normal ml-1">天</span></div>
               </div>
               <div className="bg-[#2b2b2d] p-4 rounded-xl border border-white/5">
                  <div className="text-gray-500 text-xs mb-1">迟到</div>
                  <div className="text-2xl font-semibold text-amber-500">{stats?.lateTimes || 0}<span className="text-sm text-amber-500/50 font-normal ml-1">次</span></div>
               </div>
               <div className="bg-[#2b2b2d] p-4 rounded-xl border border-white/5">
                  <div className="text-gray-500 text-xs mb-1">早退</div>
                  <div className="text-2xl font-semibold text-amber-500">{stats?.earlyLeaveTimes || 0}<span className="text-sm text-amber-500/50 font-normal ml-1">次</span></div>
               </div>
               <div className="bg-[#2b2b2d] p-4 rounded-xl border border-white/5">
                  <div className="text-gray-500 text-xs mb-1">缺卡</div>
                  <div className="text-2xl font-semibold text-red-400">{stats?.missedPunches || 0}<span className="text-sm text-red-400/50 font-normal ml-1">次</span></div>
               </div>
            </div>

            <div>
               <h4 className="text-sm font-medium text-gray-400 mb-3 pl-1">考勤异常记录</h4>
               <div className="bg-[#2b2b2d] border border-white/5 rounded-xl overflow-hidden">
                  {records.map(record => (
                    <div key={record.id} className="p-4 border-b border-white/5 flex items-center justify-between">
                      <div>
                         <div className="text-sm text-gray-200 font-medium">{record.date}</div>
                         <div className={`text-xs mt-1 ${record.status === 'pending' ? 'text-red-400' : 'text-amber-500'}`}>{record.type}</div>
                      </div>
                      {record.status === 'pending' ? (
                        <button 
                          className="bg-indigo-500/10 hover:bg-indigo-500/20 text-indigo-400 px-4 py-1.5 rounded-lg text-xs font-medium transition-colors"
                          onClick={() => {
                             setAppealItem(record);
                             setShowAppealModal(true);
                          }}
                        >
                          发起补签
                        </button>
                      ) : (
                        <span className="text-xs text-gray-500">已处理</span>
                      )}
                    </div>
                  ))}
               </div>
            </div>
         </div>
         <div className="px-6 py-4 border-t border-white/10 flex justify-end gap-3 bg-black/20 shrink-0">
           <button onClick={() => setShowStatsModal(false)} className="px-5 py-2 rounded-xl bg-white/5 hover:bg-white/10 text-white font-medium transition-colors">关闭</button>
         </div>
      </motion.div>
    </motion.div>
  );
};

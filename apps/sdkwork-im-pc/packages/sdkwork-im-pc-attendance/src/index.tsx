import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { 
  MapPin, Clock, Calendar, ChevronLeft, ChevronRight, Activity, 
  Map, History, Fingerprint
} from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';
import { toast } from '@sdkwork/im-pc-chat';
import { attendanceService, AttendanceStats, AttendanceRecord, AttendanceStatus } from './services/AttendanceService';
import { StatsModal } from './components/StatsModal';
import { AppealModal } from './components/AppealModal';

export const AttendanceView: React.FC = () => {
  const [currentTime, setCurrentTime] = useState(new Date());
  
  const [status, setStatus] = useState<AttendanceStatus | null>(null);
  const [stats, setStats] = useState<AttendanceStats | null>(null);
  const [records, setRecords] = useState<AttendanceRecord[]>([]);

  const [showStatsModal, setShowStatsModal] = useState(false);
  const [showAppealModal, setShowAppealModal] = useState(false);
  const [appealReason, setAppealReason] = useState('');
  const [appealItem, setAppealItem] = useState<{id: string, date: string, type: string} | null>(null);

  useEffect(() => {
    const timer = setInterval(() => setCurrentTime(new Date()), 1000);
    return () => clearInterval(timer);
  }, []);

  useEffect(() => {
    const fetchData = async () => {
      setStatus(await attendanceService.getCurrentStatus());
      setStats(await attendanceService.getStats());
      setRecords(await attendanceService.getRecords());
    };
    fetchData();
  }, []);

  const formatTime = (date: Date) => {
    return date.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit', second: '2-digit' });
  };
  
  const formatDate = (date: Date) => {
    return date.toLocaleDateString('zh-CN', { year: 'numeric', month: 'long', day: 'numeric', weekday: 'long' });
  };

  const handlePunch = async () => {
    if (!status) return;
    
    if (!status.isCheckedIn) {
      const result = await attendanceService.punchIn();
      if (result.success) {
        setStatus({ ...status, isCheckedIn: true, checkInTime: result.time });
        toast('上班打卡成功', 'success');
      }
    } else if (!status.isCheckedOut) {
      const result = await attendanceService.punchOut();
      if (result.success) {
        setStatus({ ...status, isCheckedOut: true, checkOutTime: result.time });
        toast('下班打卡成功', 'success');
      }
    } else {
      toast('今日打卡已完成', 'success');
    }
  };

  const submitAppeal = async () => {
    if (appealItem && appealReason.trim()) {
      await attendanceService.submitAppeal(appealItem.id, appealReason);
      setShowAppealModal(false); 
      setAppealReason('');
      toast('补签申请已提交至审批中心', 'success'); 
    }
  };

  if (!status) return null;

  return (
    <motion.div initial={{ opacity: 0 }} animate={{ opacity: 1 }} className="flex-1 flex flex-col bg-[#1e1e1e] min-w-0">
      {/* Header */}
      <div className="h-16 border-b border-white/5 flex items-center justify-between px-6 shrink-0 bg-[#1e1e1e]/80 backdrop-blur-md">
        <div className="flex items-center gap-6">
          <h2 className="text-lg font-medium text-gray-200">打卡</h2>
        </div>
        <div className="flex items-center gap-3">
           <button onClick={() => setShowStatsModal(true)} className="flex items-center gap-2 text-gray-400 hover:text-gray-200 px-3 py-1.5 rounded-lg text-sm font-medium transition-colors bg-white/5 hover:bg-white/10">
              <History size={16} /> 考勤统计
           </button>
        </div>
      </div>

      <AnimatePresence>
        <StatsModal
          showStatsModal={showStatsModal}
          setShowStatsModal={setShowStatsModal}
          stats={stats}
          records={records}
          setAppealItem={setAppealItem}
          setShowAppealModal={setShowAppealModal}
        />
      </AnimatePresence>

      {/* Main Content */}
      <div className="flex-1 overflow-y-auto custom-scrollbar p-6 bg-[#181818] flex items-center justify-center">
         <div className="max-w-4xl w-full grid grid-cols-1 md:grid-cols-2 gap-6">
            
            {/* Left: Punch Card */}
            <div className="bg-[#1e1e1e] rounded-3xl border border-white/5 p-8 flex flex-col items-center shadow-lg relative overflow-hidden">
               <div className="absolute top-0 inset-x-0 h-32 bg-gradient-to-b from-indigo-500/10 to-transparent"></div>
               
               <div className="text-center mb-8 relative z-10">
                 <div className="text-gray-400 text-sm mb-2">{formatDate(currentTime)}</div>
                 <div className="text-5xl font-mono text-gray-100 tracking-wider font-semibold">{formatTime(currentTime)}</div>
               </div>

               <div className="w-full space-y-4 mb-10 relative z-10">
                  <div className="bg-[#2b2b2d] rounded-2xl p-4 border border-white/5 flex items-center justify-between">
                     <div className="flex items-center gap-3">
                        <div className="w-10 h-10 rounded-full bg-blue-500/10 text-blue-400 flex items-center justify-center">
                           <Clock size={20} />
                        </div>
                        <div>
                          <div className="text-sm font-medium text-gray-200">上班时间 09:00</div>
                          <div className={cn("text-xs mt-0.5", status.checkInTime ? "text-emerald-400 font-medium" : "text-gray-500")}>
                            {status.checkInTime ? `打卡时间 ${status.checkInTime}` : '未打卡'}
                          </div>
                        </div>
                     </div>
                     {status.checkInTime && <span className="px-2 py-1 rounded-md bg-emerald-500/10 text-emerald-400 text-xs border border-emerald-500/20">正常</span>}
                  </div>

                  <div className="bg-[#2b2b2d] rounded-2xl p-4 border border-white/5 flex items-center justify-between">
                     <div className="flex items-center gap-3">
                        <div className="w-10 h-10 rounded-full bg-amber-500/10 text-amber-400 flex items-center justify-center">
                           <Clock size={20} />
                        </div>
                        <div>
                          <div className="text-sm font-medium text-gray-200">下班时间 18:00</div>
                          <div className={cn("text-xs mt-0.5", status.checkOutTime ? "text-emerald-400 font-medium" : "text-gray-500")}>
                            {status.checkOutTime ? `打卡时间 ${status.checkOutTime}` : '未打卡'}
                          </div>
                        </div>
                     </div>
                     {status.checkOutTime && <span className="px-2 py-1 rounded-md bg-emerald-500/10 text-emerald-400 text-xs border border-emerald-500/20">正常</span>}
                  </div>
               </div>

               <button 
                 onClick={handlePunch}
                 disabled={status.isCheckedIn && status.isCheckedOut}
                 className={cn(
                   "relative w-40 h-40 rounded-full flex flex-col items-center justify-center gap-2 transition-all duration-300 z-10 group outline-none",
                   status.isCheckedIn && status.isCheckedOut 
                     ? "bg-[#2b2b2d] text-gray-500 cursor-not-allowed border-4 border-[#3a3a3a]" 
                     : "bg-indigo-600 hover:bg-indigo-500 text-white cursor-pointer shadow-[0_0_40px_rgba(79,70,229,0.3)] hover:shadow-[0_0_60px_rgba(79,70,229,0.5)] active:scale-95 border-4 border-indigo-400/30"
                 )}
               >
                  <Fingerprint size={48} className={cn("transition-transform duration-300", (!status.isCheckedIn || !status.isCheckedOut) && "group-hover:scale-110")} />
                  <span className="text-lg font-medium tracking-widest">{status.isCheckedIn && status.isCheckedOut ? '已完成' : !status.isCheckedIn ? '上班打卡' : '下班打卡'}</span>
                  {(!status.isCheckedIn || !status.isCheckedOut) && <div className="absolute inset-0 rounded-full border border-white/20 animate-ping opacity-20"></div>}
               </button>

               <div className="flex items-center justify-center gap-2 mt-8 text-gray-500 text-sm">
                  <MapPin size={16} /> <span>当前位置: {status.location}</span>
                  <span className="text-indigo-400 ml-2 cursor-pointer hover:underline">重新定位</span>
               </div>
            </div>

            {/* Right: Map & Stats */}
            <div className="flex flex-col gap-6">
               <div className="flex-1 bg-[#1e1e1e] rounded-3xl border border-white/5 p-6 flex flex-col shadow-sm">
                  <h3 className="text-md font-medium text-gray-200 mb-4 flex items-center gap-2"><Map size={18} className="text-indigo-400"/> 打卡范围</h3>
                  <div className="flex-1 rounded-2xl bg-[#2b2b2d] border border-white/5 relative overflow-hidden flex items-center justify-center">
                     {/* Pseudo map background */}
                     <div className="absolute inset-0 opacity-10" style={{ backgroundImage: 'radial-gradient(circle at center, #4f46e5 1px, transparent 1px)', backgroundSize: '20px 20px' }}></div>
                     <div className="relative w-32 h-32 rounded-full border border-indigo-500/30 bg-indigo-500/10 flex items-center justify-center">
                       <div className="absolute w-full h-full rounded-full bg-indigo-500/5 animate-ping"></div>
                       <MapPin size={24} className="text-indigo-400" />
                     </div>
                     <div className="absolute bottom-4 left-4 right-4 bg-[#1e1e1e]/90 backdrop-blur-md p-3 rounded-xl border border-white/10 text-xs text-gray-300 text-center">
                       {status.inRange ? '已进入考勤范围，可进行打卡操作' : '未进入考勤范围'}
                     </div>
                  </div>
               </div>

               <div className="h-48 bg-[#1e1e1e] rounded-3xl border border-white/5 p-6 shadow-sm">
                  <h3 className="text-md font-medium text-gray-200 mb-4 flex items-center gap-2"><Activity size={18} className="text-emerald-400"/> 本月概况</h3>
                  <div className="grid grid-cols-3 gap-4">
                     <div className="bg-[#2b2b2d] p-4 rounded-2xl border border-white/5 text-center">
                       <div className="text-2xl font-semibold text-gray-100 mb-1">{stats?.presentDays || 0}</div>
                       <div className="text-xs text-gray-500">出勤天数</div>
                     </div>
                     <div className="bg-[#2b2b2d] p-4 rounded-2xl border border-white/5 text-center">
                       <div className="text-2xl font-semibold text-gray-100 mb-1">{(stats?.lateTimes || 0) + (stats?.earlyLeaveTimes || 0)}</div>
                       <div className="text-xs text-gray-500">迟到/早退</div>
                     </div>
                     <div className="bg-[#2b2b2d] p-4 rounded-2xl border border-white/5 text-center">
                       <div className="text-2xl font-semibold text-gray-100 mb-1">{stats?.missedPunches || 0}</div>
                       <div className="text-xs text-gray-500">缺卡次数</div>
                     </div>
                  </div>
               </div>
            </div>

         </div>
      </div>
      <AnimatePresence>
        <AppealModal
          showAppealModal={showAppealModal}
          setShowAppealModal={setShowAppealModal}
          appealItem={appealItem}
          appealReason={appealReason}
          setAppealReason={setAppealReason}
          submitAppeal={submitAppeal}
        />
      </AnimatePresence>
    </motion.div>
  );
};

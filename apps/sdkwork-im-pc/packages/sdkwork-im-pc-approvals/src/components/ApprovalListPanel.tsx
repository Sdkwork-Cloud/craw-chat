import React, { ReactNode } from 'react';
import { cn } from '@sdkwork/im-pc-commons';
import { Search, Filter, CheckCircle } from 'lucide-react';
import { ApprovalItem } from '../services/ApprovalsService';

interface ApprovalListPanelProps {
  displayList: ApprovalItem[];
  selectedId: string | null;
  setSelectedId: (id: string | null) => void;
  getTypeIcon: (type: string) => ReactNode;
  getStatusIcon: (status: string) => ReactNode;
}

export const ApprovalListPanel: React.FC<ApprovalListPanelProps> = ({
  displayList,
  selectedId,
  setSelectedId,
  getTypeIcon,
  getStatusIcon
}) => {
  return (
    <div className={cn("flex flex-col h-full bg-[#181818] transition-all duration-300 border-r border-white/5", selectedId ? "w-[360px]" : "w-full lg:w-[400px]")}>
       <div className="p-4 border-b border-white/5 shrink-0 flex items-center gap-2">
         <div className="relative flex-1">
            <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500" />
            <input type="text" placeholder="搜索审批..." className="w-full bg-[#2b2b2d] border border-transparent focus:border-indigo-500/50 rounded-lg py-2 pl-9 pr-4 text-sm text-gray-200 outline-none transition-all placeholder:text-gray-500" />
         </div>
         <button className="p-2 text-gray-400 hover:text-gray-200 bg-[#2b2b2d] rounded-lg transition-colors" title="过滤">
           <Filter size={18} />
         </button>
       </div>
       <div className="flex-1 overflow-y-auto custom-scrollbar">
          {displayList.length > 0 ? displayList.map(item => (
            <div 
               key={item.id} 
               onClick={() => setSelectedId(item.id)}
               className={cn("p-4 border-b border-white/5 cursor-pointer hover:bg-white/5 transition-all group relative", selectedId === item.id ? "bg-indigo-500/10 border-l-2 border-l-indigo-500" : "border-l-2 border-l-transparent")}
            >
               <div className="flex items-start justify-between mb-2">
                 <div className="flex items-center gap-2">
                    <div className="w-8 h-8 rounded-full bg-[#2b2b2d] flex items-center justify-center text-gray-400 shrink-0">
                       {getTypeIcon(item.type)}
                    </div>
                    <div>
                      <div className="text-sm font-medium text-gray-200">{item.applicant.name}的{item.title}</div>
                      <div className="text-[11px] text-gray-500">{item.submitTime}</div>
                    </div>
                 </div>
                 {getStatusIcon(item.status)}
               </div>
               <div className="text-xs text-gray-400 line-clamp-1 ml-10">{item.description}</div>
               {item.amount && <div className="text-xs font-medium text-emerald-400 mt-2 ml-10">¥{item.amount.toLocaleString('zh-CN', {minimumFractionDigits: 2})}</div>}
            </div>
          )) : (
            <div className="h-full flex flex-col items-center justify-center text-gray-500">
              <CheckCircle size={48} className="mb-4 opacity-20" />
              <p>没有待处理的审批</p>
            </div>
          )}
       </div>
    </div>
  );
};

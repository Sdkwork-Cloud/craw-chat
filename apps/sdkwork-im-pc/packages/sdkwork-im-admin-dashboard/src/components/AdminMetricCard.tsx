import React from 'react';
import { cn } from '@sdkwork/im-pc-commons';

export const AdminMetricCard = ({ title, value, trend, isUp, icon: Icon, color }: any) => {
  const colorMap: Record<string, string> = {
    indigo: "text-indigo-400 bg-indigo-500/10",
    emerald: "text-emerald-400 bg-emerald-500/10",
    blue: "text-blue-400 bg-blue-500/10",
    amber: "text-amber-400 bg-amber-500/10"
  };

  return (
    <div className="bg-admin-bg-panel p-5 rounded-2xl border border-admin-border shadow-lg flex flex-col relative overflow-hidden group">
      <div className={cn("absolute -right-6 -top-6 w-24 h-24 rounded-full blur-[40px] opacity-20 group-hover:opacity-40 transition-opacity", colorMap[color].split(' ')[1])} />
      
      <div className="flex justify-between items-start mb-4 relative z-10">
        <div className={cn("p-2.5 rounded-xl border border-admin-border", colorMap[color])}>
          <Icon size={20} />
        </div>
        {trend !== "0" && (
          <div className={cn(
            "px-2 py-1 rounded-md text-[10px] font-mono tracking-wider",
            isUp ? "bg-emerald-500/10 text-emerald-400 border border-emerald-500/20" : "bg-rose-500/10 text-rose-400 border border-rose-500/20"
          )}>
            {trend}
          </div>
        )}
      </div>
      <div className="flex flex-col relative z-10">
        <span className="text-[28px] font-bold text-admin-text-main leading-none mb-1 tracking-tight">{value}</span>
        <div className="flex justify-between items-baseline mt-1">
          <span className="text-xs text-admin-text-muted font-medium tracking-wide">{title}</span>
          <span className="text-[10px] text-admin-text-muted font-mono">{trend}</span>
        </div>
      </div>
    </div>
  );
};

import React from 'react';
import { cn } from '@sdkwork/im-pc-commons';

export const MetricCard = ({ title, value, trend, isUp, icon: Icon, color }: any) => {
  const colorMap: Record<string, { main: string; bg: string; ring: string }> = {
    blue: { main: "text-blue-500", bg: "bg-blue-500/10", ring: "ring-blue-500/20" },
    indigo: { main: "text-indigo-500", bg: "bg-indigo-500/10", ring: "ring-indigo-500/20" },
    emerald: { main: "text-emerald-500", bg: "bg-emerald-500/10", ring: "ring-emerald-500/20" },
    amber: { main: "text-amber-500", bg: "bg-amber-500/10", ring: "ring-amber-500/20" },
    purple: { main: "text-purple-500", bg: "bg-purple-500/10", ring: "ring-purple-500/20" }
  };
  const theme = colorMap[color] || colorMap.blue;

  return (
    <div className="bg-console-bg-panel p-5 rounded-2xl border border-console-border shadow-sm flex flex-col relative overflow-hidden group hover:border-console-border-hover transition-all">
      <div className={cn("absolute -right-6 -top-6 w-24 h-24 rounded-full blur-[40px] opacity-30 transition-opacity group-hover:opacity-60", theme.bg)} />
      
      <div className="flex justify-between items-start mb-6 relative z-10">
        <div className={cn("p-2.5 rounded-xl border ring-1 ring-inset shadow-sm flex items-center justify-center", theme.bg, theme.ring, "border-white/10")}>
          <Icon size={20} className={theme.main} />
        </div>
        <div className={cn(
          "px-2.5 py-1 rounded-full text-[11px] font-bold tracking-wider uppercase border",
          isUp ? "bg-emerald-500/10 text-emerald-600 border-emerald-500/20" : "bg-rose-500/10 text-rose-600 border-rose-500/20",
          trend === '安全' && "bg-gray-500/10 text-gray-500 border-gray-500/20"
        )}>
          {trend}
        </div>
      </div>
      <div className="flex flex-col relative z-10 mt-auto">
        <span className="text-[32px] font-bold text-console-text-main leading-none mb-1.5 font-mono tracking-tight">{value}</span>
        <span className="text-[13px] text-console-text-muted font-medium">{title}</span>
      </div>
    </div>
  );
};

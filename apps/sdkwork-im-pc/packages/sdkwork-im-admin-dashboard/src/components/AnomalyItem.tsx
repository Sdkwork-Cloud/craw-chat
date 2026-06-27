import React from 'react';
import { cn } from '@sdkwork/im-pc-commons';

export const AnomalyItem = ({ type, tenant, message, time }: any) => {
  const typeStyles = {
    critical: "text-rose-400 bg-rose-500/10 border-rose-500/20",
    warning: "text-amber-400 bg-amber-500/10 border-amber-500/20",
    info: "text-blue-400 bg-blue-500/10 border-blue-500/20"
  };
  
  return (
    <div className="flex items-center gap-3 p-3 rounded-xl bg-admin-bg-root border border-admin-border hover:bg-admin-bg-hover transition-colors cursor-pointer">
      <div className={cn("px-1.5 py-0.5 rounded font-mono text-[9px] border break-keep shrink-0", typeStyles[type as keyof typeof typeStyles])}>
        {tenant}
      </div>
      <div className="flex-1 min-w-0">
        <div className="text-xs text-admin-text-main leading-tight truncate font-medium">{message}</div>
        <div className="text-[10px] text-admin-text-muted mt-1">{time}</div>
      </div>
    </div>
  );
};

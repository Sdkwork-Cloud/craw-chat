import React from 'react';
import { Shield, Activity, Key, Bell } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';

export const AlertItem = ({ type, message, time }: any) => {
  const typeMap: Record<string, { icon: any; color: string; bg: string; ring: string }> = {
    high: { icon: Shield, color: 'text-rose-500', bg: 'bg-rose-500/10', ring: 'border-rose-500/20' },
    medium: { icon: Activity, color: 'text-amber-500', bg: 'bg-amber-500/10', ring: 'border-amber-500/20' },
    low: { icon: Key, color: 'text-blue-500', bg: 'bg-blue-500/10', ring: 'border-blue-500/20' },
    info: { icon: Bell, color: 'text-gray-500', bg: 'bg-gray-500/10', ring: 'border-gray-500/20' }
  };
  const t = typeMap[type] || typeMap.info;
  const Icon = t.icon;

  return (
    <div className="flex gap-3 items-start p-3 rounded-xl hover:bg-console-bg-root transition-colors border border-transparent hover:border-console-border group/alert">
      <div className={cn("mt-0.5 p-2 rounded-lg border", t.bg, t.color, t.ring)}>
        <Icon size={16} />
      </div>
      <div className="flex-1">
        <p className="text-sm font-medium text-console-text-main leading-tight group-hover/alert:text-blue-500 transition-colors mb-1">{message}</p>
        <p className="text-[11px] text-console-text-muted font-mono">{time}</p>
      </div>
    </div>
  );
};

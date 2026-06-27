import React from 'react';
import { ChartBar } from 'lucide-react';
import { ConsoleContractEmptyState } from '@sdkwork/im-pc-commons';

export const ConsoleAnalytics = () => (
  <div className="bg-console-bg-panel border border-console-border rounded-2xl shadow-sm flex flex-col min-h-[600px] overflow-hidden">
    <div className="p-6 border-b border-console-border flex items-center gap-3">
      <ChartBar size={22} className="text-blue-600" />
      <div>
        <h2 className="text-lg font-bold text-console-text-main">数据分析与报表</h2>
        <p className="text-sm text-console-text-muted mt-1">深度洞察用户活跃度、通信趋势与资源消耗</p>
      </div>
    </div>
    <ConsoleContractEmptyState title="数据分析合约暂未开放" />
  </div>
);

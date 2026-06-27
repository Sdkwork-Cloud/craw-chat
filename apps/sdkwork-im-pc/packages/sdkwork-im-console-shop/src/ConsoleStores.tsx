import React from 'react';
import { Store, Plus } from 'lucide-react';
import { ConsoleContractEmptyState } from '@sdkwork/im-pc-commons';

export const ConsoleStores = () => (
  <div className="bg-console-bg-panel border border-console-border rounded-2xl shadow-sm flex flex-col flex-1 min-h-0 h-full overflow-hidden">
    <div className="flex items-center justify-between p-6 border-b border-console-border shrink-0 bg-console-bg-root/50">
      <div className="flex items-center gap-3">
        <Store size={22} className="text-blue-600" />
        <div>
          <h2 className="text-xl font-bold text-console-text-main tracking-wide line-clamp-1">全域门店与网点管理</h2>
          <p className="text-sm text-console-text-muted mt-1">管理各区域下线下门店及线上微店的配置与同步</p>
        </div>
      </div>
      <button
        type="button"
        disabled
        className="text-console-text-muted px-4 py-2 rounded-lg text-sm font-medium border border-console-border opacity-60 cursor-not-allowed flex items-center gap-2"
      >
        <Plus size={16} />
        部署新门店
      </button>
    </div>
    <ConsoleContractEmptyState title="门店管理合约暂未开放" />
  </div>
);

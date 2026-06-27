import React from 'react';
import { Package, Plus } from 'lucide-react';
import { ConsoleContractEmptyState } from '@sdkwork/im-pc-commons';

export const ConsoleProducts = () => (
  <div className="bg-console-bg-panel border border-console-border rounded-2xl shadow-sm flex flex-col flex-1 min-h-0 h-full overflow-hidden">
    <div className="flex items-center justify-between p-6 border-b border-console-border shrink-0 bg-console-bg-root/50">
      <div className="flex items-center gap-3">
        <Package size={22} className="text-blue-600" />
        <div>
          <h2 className="text-xl font-bold text-console-text-main tracking-wide line-clamp-1">商品与微店中台</h2>
          <p className="text-sm text-console-text-muted mt-1">管理各门店下架商品、库存预警以及 SKU 同步配置</p>
        </div>
      </div>
      <button
        type="button"
        disabled
        className="text-console-text-muted px-4 py-2 rounded-lg text-sm font-medium border border-console-border opacity-60 cursor-not-allowed flex items-center gap-2"
      >
        <Plus size={16} />
        添加新商品
      </button>
    </div>
    <ConsoleContractEmptyState title="商品管理合约暂未开放" />
  </div>
);

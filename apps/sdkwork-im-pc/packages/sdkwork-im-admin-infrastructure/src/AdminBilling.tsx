import React, { useState, useEffect } from 'react';
import { CreditCard, DollarSign, TrendingUp, PieChart, Download, FileText, ArrowUpRight, ArrowDownRight, Package } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';
import { adminBillingService, AdminBillingData } from './services/AdminBillingService';

export const AdminBilling = () => {
  const [data, setData] = useState<AdminBillingData | null>(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const fetchData = async () => {
      setLoading(true);
      try {
        const res = await adminBillingService.getBillingData();
        setData(res);
      } finally {
        setLoading(false);
      }
    };
    fetchData();
  }, []);

  if (loading && !data) {
    return <div className="p-8 text-center text-admin-text-muted">加载账单数据中...</div>;
  }

  if (!data) return null;
  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <h2 className="text-xl font-bold text-admin-text-main tracking-wide">Billing & Revenue</h2>
          <p className="text-sm text-admin-text-muted mt-1">Platform-wide subscription metrics, MRR, and transaction logs</p>
        </div>
        <div className="flex gap-2">
          <button className="bg-admin-bg-hover hover:bg-admin-border-subtle border border-admin-border text-admin-text-main px-4 py-2 rounded-lg text-sm font-medium transition-colors flex items-center gap-2">
            <Download size={16} />
            Export CSV
          </button>
          <button className="bg-indigo-600 hover:bg-indigo-500 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors shadow-[0_0_15px_rgba(79,70,229,0.3)]">
            Configure Plans
          </button>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <BillingStat title={data.stats.mrr.title} value={data.stats.mrr.value} trend={data.stats.mrr.trend} isUp={data.stats.mrr.isUp} icon={DollarSign} color="emerald" />
        <BillingStat title={data.stats.active.title} value={data.stats.active.value} trend={data.stats.active.trend} isUp={data.stats.active.isUp} icon={CreditCard} color="indigo" />
        <BillingStat title={data.stats.net.title} value={data.stats.net.value} trend={data.stats.net.trend} isUp={data.stats.net.isUp} icon={TrendingUp} color="blue" />
        <BillingStat title={data.stats.churn.title} value={data.stats.churn.value} trend={data.stats.churn.trend} isUp={data.stats.churn.isUp} icon={PieChart} color="amber" />
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Plan Distribution */}
        <div className="lg:col-span-1 bg-admin-bg-panel border border-admin-border rounded-2xl shadow-xl p-6 flex flex-col relative overflow-hidden">
          <div className="absolute top-0 right-0 w-48 h-48 bg-indigo-500/5 blur-[60px] rounded-full pointer-events-none" />
          <h3 className="text-base font-semibold text-admin-text-main mb-6 relative z-10">Plan Distribution</h3>
          
          <div className="flex-1 flex flex-col gap-4 relative z-10">
            {data.plans.map((p, i) => (
              <PlanBar key={i} name={p.name} percent={p.percent} users={p.users} color={['bg-indigo-500', 'bg-blue-500', 'bg-emerald-500', 'bg-gray-500'][i % 4]} />
            ))}
          </div>
        </div>

        {/* Recent Transactions */}
        <div className="lg:col-span-2 bg-admin-bg-panel border border-admin-border rounded-2xl shadow-xl overflow-hidden flex flex-col">
          <div className="p-6 border-b border-admin-border flex justify-between items-center bg-admin-bg-root/30">
            <h3 className="text-base font-semibold text-admin-text-main">Recent Transactions</h3>
            <button className="text-xs text-indigo-400 font-medium hover:text-indigo-300">View All</button>
          </div>
          <div className="flex-1 overflow-auto custom-scrollbar">
            <table className="w-full text-left border-collapse">
              <thead>
                <tr className="text-[11px] uppercase tracking-widest text-admin-text-muted border-b border-admin-border bg-admin-bg-root/50">
                  <th className="px-6 py-4 font-semibold">Tenant</th>
                  <th className="px-6 py-4 font-semibold">Plan</th>
                  <th className="px-6 py-4 font-semibold">Amount</th>
                  <th className="px-6 py-4 font-semibold">Status</th>
                  <th className="px-6 py-4 font-semibold">Date</th>
                  <th className="px-6 py-4 font-semibold">Action</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-admin-border text-sm">
                {data.transactions.map((t) => (
                  <TransactionRow key={t.id} tenant={t.tenant} tenantId={t.tenantId} plan={t.plan} amount={t.amount} status={t.status} date={t.date} />
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  );
};

const BillingStat = ({ title, value, trend, isUp, icon: Icon, color }: any) => {
  const colorMap: Record<string, string> = {
    emerald: "text-emerald-400 bg-emerald-500/10 border-emerald-500/20",
    indigo: "text-indigo-400 bg-indigo-500/10 border-indigo-500/20",
    blue: "text-blue-400 bg-blue-500/10 border-blue-500/20",
    amber: "text-amber-400 bg-amber-500/10 border-amber-500/20",
  };

  return (
    <div className="bg-admin-bg-panel p-5 rounded-2xl border border-admin-border shadow-lg flex flex-col relative overflow-hidden group">
      <div className="flex justify-between items-start mb-4 relative z-10">
        <div className={cn("p-2.5 rounded-xl border", colorMap[color])}>
          <Icon size={20} />
        </div>
        <div className={cn(
          "flex items-center gap-1 px-2 py-1 rounded-md text-[10px] font-mono tracking-wider border",
          isUp ? "bg-emerald-500/10 text-emerald-400 border-emerald-500/20" : "bg-rose-500/10 text-rose-400 border-rose-500/20"
        )}>
          {isUp ? <ArrowUpRight size={12} /> : <ArrowDownRight size={12} />}
          {trend}
        </div>
      </div>
      <div className="flex flex-col relative z-10">
        <span className="text-[28px] font-bold text-admin-text-main leading-none mb-1 tracking-tight">{value}</span>
        <span className="text-xs text-admin-text-muted font-medium tracking-wide mt-1">{title}</span>
      </div>
    </div>
  );
};

const PlanBar = ({ name, percent, users, color }: any) => (
  <div>
    <div className="flex justify-between text-sm mb-1.5">
      <span className="text-admin-text-main font-medium">{name}</span>
      <span className="text-admin-text-muted">{percent}% <span className="text-[10px] ml-1">({users} tenants)</span></span>
    </div>
    <div className="w-full h-2 bg-admin-bg-root rounded-full overflow-hidden border border-admin-border-subtle">
      <div className={cn("h-full rounded-full", color)} style={{ width: `${percent}%` }}></div>
    </div>
  </div>
);

const TransactionRow = ({ tenant, tenantId, plan, amount, status, date }: any) => {
  const statusColors = {
    paid: "bg-emerald-500/10 text-emerald-400 border-emerald-500/20",
    failed: "bg-rose-500/10 text-rose-400 border-rose-500/20",
    pending: "bg-amber-500/10 text-amber-400 border-amber-500/20"
  };

  return (
    <tr className="hover:bg-admin-bg-hover transition-colors group">
      <td className="px-6 py-4">
        <div className="font-semibold text-admin-text-main">{tenant}</div>
        <div className="text-[10px] font-mono text-admin-text-muted mt-0.5">{tenantId}</div>
      </td>
      <td className="px-6 py-4 flex items-center gap-1.5 text-admin-text-main">
        <Package size={14} className="text-admin-text-muted" /> {plan}
      </td>
      <td className="px-6 py-4 font-mono text-admin-text-main">{amount}</td>
      <td className="px-6 py-4">
        <span className={cn("px-2.5 py-1 rounded-md text-[10px] font-mono uppercase tracking-wider border", statusColors[status as keyof typeof statusColors])}>
          {status}
        </span>
      </td>
      <td className="px-6 py-4 text-xs text-admin-text-muted">{date}</td>
      <td className="px-6 py-4">
        <button className="p-1.5 text-admin-text-muted hover:text-indigo-400 hover:bg-admin-bg-root rounded-md transition-colors">
          <FileText size={16} />
        </button>
      </td>
    </tr>
  );
};

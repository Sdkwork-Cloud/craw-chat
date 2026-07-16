import { useEffect, useState } from 'react';
import {
  AlertTriangle,
  ArrowDownRight,
  ArrowUpRight,
  CreditCard,
  DollarSign,
  Package,
  PieChart,
  TrendingUp,
  type LucideIcon,
} from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';
import {
  adminBillingService,
  type AdminBillingData,
  type BillingStatItem,
  type PlanDistribution,
  type TransactionInfo,
} from './services/AdminBillingService';

function getErrorMessage(error: unknown): string {
  if (error instanceof Error && error.message.trim()) {
    return error.message;
  }

  return 'Billing data could not be loaded. Retry when the backend service is available.';
}

export const AdminBilling = () => {
  const [data, setData] = useState<AdminBillingData | null>(null);
  const [loading, setLoading] = useState(true);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [requestVersion, setRequestVersion] = useState(0);

  useEffect(() => {
    let cancelled = false;

    const fetchData = async () => {
      setLoading(true);
      setErrorMessage(null);

      try {
        const nextData = await adminBillingService.getBillingData();

        if (!cancelled) {
          setData(nextData);
        }
      } catch (error) {
        if (!cancelled) {
          setErrorMessage(getErrorMessage(error));
        }
      } finally {
        if (!cancelled) {
          setLoading(false);
        }
      }
    };

    void fetchData();

    return () => {
      cancelled = true;
    };
  }, [requestVersion]);

  const retry = () => {
    setRequestVersion((currentVersion) => currentVersion + 1);
  };

  if (loading && !data) {
    return <div className="p-8 text-center text-admin-text-muted">Loading billing data...</div>;
  }

  if (!data) {
    return <BillingError errorMessage={errorMessage} onRetry={retry} />;
  }

  return (
    <div className="space-y-6">
      <header>
        <h1 className="text-xl font-bold tracking-wide text-admin-text-main">Billing and revenue</h1>
        <p className="mt-1 text-sm text-admin-text-muted">
          Platform subscription metrics and one server-paginated billing-event page.
        </p>
      </header>

      {errorMessage && (
        <div
          className="flex flex-col gap-3 border border-amber-500/30 bg-amber-500/10 p-4 text-sm text-admin-text-main sm:flex-row sm:items-center sm:justify-between"
          role="alert"
        >
          <span>{errorMessage}</span>
          <button
            className="border border-admin-border bg-admin-bg-panel px-3 py-1.5 text-sm font-medium text-admin-text-main transition-colors hover:bg-admin-bg-hover"
            onClick={retry}
            type="button"
          >
            Retry
          </button>
        </div>
      )}

      <div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-4">
        <BillingStat color="emerald" icon={DollarSign} item={data.stats.mrr} />
        <BillingStat color="indigo" icon={CreditCard} item={data.stats.active} />
        <BillingStat color="blue" icon={TrendingUp} item={data.stats.net} />
        <BillingStat color="amber" icon={PieChart} item={data.stats.churn} />
      </div>

      <div className="grid grid-cols-1 gap-6 lg:grid-cols-3">
        <section className="flex flex-col border border-admin-border bg-admin-bg-panel p-6 lg:col-span-1">
          <h2 className="mb-6 text-base font-semibold text-admin-text-main">Plan distribution</h2>

          {data.plans.length > 0 ? (
            <div className="flex flex-1 flex-col gap-4">
              {data.plans.map((plan, index) => (
                <PlanBar key={`${plan.name}-${index}`} plan={plan} />
              ))}
            </div>
          ) : (
            <EmptyDataState message="No plan distribution records are available." />
          )}
        </section>

        <section className="flex flex-col overflow-hidden border border-admin-border bg-admin-bg-panel lg:col-span-2">
          <div className="border-b border-admin-border bg-admin-bg-root/30 p-6">
            <h2 className="text-base font-semibold text-admin-text-main">Billing events</h2>
          </div>
          <div className="flex-1 overflow-auto custom-scrollbar">
            <table className="w-full border-collapse text-left text-sm">
              <thead>
                <tr className="border-b border-admin-border bg-admin-bg-root/50 text-[11px] uppercase tracking-widest text-admin-text-muted">
                  <th className="px-6 py-4 font-semibold">Tenant</th>
                  <th className="px-6 py-4 font-semibold">Plan</th>
                  <th className="px-6 py-4 font-semibold">Amount</th>
                  <th className="px-6 py-4 font-semibold">Status</th>
                  <th className="px-6 py-4 font-semibold">Date</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-admin-border">
                {data.transactions.length > 0 ? data.transactions.map((transaction) => (
                  <TransactionRow key={transaction.id} transaction={transaction} />
                )) : (
                  <tr>
                    <td className="px-6 py-10 text-center text-admin-text-muted" colSpan={5}>
                      No billing event records are available.
                    </td>
                  </tr>
                )}
              </tbody>
            </table>
          </div>
        </section>
      </div>
    </div>
  );
};

function BillingError({
  errorMessage,
  onRetry,
}: {
  errorMessage: string | null;
  onRetry: () => void;
}) {
  return (
    <section className="flex min-h-[320px] flex-col items-center justify-center gap-4 border border-admin-border bg-admin-bg-panel p-8 text-center" role="alert">
      <AlertTriangle className="text-admin-text-muted" size={32} />
      <h1 className="text-lg font-semibold text-admin-text-main">Billing data unavailable</h1>
      <p className="max-w-lg text-sm text-admin-text-muted">
        {errorMessage ?? 'Billing data could not be loaded.'}
      </p>
      <button
        className="border border-admin-border bg-admin-bg-root px-4 py-2 text-sm font-medium text-admin-text-main transition-colors hover:bg-admin-bg-hover"
        onClick={onRetry}
        type="button"
      >
        Retry
      </button>
    </section>
  );
}

function EmptyDataState({ message }: { message: string }) {
  return (
    <div className="flex min-h-[160px] flex-1 items-center justify-center text-center text-sm text-admin-text-muted">
      {message}
    </div>
  );
}

function BillingStat({
  color,
  icon: Icon,
  item,
}: {
  color: 'amber' | 'blue' | 'emerald' | 'indigo';
  icon: LucideIcon;
  item: BillingStatItem;
}) {
  const colorMap: Record<'amber' | 'blue' | 'emerald' | 'indigo', string> = {
    amber: 'border-amber-500/20 bg-amber-500/10 text-amber-400',
    blue: 'border-blue-500/20 bg-blue-500/10 text-blue-400',
    emerald: 'border-emerald-500/20 bg-emerald-500/10 text-emerald-400',
    indigo: 'border-indigo-500/20 bg-indigo-500/10 text-indigo-400',
  };

  return (
    <div className="flex flex-col border border-admin-border bg-admin-bg-panel p-5">
      <div className="mb-4 flex items-start justify-between">
        <div className={cn('rounded-xl border p-2.5', colorMap[color])}>
          <Icon size={20} />
        </div>
        {item.available && item.trend ? (
          <div className={cn(
            'flex items-center gap-1 border px-2 py-1 font-mono text-[10px] tracking-wider',
            item.isUp
              ? 'border-emerald-500/20 bg-emerald-500/10 text-emerald-400'
              : 'border-rose-500/20 bg-rose-500/10 text-rose-400',
          )}>
            {item.isUp ? <ArrowUpRight size={12} /> : <ArrowDownRight size={12} />}
            {item.trend}
          </div>
        ) : (
          <span className="text-[10px] text-admin-text-muted">
            {item.available ? 'Trend unavailable' : 'Metric unavailable'}
          </span>
        )}
      </div>
      <span className="mb-1 text-[28px] font-bold leading-none tracking-tight text-admin-text-main">{item.value}</span>
      <span className="mt-1 text-xs font-medium tracking-wide text-admin-text-muted">{item.title}</span>
    </div>
  );
}

function PlanBar({ plan }: { plan: PlanDistribution }) {
  const percentage = plan.percent === null ? null : Math.max(0, Math.min(100, plan.percent));

  return (
    <div>
      <div className="mb-1.5 flex justify-between text-sm">
        <span className="font-medium text-admin-text-main">{plan.name}</span>
        <span className="text-admin-text-muted">
          {percentage === null ? 'Unavailable' : `${percentage}%`}
          <span className="ml-1 text-[10px]">
            ({plan.users === null ? 'Unavailable' : plan.users} tenants)
          </span>
        </span>
      </div>
      <div className="h-2 w-full overflow-hidden border border-admin-border-subtle bg-admin-bg-root">
        {percentage !== null && <div className="h-full bg-indigo-500" style={{ width: `${percentage}%` }} />}
      </div>
    </div>
  );
}

function TransactionRow({ transaction }: { transaction: TransactionInfo }) {
  const statusColors: Record<TransactionInfo['status'], string> = {
    failed: 'border-rose-500/20 bg-rose-500/10 text-rose-400',
    paid: 'border-emerald-500/20 bg-emerald-500/10 text-emerald-400',
    pending: 'border-amber-500/20 bg-amber-500/10 text-amber-400',
    unknown: 'border-gray-500/20 bg-gray-500/10 text-admin-text-muted',
  };

  return (
    <tr className="transition-colors hover:bg-admin-bg-hover">
      <td className="px-6 py-4">
        <div className="font-semibold text-admin-text-main">{transaction.tenant}</div>
        <div className="mt-0.5 font-mono text-[10px] text-admin-text-muted">{transaction.tenantId}</div>
      </td>
      <td className="flex items-center gap-1.5 px-6 py-4 text-admin-text-main">
        <Package className="text-admin-text-muted" size={14} />
        {transaction.plan}
      </td>
      <td className="px-6 py-4 font-mono text-admin-text-main">{transaction.amount}</td>
      <td className="px-6 py-4">
        <span className={cn('border px-2.5 py-1 font-mono text-[10px] uppercase tracking-wider', statusColors[transaction.status])}>
          {transaction.status}
        </span>
      </td>
      <td className="px-6 py-4 text-xs text-admin-text-muted">{transaction.date}</td>
    </tr>
  );
}

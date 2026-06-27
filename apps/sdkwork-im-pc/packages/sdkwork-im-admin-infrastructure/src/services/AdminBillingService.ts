import { getBackendSdkClientWithSession } from '@sdkwork/im-admin-core/sdk';

export interface BillingStatItem {
  title: string;
  value: string;
  trend: string;
  isUp: boolean;
}

export interface PlanDistribution {
  name: string;
  percent: number;
  users: number;
}

export interface TransactionInfo {
  id: string;
  tenant: string;
  tenantId: string;
  plan: string;
  amount: string;
  status: 'paid' | 'failed' | 'pending';
  date: string;
}

export interface AdminBillingData {
  stats: Record<string, BillingStatItem>;
  plans: PlanDistribution[];
  transactions: TransactionInfo[];
}

type UnknownRecord = Record<string, unknown>;

function asRecord(value: unknown): UnknownRecord {
  return value && typeof value === 'object' && !Array.isArray(value) ? value as UnknownRecord : {};
}

function asRecordArray(value: unknown): UnknownRecord[] {
  return Array.isArray(value) ? value.map(asRecord).filter((item) => Object.keys(item).length > 0) : [];
}

function readRecord(record: UnknownRecord, keys: string[]): UnknownRecord {
  for (const key of keys) {
    const value = asRecord(record[key]);
    if (Object.keys(value).length > 0) {
      return value;
    }
  }
  return {};
}

function readRecords(record: UnknownRecord, keys: string[]): UnknownRecord[] {
  for (const key of keys) {
    const values = asRecordArray(record[key]);
    if (values.length > 0) {
      return values;
    }
  }
  return [];
}

function readString(record: UnknownRecord, keys: string[], fallback = ''): string {
  for (const key of keys) {
    const value = record[key];
    if (typeof value === 'string' && value.trim()) {
      return value.trim();
    }
    if (typeof value === 'number' && Number.isFinite(value)) {
      return String(value);
    }
  }
  return fallback;
}

function readNumber(record: UnknownRecord, keys: string[], fallback = 0): number {
  for (const key of keys) {
    const value = record[key];
    if (typeof value === 'number' && Number.isFinite(value)) {
      return value;
    }
    if (typeof value === 'string' && value.trim()) {
      const parsed = Number(value.replace(/[$,%\s,]/gu, ''));
      if (Number.isFinite(parsed)) {
        return parsed;
      }
    }
  }
  return fallback;
}

function formatCurrency(value: number, fallback = '$0'): string {
  if (!Number.isFinite(value) || value <= 0) {
    return fallback;
  }
  return new Intl.NumberFormat('en-US', {
    currency: 'USD',
    maximumFractionDigits: 0,
    style: 'currency',
  }).format(value);
}

function formatCount(value: number): string {
  if (value >= 1_000_000) {
    return `${(value / 1_000_000).toFixed(1)}M`;
  }
  if (value >= 1_000) {
    return `${(value / 1_000).toFixed(value >= 10_000 ? 0 : 1)}K`;
  }
  return String(Math.max(0, Math.round(value)));
}

function formatPercent(value: number, fallback = '0%'): string {
  if (!Number.isFinite(value)) {
    return fallback;
  }
  return `${value.toFixed(Math.abs(value) < 10 ? 1 : 0)}%`;
}

function formatTrend(value: unknown): string {
  if (typeof value === 'string' && value.trim()) {
    return value.trim();
  }
  if (typeof value === 'number' && Number.isFinite(value)) {
    const sign = value > 0 ? '+' : '';
    return `${sign}${value}`;
  }
  return '';
}

function resolveTrend(record: UnknownRecord, keys: string[]): string {
  for (const key of keys) {
    const trend = formatTrend(record[key]);
    if (trend) {
      return trend;
    }
  }
  return '';
}

function isPositiveTrend(trend: string): boolean {
  if (!trend) {
    return true;
  }
  return !trend.trim().startsWith('-');
}

function normalizeStatus(value: unknown): TransactionInfo['status'] {
  const status = String(value ?? '').trim().toLowerCase();
  if (status === 'failed' || status === 'fail' || status === 'error' || status === 'declined') {
    return 'failed';
  }
  if (status === 'pending' || status === 'processing' || status === 'open') {
    return 'pending';
  }
  return 'paid';
}

function buildStats(summary: UnknownRecord, eventsSummary: UnknownRecord): Record<string, BillingStatItem> {
  const subscriptionSummary = readRecord(summary, ['subscriptions', 'subscriptionSummary']);
  const mrr = readNumber(summary, ['mrr', 'monthlyRecurringRevenue', 'monthlyRevenue'], 0);
  const active = readNumber(subscriptionSummary, ['active', 'activeSubscriptions', 'count'], readNumber(summary, ['activeSubscriptions', 'subscriptions'], 0));
  const retention = readNumber(summary, ['netRevenueRetention', 'retentionRate', 'nrr'], Number.NaN);
  const churn = readNumber(summary, ['churnRate', 'mrrChurnRate', 'churn'], Number.NaN);
  const mrrTrend = resolveTrend(summary, ['mrrTrend', 'monthlyRecurringRevenueTrend', 'revenueTrend']);
  const activeTrend = resolveTrend(subscriptionSummary, ['activeTrend', 'trend']);
  const retentionTrend = resolveTrend(summary, ['retentionTrend', 'netRevenueRetentionTrend', 'nrrTrend']);
  const churnTrend = resolveTrend(summary, ['churnTrend', 'mrrChurnTrend']);
  const fallbackRevenue = readNumber(eventsSummary, ['paidAmount', 'totalPaidAmount', 'totalAmount'], 0);

  return {
    active: {
      isUp: isPositiveTrend(activeTrend),
      title: 'Active Subscriptions',
      trend: activeTrend,
      value: formatCount(active),
    },
    churn: {
      isUp: churnTrend ? !isPositiveTrend(churnTrend) : churn <= 2,
      title: 'Churn Rate (MRR)',
      trend: churnTrend,
      value: Number.isFinite(churn) ? formatPercent(churn) : '0%',
    },
    mrr: {
      isUp: isPositiveTrend(mrrTrend),
      title: 'Monthly Recurring Revenue',
      trend: mrrTrend,
      value: formatCurrency(mrr || fallbackRevenue),
    },
    net: {
      isUp: isPositiveTrend(retentionTrend),
      title: 'Net Revenue Retention',
      trend: retentionTrend,
      value: Number.isFinite(retention) ? formatPercent(retention) : '0%',
    },
  };
}

function buildPlans(summary: UnknownRecord): PlanDistribution[] {
  const planRecords = readRecords(summary, ['plans', 'planDistribution', 'subscriptionPlans']);
  const totalUsers = planRecords.reduce((total, plan) => total + readNumber(plan, ['users', 'tenants', 'count', 'subscriptions'], 0), 0);
  return planRecords.map((plan) => {
    const users = readNumber(plan, ['users', 'tenants', 'count', 'subscriptions'], 0);
    const percent = readNumber(plan, ['percent', 'percentage', 'share'], totalUsers > 0 ? (users / totalUsers) * 100 : 0);
    return {
      name: readString(plan, ['name', 'plan', 'planName', 'tier'], 'Unassigned'),
      percent: Math.max(0, Math.min(100, Math.round(percent))),
      users: Math.max(0, Math.round(users)),
    };
  });
}

function buildTransactions(events: UnknownRecord): TransactionInfo[] {
  return readRecords(events, ['items', 'data', 'events', 'records', 'transactions']).map((event, index) => {
    const amount = readNumber(event, ['amount', 'paidAmount', 'total', 'value'], 0);
    return {
      amount: amount > 0 ? formatCurrency(amount) : readString(event, ['amountText', 'formattedAmount'], '$0'),
      date: readString(event, ['createdAt', 'paidAt', 'eventTime', 'date', 'time'], ''),
      id: readString(event, ['id', 'eventId', 'transactionId', 'recordId'], `billing-event-${index + 1}`),
      plan: readString(event, ['plan', 'planName', 'tier'], 'Unassigned'),
      status: normalizeStatus(readString(event, ['status', 'paymentStatus', 'state'], 'paid')),
      tenant: readString(event, ['tenantName', 'tenant', 'organizationName', 'accountName'], 'Unknown tenant'),
      tenantId: readString(event, ['tenantId', 'organizationId', 'accountId'], ''),
    };
  });
}

class AdminBillingService {
  async getBillingData(): Promise<AdminBillingData> {
    const backend = getBackendSdkClientWithSession();
    const [summary, eventsSummary, events] = await Promise.all([
      backend.admin.billing.summary.retrieve(),
      backend.admin.billing.events.summary.retrieve(),
      backend.admin.billing.events.list(),
    ]);
    const normalizedSummary = asRecord(summary);
    const normalizedEventsSummary = asRecord(eventsSummary);

    return {
      plans: buildPlans(normalizedSummary),
      stats: buildStats(normalizedSummary, normalizedEventsSummary),
      transactions: buildTransactions(asRecord(events)),
    };
  }
}

export const adminBillingService = new AdminBillingService();

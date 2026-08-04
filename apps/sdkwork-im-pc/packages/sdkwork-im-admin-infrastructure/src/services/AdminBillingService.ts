import { formatMoney } from '@sdkwork/utils/money';
import { getBackendSdkClientWithSession } from '@sdkwork/im-pc-admin-sdk';

export interface BillingStatItem {
  available: boolean;
  title: string;
  value: string;
  trend: string;
  isUp: boolean;
}

export interface PlanDistribution {
  name: string;
  percent: number | null;
  users: number | null;
}

export interface TransactionInfo {
  id: string;
  tenant: string;
  tenantId: string;
  plan: string;
  amount: string;
  status: 'paid' | 'failed' | 'pending' | 'unknown';
  date: string;
}

export interface AdminBillingData {
  stats: Record<string, BillingStatItem>;
  plans: PlanDistribution[];
  transactions: TransactionInfo[];
}

type UnknownRecord = Record<string, unknown>;

/**
 * Billing events are an interactive list. Keep the initial dashboard request
 * within the SDKWork default page size instead of materializing the complete
 * event history in the renderer process.
 */
export const BILLING_EVENTS_PAGE_SIZE = 20;

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

function formatCurrency(value: number): string {
  if (!Number.isFinite(value)) {
    return '—';
  }
  return (
    formatMoney(value, {
      currency: 'USD',
      locale: 'en-US',
      mode: 'symbol',
      minFractionDigits: 0,
      maxFractionDigits: 0,
    }) ?? '—'
  );
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

function formatPercent(value: number, fallback = '—'): string {
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
  if (status === 'paid' || status === 'completed' || status === 'settled' || status === 'success') {
    return 'paid';
  }
  return 'unknown';
}

function buildStats(summary: UnknownRecord, eventsSummary: UnknownRecord): Record<string, BillingStatItem> {
  const subscriptionSummary = readRecord(summary, ['subscriptions', 'subscriptionSummary']);
  const mrr = readNumber(summary, ['mrr', 'monthlyRecurringRevenue', 'monthlyRevenue'], Number.NaN);
  const active = readNumber(
    subscriptionSummary,
    ['active', 'activeSubscriptions', 'count'],
    readNumber(summary, ['activeSubscriptions', 'subscriptions'], Number.NaN),
  );
  const retention = readNumber(summary, ['netRevenueRetention', 'retentionRate', 'nrr'], Number.NaN);
  const churn = readNumber(summary, ['churnRate', 'mrrChurnRate', 'churn'], Number.NaN);
  const mrrTrend = resolveTrend(summary, ['mrrTrend', 'monthlyRecurringRevenueTrend', 'revenueTrend']);
  const activeTrend = resolveTrend(subscriptionSummary, ['activeTrend', 'trend']);
  const retentionTrend = resolveTrend(summary, ['retentionTrend', 'netRevenueRetentionTrend', 'nrrTrend']);
  const churnTrend = resolveTrend(summary, ['churnTrend', 'mrrChurnTrend']);
  const fallbackRevenue = readNumber(eventsSummary, ['paidAmount', 'totalPaidAmount', 'totalAmount'], Number.NaN);
  const mrrValue = Number.isFinite(mrr) ? mrr : fallbackRevenue;
  const mrrAvailable = Number.isFinite(mrrValue);
  const activeAvailable = Number.isFinite(active);
  const retentionAvailable = Number.isFinite(retention);
  const churnAvailable = Number.isFinite(churn);

  return {
    active: {
      available: activeAvailable,
      isUp: isPositiveTrend(activeTrend),
      title: 'Active Subscriptions',
      trend: activeTrend,
      value: activeAvailable ? formatCount(active) : '—',
    },
    churn: {
      available: churnAvailable,
      isUp: churnTrend ? !isPositiveTrend(churnTrend) : churn <= 2,
      title: 'Churn Rate (MRR)',
      trend: churnTrend,
      value: churnAvailable ? formatPercent(churn) : '—',
    },
    mrr: {
      available: mrrAvailable,
      isUp: isPositiveTrend(mrrTrend),
      title: 'Monthly Recurring Revenue',
      trend: mrrTrend,
      value: mrrAvailable ? formatCurrency(mrrValue) : '—',
    },
    net: {
      available: retentionAvailable,
      isUp: isPositiveTrend(retentionTrend),
      title: 'Net Revenue Retention',
      trend: retentionTrend,
      value: retentionAvailable ? formatPercent(retention) : '—',
    },
  };
}

function buildPlans(summary: UnknownRecord): PlanDistribution[] {
  const planRecords = readRecords(summary, ['plans', 'planDistribution', 'subscriptionPlans']);
  const userCounts = planRecords.map((plan) => readNumber(
    plan,
    ['users', 'tenants', 'count', 'subscriptions'],
    Number.NaN,
  ));
  const totalUsers = userCounts.reduce(
    (total, users) => Number.isFinite(users) ? total + users : total,
    0,
  );

  return planRecords.map((plan, index) => {
    const users = userCounts[index] ?? Number.NaN;
    const explicitPercent = readNumber(plan, ['percent', 'percentage', 'share'], Number.NaN);
    let percent = Number.NaN;
    if (Number.isFinite(explicitPercent)) {
      percent = explicitPercent;
    } else if (totalUsers > 0 && Number.isFinite(users)) {
      percent = (users / totalUsers) * 100;
    }

    return {
      name: readString(plan, ['name', 'plan', 'planName', 'tier'], 'Unassigned'),
      percent: Number.isFinite(percent) ? Math.max(0, Math.min(100, Math.round(percent))) : null,
      users: Number.isFinite(users) ? Math.max(0, Math.round(users)) : null,
    };
  });
}

function buildTransactions(events: UnknownRecord): TransactionInfo[] {
  return readRecords(events, ['items', 'data', 'events', 'records', 'transactions']).map((event, index) => {
    const amount = readNumber(event, ['amount', 'paidAmount', 'total', 'value'], Number.NaN);
    return {
      amount: Number.isFinite(amount)
        ? formatCurrency(amount)
        : readString(event, ['amountText', 'formattedAmount'], 'Unavailable'),
      date: readString(event, ['createdAt', 'paidAt', 'eventTime', 'date', 'time'], 'Unavailable'),
      id: readString(event, ['id', 'eventId', 'transactionId', 'recordId'], `billing-event-${index + 1}`),
      plan: readString(event, ['plan', 'planName', 'tier'], 'Unavailable'),
      status: normalizeStatus(readString(event, ['status', 'paymentStatus', 'state'])),
      tenant: readString(event, ['tenantName', 'tenant', 'organizationName', 'accountName'], 'Unavailable'),
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
      backend.admin.billing.events.list({ pageSize: BILLING_EVENTS_PAGE_SIZE }),
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

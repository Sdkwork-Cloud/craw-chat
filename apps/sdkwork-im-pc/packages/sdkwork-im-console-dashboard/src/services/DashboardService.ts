import { getAppSdkClientWithSession } from '@sdkwork/im-pc-core';

export interface DashboardMetrics {
  totalUsers: { value: string; trend: string; isUp: boolean };
  dailyMessages: { value: string; trend: string; isUp: boolean };
  activeGroups: { value: string; trend: string; isUp: boolean };
  storageUsage: { value: string; trend: string; isUp: boolean };
}

export interface ActivityTrend {
  day: string;
  value: number;
}

export interface SecurityAlert {
  id: string;
  type: 'high' | 'medium' | 'low' | 'info';
  message: string;
  time: string;
}

interface ConsoleDashboardSnapshot {
  dashboard: Record<string, unknown>;
  conversations: Record<string, unknown>;
}

type UnknownRecord = Record<string, unknown>;

const FALLBACK_DAYS = ['一', '二', '三', '四', '五', '六', '日'];

let cachedSnapshot: Promise<ConsoleDashboardSnapshot> | null = null;

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

function readNumber(record: UnknownRecord, keys: string[], fallback = 0): number {
  for (const key of keys) {
    const value = record[key];
    if (typeof value === 'number' && Number.isFinite(value)) {
      return value;
    }
    if (typeof value === 'string' && value.trim()) {
      const parsed = Number(value.replace(/[,%\s]/gu, ''));
      if (Number.isFinite(parsed)) {
        return parsed;
      }
    }
  }
  return fallback;
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

function readTrend(record: UnknownRecord, keys: string[]): string {
  for (const key of keys) {
    const value = record[key];
    if (typeof value === 'string' && value.trim()) {
      return value.trim();
    }
    if (typeof value === 'number' && Number.isFinite(value)) {
      const sign = value > 0 ? '+' : '';
      return `${sign}${value}%`;
    }
  }
  return '';
}

function isUp(trend: string, fallback = true): boolean {
  return trend ? !trend.trim().startsWith('-') : fallback;
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

function formatStorage(value: number, unit: string): string {
  if (!Number.isFinite(value) || value <= 0) {
    return '0 GB';
  }
  return `${value.toFixed(value >= 10 ? 0 : 1)} ${unit || 'GB'}`;
}

function normalizeAlertType(value: unknown): SecurityAlert['type'] {
  const type = String(value ?? '').trim().toLowerCase();
  if (type === 'critical' || type === 'high') {
    return 'high';
  }
  if (type === 'warning' || type === 'medium') {
    return 'medium';
  }
  if (type === 'low') {
    return 'low';
  }
  return 'info';
}

function buildMetrics(snapshot: ConsoleDashboardSnapshot): DashboardMetrics {
  const metrics = readRecord(snapshot.dashboard, ['metrics', 'summary', 'stats']);
  const users = readRecord(metrics, ['users', 'totalUsers']);
  const messages = readRecord(metrics, ['messages', 'dailyMessages']);
  const groups = readRecord(metrics, ['groups', 'activeGroups']);
  const storage = readRecord(metrics, ['storage', 'storageUsage']);
  const userTrend = readTrend(users, ['trend', 'growth', 'change']);
  const messageTrend = readTrend(messages, ['trend', 'growth', 'change']);
  const groupTrend = readTrend(groups, ['trend', 'growth', 'change']);
  const storageTrend = readTrend(storage, ['trend', 'growth', 'change', 'health']);
  const fallbackMessageCount = readNumber(snapshot.conversations, ['dailyMessages', 'messageCount', 'messages'], 0);

  return {
    activeGroups: {
      isUp: isUp(groupTrend),
      trend: groupTrend,
      value: formatCount(readNumber(groups, ['value', 'count', 'active'], readNumber(snapshot.conversations, ['activeGroups', 'groupCount'], 0))),
    },
    dailyMessages: {
      isUp: isUp(messageTrend),
      trend: messageTrend,
      value: formatCount(readNumber(messages, ['value', 'count', 'daily'], fallbackMessageCount)),
    },
    storageUsage: {
      isUp: isUp(storageTrend),
      trend: storageTrend,
      value: readString(storage, ['displayValue', 'valueText'], formatStorage(readNumber(storage, ['value', 'used', 'usedGb'], 0), readString(storage, ['unit'], 'GB'))),
    },
    totalUsers: {
      isUp: isUp(userTrend),
      trend: userTrend,
      value: formatCount(readNumber(users, ['value', 'count', 'total'], readNumber(snapshot.dashboard, ['totalUsers', 'userCount'], 0))),
    },
  };
}

function buildActivityTrends(snapshot: ConsoleDashboardSnapshot): ActivityTrend[] {
  const trendRecords = readRecords(snapshot.conversations, ['activityTrends', 'trends', 'dailyActivity', 'samples'])
    .concat(readRecords(snapshot.dashboard, ['activityTrends', 'trends', 'dailyActivity', 'samples']));
  if (trendRecords.length === 0) {
    return FALLBACK_DAYS.map((day) => ({ day, value: 0 }));
  }
  const maxValue = Math.max(...trendRecords.map((item) => readNumber(item, ['value', 'count', 'messages'], 0)), 1);
  return trendRecords.slice(0, 14).map((item, index) => {
    const value = readNumber(item, ['percent', 'percentage'], (readNumber(item, ['value', 'count', 'messages'], 0) / maxValue) * 100);
    return {
      day: readString(item, ['day', 'label', 'name'], FALLBACK_DAYS[index % FALLBACK_DAYS.length]),
      value: Math.max(0, Math.min(100, Math.round(value))),
    };
  });
}

function buildSecurityAlerts(snapshot: ConsoleDashboardSnapshot): SecurityAlert[] {
  const alertRecords = readRecords(snapshot.dashboard, ['securityAlerts', 'alerts', 'securityEvents']);
  return alertRecords.slice(0, 8).map((alert, index) => ({
    id: readString(alert, ['id', 'alertId', 'eventId'], `alert-${index + 1}`),
    message: readString(alert, ['message', 'summary', 'title'], 'Security event'),
    time: readString(alert, ['time', 'createdAt', 'recordedAt'], ''),
    type: normalizeAlertType(readString(alert, ['type', 'severity', 'level'], 'info')),
  }));
}

async function loadSnapshot(): Promise<ConsoleDashboardSnapshot> {
  if (!cachedSnapshot) {
    cachedSnapshot = Promise.all([
      getAppSdkClientWithSession().portal.dashboard.retrieve(),
      getAppSdkClientWithSession().portal.conversationSnapshot.retrieve(),
    ]).then(([dashboard, conversations]) => ({
      conversations: asRecord(conversations),
      dashboard: asRecord(dashboard),
    })).catch((error: unknown) => {
      cachedSnapshot = null;
      throw error;
    });
  }
  return cachedSnapshot;
}

class DashboardService {
  async getMetrics(): Promise<DashboardMetrics> {
    return buildMetrics(await loadSnapshot());
  }

  async getActivityTrends(_period: string): Promise<ActivityTrend[]> {
    return buildActivityTrends(await loadSnapshot());
  }

  async getSecurityAlerts(): Promise<SecurityAlert[]> {
    return buildSecurityAlerts(await loadSnapshot());
  }
}

export const dashboardService = new DashboardService();

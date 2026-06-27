import { getAppSdkClientWithSession } from '@sdkwork/im-pc-core';

export interface SecurityIntercept {
  id: string;
  title: string;
  count: number;
  level: 'critical' | 'high' | 'warning' | 'info';
}

export interface SecurityAuditLog {
  id: string;
  time: string;
  user: string;
  action: string;
}

export interface SecurityDashboardData {
  healthScore: number;
  intercepts: SecurityIntercept[];
  auditLogs: SecurityAuditLog[];
}

type UnknownRecord = Record<string, unknown>;

function asRecord(value: unknown): UnknownRecord {
  return value && typeof value === 'object' && !Array.isArray(value) ? value as UnknownRecord : {};
}

function asRecordArray(value: unknown): UnknownRecord[] {
  return Array.isArray(value) ? value.map(asRecord).filter((item) => Object.keys(item).length > 0) : [];
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
      const parsed = Number(value.replace(/[,%\s]/gu, ''));
      if (Number.isFinite(parsed)) {
        return parsed;
      }
    }
  }
  return fallback;
}

function normalizeInterceptLevel(value: unknown): SecurityIntercept['level'] {
  const level = String(value ?? '').trim().toLowerCase();
  if (level === 'critical') {
    return 'critical';
  }
  if (level === 'high' || level === 'error' || level === 'failed') {
    return 'high';
  }
  if (level === 'warning' || level === 'medium') {
    return 'warning';
  }
  return 'info';
}

function deriveHealthScore(health: UnknownRecord, records: UnknownRecord[]): number {
  const explicitScore = readNumber(health, ['healthScore', 'securityScore', 'score'], Number.NaN);
  if (Number.isFinite(explicitScore)) {
    return Math.max(0, Math.min(100, Math.round(explicitScore)));
  }
  const criticalCount = records.filter((record) => normalizeInterceptLevel(readString(record, ['severity', 'level', 'status'], '')) === 'critical').length;
  const highCount = records.filter((record) => normalizeInterceptLevel(readString(record, ['severity', 'level', 'status'], '')) === 'high').length;
  const warningCount = records.filter((record) => normalizeInterceptLevel(readString(record, ['severity', 'level', 'status'], '')) === 'warning').length;
  return Math.max(0, Math.min(100, 100 - criticalCount * 15 - highCount * 8 - warningCount * 3));
}

function buildIntercepts(health: UnknownRecord, records: UnknownRecord[]): SecurityIntercept[] {
  const interceptRecords = readRecords(health, ['intercepts', 'securityIntercepts', 'alerts']);
  if (interceptRecords.length > 0) {
    return interceptRecords.slice(0, 8).map((item, index) => ({
      count: readNumber(item, ['count', 'total', 'value'], 0),
      id: readString(item, ['id', 'key', 'type'], `intercept-${index + 1}`),
      level: normalizeInterceptLevel(readString(item, ['level', 'severity'], 'info')),
      title: readString(item, ['title', 'name', 'message'], 'Security signal'),
    }));
  }

  const buckets = new Map<SecurityIntercept['level'], number>([
    ['critical', 0],
    ['high', 0],
    ['warning', 0],
    ['info', 0],
  ]);
  for (const record of records) {
    const level = normalizeInterceptLevel(readString(record, ['severity', 'level', 'status'], 'info'));
    buckets.set(level, (buckets.get(level) ?? 0) + 1);
  }

  return [
    { id: 'critical', title: 'Critical security events', count: buckets.get('critical') ?? 0, level: 'critical' },
    { id: 'high', title: 'High risk events', count: buckets.get('high') ?? 0, level: 'high' },
    { id: 'warning', title: 'Policy warnings', count: buckets.get('warning') ?? 0, level: 'warning' },
    { id: 'info', title: 'Informational audit events', count: buckets.get('info') ?? 0, level: 'info' },
  ];
}

function buildAuditLogs(records: UnknownRecord[]): SecurityAuditLog[] {
  return records.slice(0, 8).map((record, index) => ({
    action: readString(record, ['action', 'eventType', 'type', 'summary'], 'audit.record'),
    id: readString(record, ['recordId', 'id'], `security-audit-${index + 1}`),
    time: readString(record, ['recordedAt', 'createdAt', 'time'], ''),
    user: readString(record, ['actorId', 'createdBy', 'userId', 'tenantId'], 'system'),
  }));
}

class SecurityService {
  async getDashboardData(): Promise<SecurityDashboardData> {
    const app = getAppSdkClientWithSession();
    const [governance, access] = await Promise.all([
      app.portal.governance.retrieve(),
      app.portal.access.retrieve(),
    ]);
    const normalizedHealth = asRecord(governance);
    const records = readRecords(asRecord(access), ['items', 'data', 'records', 'auditLogs', 'securityEvents', 'alerts']);

    return {
      auditLogs: buildAuditLogs(records),
      healthScore: deriveHealthScore(normalizedHealth, records),
      intercepts: buildIntercepts(normalizedHealth, records),
    };
  }
}

export const securityService = new SecurityService();

import { getBackendSdkClientWithSession } from '@sdkwork/im-admin-core/sdk';

export interface AuditLog {
  id: string;
  time: string;
  actor: string;
  action: string;
  resource: string;
  ip: string;
}

export interface ComplianceData {
  systemSecure: boolean;
  legalHolds: number;
  uptime: string;
  auditLogs: AuditLog[];
}

type UnknownRecord = Record<string, unknown>;

function asRecord(value: unknown): UnknownRecord {
  return value && typeof value === 'object' && !Array.isArray(value) ? value as UnknownRecord : {};
}

function asRecordArray(value: unknown): UnknownRecord[] {
  return Array.isArray(value) ? value.map(asRecord).filter((item) => Object.keys(item).length > 0) : [];
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

function readBoolean(record: UnknownRecord, keys: string[], fallback = false): boolean {
  for (const key of keys) {
    const value = record[key];
    if (typeof value === 'boolean') {
      return value;
    }
    if (typeof value === 'string' && value.trim()) {
      return ['1', 'true', 'yes', 'healthy', 'secure', 'ok', 'ready'].includes(value.trim().toLowerCase());
    }
  }
  return fallback;
}

function readRecords(record: UnknownRecord, keys: string[]): UnknownRecord[] {
  for (const key of keys) {
    const records = asRecordArray(record[key]);
    if (records.length > 0) {
      return records;
    }
  }
  return [];
}

function formatUptime(health: UnknownRecord): string {
  const explicit = readString(health, ['uptime', 'uptimePercent', 'availability'], '');
  if (explicit) {
    return explicit.endsWith('%') ? explicit : `${explicit}%`;
  }
  const uptimePercent = readNumber(health, ['availabilityPercent', 'slaPercent'], Number.NaN);
  if (Number.isFinite(uptimePercent)) {
    return `${uptimePercent.toFixed(3)}%`;
  }
  return '0%';
}

function hasCriticalSignals(health: UnknownRecord, records: UnknownRecord[]): boolean {
  const healthStatus = readString(health, ['status', 'state', 'health'], '').toLowerCase();
  if (['critical', 'failed', 'down', 'error', 'unhealthy'].includes(healthStatus)) {
    return true;
  }
  return records.some((record) => {
    const severity = readString(record, ['severity', 'level', 'status'], '').toLowerCase();
    const action = readString(record, ['action', 'eventType', 'type'], '').toLowerCase();
    return ['critical', 'failed', 'error', 'violation'].includes(severity)
      || action.includes('critical')
      || action.includes('security_violation');
  });
}

function normalizeAuditLog(record: UnknownRecord, index: number): AuditLog {
  const aggregateType = readString(record, ['aggregateType', 'resourceType'], 'system');
  const aggregateId = readString(record, ['aggregateId', 'resourceId'], '');
  const payload = asRecord(record.payload);
  return {
    action: readString(record, ['action', 'eventType', 'type'], 'audit.record'),
    actor: readString(record, ['actorId', 'createdBy', 'userId', 'tenantId'], 'system'),
    id: readString(record, ['recordId', 'id'], `audit-${index + 1}`),
    ip: readString(record, ['ip', 'ipAddress', 'remoteIp', 'clientIp'], readString(payload, ['ip', 'ipAddress'], '')),
    resource: aggregateId ? `${aggregateType}: ${aggregateId}` : aggregateType,
    time: readString(record, ['recordedAt', 'createdAt', 'time'], ''),
  };
}

class AdminComplianceService {
  async getComplianceData(searchTerm: string): Promise<ComplianceData> {
    const backend = getBackendSdkClientWithSession();
    const [health, auditRecords] = await Promise.all([
      backend.ops.health.retrieve(),
      backend.audit.records.list(),
    ]);
    const normalizedHealth = asRecord(health);
    const records = readRecords(asRecord(auditRecords), ['items', 'data', 'records', 'auditLogs']);
    const query = searchTerm.trim().toLowerCase();
    const auditLogs = records
      .map(normalizeAuditLog)
      .filter((log) => !query
        || log.action.toLowerCase().includes(query)
        || log.actor.toLowerCase().includes(query)
        || log.resource.toLowerCase().includes(query)
        || log.ip.toLowerCase().includes(query));

    return {
      auditLogs,
      legalHolds: readNumber(normalizedHealth, ['legalHolds', 'activeLegalHolds'], 0),
      systemSecure: readBoolean(normalizedHealth, ['systemSecure', 'secure'], !hasCriticalSignals(normalizedHealth, records)),
      uptime: formatUptime(normalizedHealth),
    };
  }
}

export const adminComplianceService = new AdminComplianceService();

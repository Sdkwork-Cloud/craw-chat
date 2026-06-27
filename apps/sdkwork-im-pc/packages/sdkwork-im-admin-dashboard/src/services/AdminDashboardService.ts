import { getBackendSdkClientWithSession } from '@sdkwork/im-admin-core/sdk';

export interface AdminMetrics {
  systemLoad: { value: string; trend: string; isUp: boolean };
  activeTenants: { value: string; trend: string; isUp: boolean };
  activeConnections: { value: string; trend: string; isUp: boolean };
  globalNodes: { value: string; trend: string; isUp: boolean };
}

export interface NetworkThroughput {
  egress: number;
  ingress: number;
}

export interface SystemAnomaly {
  id: string;
  type: 'critical' | 'warning' | 'info';
  tenant: string;
  message: string;
  time: string;
}

export interface AdminDashboardData {
  metrics: AdminMetrics;
  throughput: NetworkThroughput[];
  anomalies: SystemAnomaly[];
}

type UnknownRecord = Record<string, unknown>;

function asRecord(value: unknown): UnknownRecord {
  return value && typeof value === 'object' && !Array.isArray(value) ? value as UnknownRecord : {};
}

function asRecordArray(value: unknown): UnknownRecord[] {
  return Array.isArray(value) ? value.map(asRecord).filter((item) => Object.keys(item).length > 0) : [];
}

function readNumber(record: UnknownRecord, keys: string[], fallback = 0): number {
  for (const key of keys) {
    const value = record[key];
    if (typeof value === 'number' && Number.isFinite(value)) {
      return value;
    }
    if (typeof value === 'string' && value.trim()) {
      const parsed = Number(value);
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

function formatCount(value: number): string {
  if (value >= 1_000_000) {
    return `${(value / 1_000_000).toFixed(1)}M`;
  }
  if (value >= 1_000) {
    return `${(value / 1_000).toFixed(value >= 10_000 ? 0 : 1)}K`;
  }
  return String(Math.max(0, Math.round(value)));
}

function formatPercent(value: number): string {
  return `${Math.max(0, Math.min(100, Math.round(value)))}%`;
}

function resolveSystemLoad(health: UnknownRecord, diagnostics: UnknownRecord): number {
  const direct = readNumber(health, ['systemLoad', 'loadPercent', 'cpuUsagePercent'], Number.NaN);
  if (Number.isFinite(direct)) {
    return direct;
  }
  const nodes = asRecordArray(diagnostics.clientRoutes);
  return nodes.length > 0 ? Math.min(100, nodes.length * 8) : 0;
}

function resolveActiveConnections(health: UnknownRecord, cluster: UnknownRecord): number {
  const direct = readNumber(health, ['activeConnections', 'connectionCount', 'websocketConnections'], Number.NaN);
  if (Number.isFinite(direct)) {
    return direct;
  }
  return asRecordArray(cluster.nodes)
    .reduce((total, node) => total + readNumber(node, ['connectionCount', 'connections', 'clientRouteCount'], 0), 0);
}

function buildThroughput(health: UnknownRecord, diagnostics: UnknownRecord): NetworkThroughput[] {
  const samples = asRecordArray(health.throughputSamples)
    .concat(asRecordArray(diagnostics.throughputSamples));
  return samples.slice(0, 12).map((sample) => ({
    egress: readNumber(sample, ['egress', 'egressPercent', 'outbound'], 0),
    ingress: readNumber(sample, ['ingress', 'ingressPercent', 'inbound'], 0),
  }));
}

function buildAnomalies(records: UnknownRecord[]): SystemAnomaly[] {
  return records.slice(0, 4).map((record, index) => {
    const action = readString(record, ['action', 'eventType', 'type'], 'backend.audit');
    const aggregate = readString(record, ['aggregateId', 'recordId', 'id'], `record-${index + 1}`);
    return {
      id: readString(record, ['recordId', 'id'], `audit-${index + 1}`),
      message: action,
      tenant: readString(record, ['tenantId', 'aggregateType'], 'System'),
      time: readString(record, ['recordedAt', 'createdAt', 'time'], ''),
      type: action.toLowerCase().includes('error') || action.toLowerCase().includes('fail') ? 'critical' : 'info',
      ...(aggregate ? { message: `${action} (${aggregate})` } : {}),
    };
  });
}

class AdminDashboardService {
  async getDashboardData(): Promise<AdminDashboardData> {
    const backend = getBackendSdkClientWithSession();
    const [health, cluster, diagnostics, auditRecords] = await Promise.all([
      backend.ops.health.retrieve(),
      backend.ops.cluster.retrieve(),
      backend.ops.diagnostics.retrieve(),
      backend.audit.records.list(),
    ]);
    const normalizedHealth = asRecord(health);
    const normalizedCluster = asRecord(cluster);
    const normalizedDiagnostics = asRecord(diagnostics);
    const nodeCount = asRecordArray(normalizedCluster.nodes).length;
    const activeConnections = resolveActiveConnections(normalizedHealth, normalizedCluster);
    const systemLoad = resolveSystemLoad(normalizedHealth, normalizedDiagnostics);
    const records = asRecordArray(asRecord(auditRecords).items);

    return {
      metrics: {
        systemLoad: { value: formatPercent(systemLoad), trend: '', isUp: systemLoad < 80 },
        activeTenants: { value: formatCount(readNumber(normalizedHealth, ['activeTenants', 'tenantCount'], 0)), trend: '', isUp: true },
        activeConnections: { value: formatCount(activeConnections), trend: '', isUp: true },
        globalNodes: { value: String(nodeCount), trend: '', isUp: nodeCount > 0 },
      },
      throughput: buildThroughput(normalizedHealth, normalizedDiagnostics),
      anomalies: buildAnomalies(records),
    };
  }
}

export const adminDashboardService = new AdminDashboardService();

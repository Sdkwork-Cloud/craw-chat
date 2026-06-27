import { getBackendSdkClientWithSession } from '@sdkwork/im-admin-core/sdk';

export interface ServerNode {
  region: string;
  status: 'healthy' | 'warning' | 'error';
  cpu: number;
  mem: number;
  connections: string;
}

export interface MetricItem {
  title: string;
  value: string;
  usage: number;
}

export interface InfraStatusData {
  metrics: {
    connectionPool: MetricItem;
    dbIops: MetricItem;
    redisHitRate: MetricItem;
  };
  nodes: ServerNode[];
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

function normalizeUsage(value: number): number {
  return Math.max(0, Math.min(100, Math.round(value)));
}

function resolveStatus(node: UnknownRecord): ServerNode['status'] {
  const status = readString(node, ['status', 'health', 'drainStatus'], 'healthy').toLowerCase();
  if (status.includes('error') || status.includes('down') || status.includes('failed')) {
    return 'error';
  }
  if (status.includes('warning') || status.includes('drain')) {
    return 'warning';
  }
  return 'healthy';
}

function mapNode(node: UnknownRecord, index: number): ServerNode {
  const routeCount = readNumber(node, ['clientRouteCount', 'ownedRouteCount', 'connectionCount'], 0);
  return {
    connections: formatCount(routeCount),
    cpu: normalizeUsage(readNumber(node, ['cpu', 'cpuUsage', 'cpuUsagePercent'], 0)),
    mem: normalizeUsage(readNumber(node, ['mem', 'memory', 'memoryUsage', 'memoryUsagePercent'], 0)),
    region: readString(node, ['region', 'nodeId', 'profile'], `node-${index + 1}`),
    status: resolveStatus(node),
  };
}

class InfraStatusService {
  async getStatusData(): Promise<InfraStatusData> {
    const backend = getBackendSdkClientWithSession();
    const [health, cluster, diagnostics] = await Promise.all([
      backend.ops.health.retrieve(),
      backend.ops.cluster.retrieve(),
      backend.ops.diagnostics.retrieve(),
    ]);
    const normalizedHealth = asRecord(health);
    const normalizedCluster = asRecord(cluster);
    const normalizedDiagnostics = asRecord(diagnostics);
    const nodes = asRecordArray(normalizedCluster.nodes).map(mapNode);
    const activeConnections = nodes.reduce((total, node) => {
      const value = node.connections.endsWith('K')
        ? Number(node.connections.slice(0, -1)) * 1_000
        : node.connections.endsWith('M')
          ? Number(node.connections.slice(0, -1)) * 1_000_000
          : Number(node.connections);
      return total + (Number.isFinite(value) ? value : 0);
    }, 0);
    const projectionMetrics = asRecord(asRecord(normalizedHealth.projectionPlane).metrics);
    const conversationPersist = asRecord(projectionMetrics.conversationSnapshotPersist);
    const clientRoutePersist = asRecord(projectionMetrics.clientRouteSyncSnapshotPersist);
    const dbIops = readNumber(conversationPersist, ['successCount'], 0)
      + readNumber(clientRoutePersist, ['successCount'], 0);
    const realtimeInbox = asRecord(normalizedHealth.realtimeInbox);
    const redisHitRate = 100 - normalizeUsage(readNumber(realtimeInbox, ['maxClientRouteWindowUsagePermille'], 0) / 10);

    return {
      metrics: {
        connectionPool: { title: 'Global Connection Pool', value: formatCount(activeConnections), usage: normalizeUsage(activeConnections / 10_000) },
        dbIops: { title: 'Database IOPS (Avg)', value: formatCount(dbIops), usage: normalizeUsage(dbIops / 1_000) },
        redisHitRate: { title: 'Realtime Window Health', value: `${redisHitRate}%`, usage: redisHitRate },
      },
      nodes: nodes.length > 0 ? nodes : asRecordArray(normalizedDiagnostics.clientRoutes).map(mapNode),
    };
  }
}

export const infraStatusService = new InfraStatusService();

import { getAppbaseBackendSdkClientWithSession } from '@sdkwork/im-admin-core/sdk';

export interface Tenant {
  id: string;
  name: string;
  plan: 'Enterprise' | 'Business' | 'Pro';
  users: string;
  status: 'active' | 'warning';
  revenue: string;
  region: string;
}

export interface GetTenantsResponse {
  data: Tenant[];
  total: number;
}

type UnknownRecord = Record<string, unknown>;

function asRecord(value: unknown): UnknownRecord {
  return value && typeof value === 'object' && !Array.isArray(value) ? value as UnknownRecord : {};
}

function unwrapAppbaseResult(value: unknown): unknown {
  const record = asRecord(value);
  if (!('code' in record) && !('data' in record)) {
    return value;
  }

  const code = record.code;
  const normalizedCode = code === undefined || code === null ? '2000' : String(code).trim();
  if (!['0', '200', '2000'].includes(normalizedCode)) {
    throw new Error(String(record.message || record.msg || 'Appbase backend tenant request failed'));
  }
  return record.data;
}

function readRecords(value: unknown): UnknownRecord[] {
  const unwrapped = unwrapAppbaseResult(value);
  if (Array.isArray(unwrapped)) {
    return unwrapped.map(asRecord).filter((record) => Object.keys(record).length > 0);
  }
  const record = asRecord(unwrapped);
  for (const key of ['items', 'records', 'data', 'list', 'rows', 'content', 'tenants']) {
    const nested = record[key];
    if (Array.isArray(nested)) {
      return nested.map(asRecord).filter((item) => Object.keys(item).length > 0);
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

function readTotal(value: unknown, fallback: number): number {
  const record = asRecord(unwrapAppbaseResult(value));
  return readNumber(record, ['total', 'totalElements', 'totalCount', 'count'], fallback);
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

function formatCurrency(value: number): string {
  if (!Number.isFinite(value) || value <= 0) {
    return '$0';
  }
  return new Intl.NumberFormat('en-US', {
    currency: 'USD',
    maximumFractionDigits: 0,
    style: 'currency',
  }).format(value);
}

function normalizePlan(value: unknown): Tenant['plan'] {
  const plan = String(value ?? '').trim().toLowerCase();
  if (plan.includes('enterprise')) {
    return 'Enterprise';
  }
  if (plan.includes('business')) {
    return 'Business';
  }
  return 'Pro';
}

function normalizeStatus(value: unknown): Tenant['status'] {
  const status = String(value ?? '').trim().toLowerCase();
  return status === 'warning' || status === 'suspended' || status === 'limited'
    ? 'warning'
    : 'active';
}

function mapTenant(record: UnknownRecord): Tenant {
  const id = readString(record, ['tenantId', 'tenant_id', 'id'], 'tenant');
  return {
    id,
    name: readString(record, ['name', 'displayName', 'display_name', 'tenantName', 'tenant_name'], id),
    plan: normalizePlan(readString(record, ['plan', 'planName', 'tier', 'subscriptionPlan'], 'Pro')),
    region: readString(record, ['region', 'regionName', 'dataRegion'], ''),
    revenue: formatCurrency(readNumber(record, ['revenue', 'mrr', 'monthlyRevenue'], 0)),
    status: normalizeStatus(readString(record, ['status', 'state'], 'active')),
    users: formatCount(readNumber(record, ['users', 'userCount', 'memberCount', 'members'], 0)),
  };
}

class TenantService {
  async getTenants(params: { search?: string }): Promise<GetTenantsResponse> {
    const response = await getAppbaseBackendSdkClientWithSession().iam.tenants.list({
      ...(params.search?.trim() ? { q: params.search.trim() } : {}),
    });
    const records = readRecords(response);
    const data = records.map(mapTenant);
    return {
      data,
      total: readTotal(response, data.length),
    };
  }
}

export const tenantService = new TenantService();

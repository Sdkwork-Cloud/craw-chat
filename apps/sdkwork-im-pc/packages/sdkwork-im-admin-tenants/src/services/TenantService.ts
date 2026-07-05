import { getAppbaseBackendSdkClientWithSession } from '@sdkwork/im-admin-core/sdk';
import { extractBackendSdkRecords, mapAppSdkOffsetPage, readBackendPageTotal, readNumber, readString, SDKWORK_DEFAULT_PAGE_SIZE } from '@sdkwork/im-admin-core/sdk/backendSdkResponseHelpers';

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
  async getTenants(params: { search?: string; page?: number } = {}): Promise<GetTenantsResponse> {
    const page = Math.max(1, params.page ?? 1);
    const response = await getAppbaseBackendSdkClientWithSession().iam.tenants.list({
      page,
      pageSize: SDKWORK_DEFAULT_PAGE_SIZE,
      ...(params.search?.trim() ? { q: params.search.trim() } : {}),
    });
    const mapped = mapAppSdkOffsetPage(response, mapTenant, page, SDKWORK_DEFAULT_PAGE_SIZE);
    return {
      data: mapped.items,
      total: mapped.totalItems ?? readBackendPageTotal(response, mapped.items.length),
    };
  }
}

export const tenantService = new TenantService();

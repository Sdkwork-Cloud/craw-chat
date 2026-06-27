import { getAppbaseBackendSdkClientWithSession } from '@sdkwork/im-admin-core/sdk';

export interface GlobalUser {
  id: string;
  uin: string;
  name: string;
  email: string;
  tenant: string;
  security: string;
  status: 'active' | 'banned' | 'warning';
}

export interface GetGlobalUsersResponse {
  data: GlobalUser[];
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
    throw new Error(String(record.message || record.msg || 'Appbase backend user request failed'));
  }
  return record.data;
}

function readRecords(value: unknown): UnknownRecord[] {
  const unwrapped = unwrapAppbaseResult(value);
  if (Array.isArray(unwrapped)) {
    return unwrapped.map(asRecord).filter((record) => Object.keys(record).length > 0);
  }
  const record = asRecord(unwrapped);
  for (const key of ['items', 'records', 'data', 'list', 'rows', 'content', 'users']) {
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
      const parsed = Number(value.replace(/[,%\s]/gu, ''));
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

function normalizeStatus(value: unknown): GlobalUser['status'] {
  const status = String(value ?? '').trim().toLowerCase();
  if (status === 'banned' || status === 'blocked' || status === 'disabled' || status === 'deleted') {
    return 'banned';
  }
  if (status === 'warning' || status === 'pending' || status === 'pending_verification' || status === 'locked') {
    return 'warning';
  }
  return 'active';
}

function mapStatusFilter(status?: string): string | undefined {
  if (!status || status === 'All Global Statuses') {
    return undefined;
  }
  const statusMap: Record<string, string> = {
    'Active Accounts': 'active',
    'Banned Globally': 'banned',
    'Pending Verification': 'pending',
  };
  return statusMap[status] ?? status;
}

function mapUser(record: UnknownRecord): GlobalUser {
  const id = readString(record, ['userId', 'user_id', 'id', 'accountId'], 'user');
  const displayName = readString(record, ['displayName', 'display_name', 'name', 'nickname', 'username'], id);
  const tenantId = readString(record, ['tenantId', 'tenant_id'], '');
  const tenantName = readString(record, ['tenantName', 'tenant_name', 'tenant'], tenantId);
  const mfaEnabled = readString(record, ['mfaEnabled', 'mfa_enabled', 'multiFactorEnabled'], '');
  const security = readString(
    record,
    ['security', 'securityStatus', 'security_status'],
    mfaEnabled === 'true' ? 'MFA Enforced' : 'Password Only',
  );
  return {
    email: readString(record, ['email'], ''),
    id,
    name: displayName,
    security,
    status: normalizeStatus(readString(record, ['status', 'state'], 'active')),
    tenant: tenantName && tenantId && tenantName !== tenantId ? `${tenantName} (${tenantId})` : tenantName,
    uin: readString(record, ['uin', 'userNo', 'user_no', 'accountNo'], id),
  };
}

class GlobalUserService {
  async getGlobalUsers(params: { search?: string; status?: string }): Promise<GetGlobalUsersResponse> {
    const response = await getAppbaseBackendSdkClientWithSession().iam.users.list({
      ...(params.search?.trim() ? { q: params.search.trim() } : {}),
      ...(mapStatusFilter(params.status) ? { status: mapStatusFilter(params.status) } : {}),
    });
    const records = readRecords(response);
    const data = records.map(mapUser);
    return {
      data,
      total: readTotal(response, data.length),
    };
  }

  async updateUserStatus(id: string, status: GlobalUser['status']): Promise<void> {
    const userId = id.trim();
    if (!userId) {
      throw new Error('user id is required');
    }
    await getAppbaseBackendSdkClientWithSession().iam.users.update(userId, { status });
  }

  async deleteUser(id: string): Promise<void> {
    const userId = id.trim();
    if (!userId) {
      throw new Error('user id is required');
    }
    await getAppbaseBackendSdkClientWithSession().iam.users.delete(userId);
  }
}

export const globalUserService = new GlobalUserService();

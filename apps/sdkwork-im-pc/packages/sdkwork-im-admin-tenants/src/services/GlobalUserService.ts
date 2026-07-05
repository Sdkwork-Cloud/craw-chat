import { getAppbaseBackendSdkClientWithSession } from '@sdkwork/im-admin-core/sdk';
import {
  extractBackendSdkRecords,
  mapAppSdkOffsetPage,
  readBackendPageTotal,
  readString,
  SDKWORK_DEFAULT_PAGE_SIZE,
} from '@sdkwork/im-admin-core/sdk/backendSdkResponseHelpers';

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
  async getGlobalUsers(params: { search?: string; status?: string; page?: number } = {}): Promise<GetGlobalUsersResponse> {
    const page = Math.max(1, params.page ?? 1);
    const response = await getAppbaseBackendSdkClientWithSession().iam.users.list({
      page,
      pageSize: SDKWORK_DEFAULT_PAGE_SIZE,
      ...(params.search?.trim() ? { q: params.search.trim() } : {}),
      ...(mapStatusFilter(params.status) ? { status: mapStatusFilter(params.status) } : {}),
    });
    const mapped = mapAppSdkOffsetPage(response, mapUser, page, SDKWORK_DEFAULT_PAGE_SIZE);
    return {
      data: mapped.items,
      total: mapped.totalItems ?? readBackendPageTotal(response, mapped.items.length),
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

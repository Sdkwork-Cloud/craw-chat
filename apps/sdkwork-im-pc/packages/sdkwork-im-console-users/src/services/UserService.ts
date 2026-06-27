import { getAppbaseAppSdkClientWithSession } from '@sdkwork/im-pc-core';

export interface User {
  id: string;
  name: string;
  email: string;
  role: 'admin' | 'member';
  department: string;
  status: 'active' | 'offline' | 'disabled';
  lastLogin: string;
}

export interface GetUsersResponse {
  data: User[];
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

function normalizeRole(record: UnknownRecord): User['role'] {
  const role = readString(record, ['role', 'roleCode', 'role_code', 'roleName'], '').toLowerCase();
  return role.includes('admin') || role.includes('owner') ? 'admin' : 'member';
}

function normalizeStatus(value: unknown): User['status'] {
  const status = String(value ?? '').trim().toLowerCase();
  if (status === 'disabled' || status === 'banned' || status === 'blocked' || status === 'deleted') {
    return 'disabled';
  }
  if (status === 'offline' || status === 'inactive') {
    return 'offline';
  }
  return 'active';
}

function mapUser(record: UnknownRecord): User {
  const id = readString(record, ['userId', 'user_id', 'id', 'accountId'], 'user');
  return {
    department: readString(record, ['departmentName', 'department_name', 'department', 'orgName'], ''),
    email: readString(record, ['email'], ''),
    id,
    lastLogin: readString(record, ['lastLoginAt', 'last_login_at', 'lastLogin', 'updatedAt'], ''),
    name: readString(record, ['displayName', 'display_name', 'name', 'nickname', 'username'], id),
    role: normalizeRole(record),
    status: normalizeStatus(readString(record, ['status', 'state'], 'active')),
  };
}

function normalizeMemberRecord(record: UnknownRecord): UnknownRecord {
  const user = asRecord(record.user);
  const profile = asRecord(record.profile);
  return {
    ...record,
    ...user,
    ...profile,
    departmentName: readString(record, ['departmentName', 'department_name', 'department'], readString(profile, ['departmentName', 'department_name', 'department'], '')),
    role: readString(record, ['role', 'roleCode', 'roleName'], readString(profile, ['role', 'roleCode', 'roleName'], '')),
  };
}

class UserService {
  async getUsers(params: { page: number; pageSize: number; search?: string }): Promise<GetUsersResponse> {
    const client = getAppbaseAppSdkClientWithSession();
    const response = await client.iam.organizationMemberships.list({
      page: params.page,
      pageSize: params.pageSize,
      ...(params.search?.trim() ? { q: params.search.trim() } : {}),
    });
    const records = readRecords(response);
    const data = records.map(normalizeMemberRecord).map(mapUser);
    return {
      data,
      total: readTotal(response, data.length),
    };
  }

  async deleteUser(id: string): Promise<void> {
    const userId = id.trim();
    if (!userId) {
      throw new Error('user id is required');
    }
    throw new Error(
      `Deleting tenant users is an admin-only backend SDK capability. Move user ${userId} management to the admin surface or add an app-api console contract.`,
    );
  }
}

export const userService = new UserService();

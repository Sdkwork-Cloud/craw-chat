import { getAppbaseAppSdkClientWithSession } from '@sdkwork/im-pc-core';
import {
  asRecord,
  extractAppSdkRecordsFromResult,
  readRecordNumber,
  readRecordString,
  unwrapSdkWorkApiEnvelope,
} from '@sdkwork/im-pc-core/sdk/appSdkResponseHelpers';

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

function toRecord(value: unknown): UnknownRecord {
  return asRecord(value) ?? {};
}

function readRecords(value: unknown): UnknownRecord[] {
  return extractAppSdkRecordsFromResult(value);
}

function readTotal(value: unknown, fallback: number): number {
  const record = toRecord(unwrapSdkWorkApiEnvelope(value));
  return readRecordNumber(record, ['total', 'totalElements', 'totalCount', 'count'], fallback);
}

function normalizeRole(record: UnknownRecord): User['role'] {
  const role = readRecordString(record, ['role', 'roleCode', 'role_code', 'roleName'], '').toLowerCase();
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
  const id = readRecordString(record, ['userId', 'user_id', 'id', 'accountId'], 'user');
  return {
    department: readRecordString(record, ['departmentName', 'department_name', 'department', 'orgName'], ''),
    email: readRecordString(record, ['email'], ''),
    id,
    lastLogin: readRecordString(record, ['lastLoginAt', 'last_login_at', 'lastLogin', 'updatedAt'], ''),
    name: readRecordString(record, ['displayName', 'display_name', 'name', 'nickname', 'username'], id),
    role: normalizeRole(record),
    status: normalizeStatus(readRecordString(record, ['status', 'state'], 'active')),
  };
}

function normalizeMemberRecord(record: UnknownRecord): UnknownRecord {
  const user = toRecord(record.user);
  const profile = toRecord(record.profile);
  return {
    ...record,
    ...user,
    ...profile,
    departmentName: readRecordString(record, ['departmentName', 'department_name', 'department'], readRecordString(profile, ['departmentName', 'department_name', 'department'], '')),
    role: readRecordString(record, ['role', 'roleCode', 'roleName'], readRecordString(profile, ['role', 'roleCode', 'roleName'], '')),
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

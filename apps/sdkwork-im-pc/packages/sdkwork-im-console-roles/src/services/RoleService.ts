import { getAppbaseAppSdkClientWithSession } from '@sdkwork/im-pc-core';

export interface Role {
  id: string;
  name: string;
  desc: string;
  count: number;
  system: boolean;
}

export interface GetRolesResponse {
  data: Role[];
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
    throw new Error(String(record.message || record.msg || 'Appbase backend role request failed'));
  }
  return record.data;
}

function readRecords(value: unknown): UnknownRecord[] {
  const unwrapped = unwrapAppbaseResult(value);
  if (Array.isArray(unwrapped)) {
    return unwrapped.map(asRecord).filter((record) => Object.keys(record).length > 0);
  }
  const record = asRecord(unwrapped);
  for (const key of ['items', 'records', 'data', 'list', 'rows', 'content', 'roles']) {
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

function readBoolean(record: UnknownRecord, keys: string[], fallback = false): boolean {
  for (const key of keys) {
    const value = record[key];
    if (typeof value === 'boolean') {
      return value;
    }
    if (typeof value === 'number' && Number.isFinite(value)) {
      return value !== 0;
    }
    if (typeof value === 'string' && value.trim()) {
      const normalized = value.trim().toLowerCase();
      if (['1', 'true', 'yes', 'system', 'builtin', 'built_in'].includes(normalized)) {
        return true;
      }
      if (['0', 'false', 'no', 'custom'].includes(normalized)) {
        return false;
      }
    }
  }
  return fallback;
}

function readTotal(value: unknown, fallback: number): number {
  const record = asRecord(unwrapAppbaseResult(value));
  return readNumber(record, ['total', 'totalElements', 'totalCount', 'count'], fallback);
}

function mapRole(record: UnknownRecord): Role {
  const id = readString(record, ['roleId', 'role_id', 'id', 'code'], 'role');
  return {
    count: readNumber(record, ['memberCount', 'bindingCount', 'userCount', 'count'], 0),
    desc: readString(record, ['description', 'desc', 'remark'], ''),
    id,
    name: readString(record, ['name', 'displayName', 'display_name', 'roleName', 'role_name'], id),
    system: readBoolean(record, ['system', 'systemRole', 'builtIn', 'builtin', 'roleType'], false),
  };
}

function toRoleUpdateCommand(updates: Partial<Role>): Record<string, unknown> {
  return {
    ...(updates.name !== undefined ? { name: updates.name } : {}),
    ...(updates.desc !== undefined ? { description: updates.desc } : {}),
    ...(updates.system !== undefined ? { system: updates.system } : {}),
  };
}

class RoleService {
  async getRoles(): Promise<GetRolesResponse> {
    const response = await getAppbaseAppSdkClientWithSession().iam.roleBindings.list({});
    const records = readRecords(response);
    const roleMap = new Map<string, Role>();
    for (const record of records) {
      const role = mapRole(record);
      const existing = roleMap.get(role.id);
      roleMap.set(role.id, existing ? { ...existing, count: existing.count + Math.max(1, role.count) } : { ...role, count: Math.max(1, role.count) });
    }
    const data = [...roleMap.values()];
    return {
      data,
      total: readTotal(response, data.length),
    };
  }

  async updateRole(id: string, updates: Partial<Role>): Promise<Role> {
    const roleId = id.trim();
    if (!roleId) {
      throw new Error('role id is required');
    }
    const command = toRoleUpdateCommand(updates);
    throw new Error(
      `Updating role ${roleId} is an admin-only backend SDK capability. Move role management to the admin surface or add an app-api console contract. Requested fields: ${Object.keys(command).join(', ') || 'none'}.`,
    );
  }
}

export const roleService = new RoleService();

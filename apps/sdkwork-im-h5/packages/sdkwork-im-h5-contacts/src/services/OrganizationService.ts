import { getIamAppSdkClient } from "@sdkwork/im-h5-core/sdk";

const DIRECTORY_PAGE_SIZE = 100;
const DIRECTORY_MAX_PAGES = 50;
const DIRECTORY_CACHE_TTL_MS = 5 * 60 * 1000;

export interface OrgDepartment {
  id: string;
  name: string;
  parentId: string | null;
  count: number;
  /** Present when the wire record carries the owning organization. */
  organizationId?: string;
}

export interface OrgMember {
  id: string;
  name: string;
  avatar: string;
  jobTitle: string;
  departmentId: string;
}

export interface Organization {
  id: string;
  name: string;
  logo: string;
}

interface DirectoryPage {
  items: Record<string, unknown>[];
  pageInfo: {
    mode: string;
    hasMore?: boolean;
    nextCursor?: string | null;
  };
}

/** All departments of every organization, cached for the client session. */
let allDepartmentsCache: OrgDepartment[] | null = null;
let allDepartmentsCachedAt = 0;

interface CachedMembers {
  members: OrgMember[];
  cachedAt: number;
}

/** Members keyed by `${organizationId ?? "*"}:${departmentId}`. */
const membersByDepartmentCache = new Map<string, CachedMembers>();

/** Members keyed by organization id (used for in-memory member search). */
const membersByOrganizationCache = new Map<string, CachedMembers>();

function isCacheFresh(cachedAt: number): boolean {
  return Date.now() - cachedAt < DIRECTORY_CACHE_TTL_MS;
}

function pickString(...values: unknown[]): string | undefined {
  for (const value of values) {
    if (typeof value === "string" && value.trim().length > 0) {
      return value.trim();
    }
  }
  return undefined;
}

function pickNumber(...values: unknown[]): number | undefined {
  for (const value of values) {
    if (typeof value === "number" && Number.isFinite(value)) {
      return value;
    }
    if (typeof value === "string" && value.trim() !== "" && Number.isFinite(Number(value))) {
      return Number(value);
    }
  }
  return undefined;
}

async function fetchCursorPages(
  fetchPage: (cursor?: string) => Promise<DirectoryPage>,
): Promise<Record<string, unknown>[]> {
  const records: Record<string, unknown>[] = [];
  let cursor: string | undefined;
  for (let depth = 0; depth < DIRECTORY_MAX_PAGES; depth += 1) {
    const page = await fetchPage(cursor);
    records.push(...page.items);
    if (!page.pageInfo.hasMore || !page.pageInfo.nextCursor) {
      break;
    }
    cursor = page.pageInfo.nextCursor;
  }
  return records;
}

function mapOrganization(record: Record<string, unknown>): Organization | undefined {
  const id = pickString(record.organizationId, record.organization_id, record.id);
  const name = pickString(
    record.name,
    record.displayName,
    record.display_name,
    record.organizationName,
    record.organization_name,
  );
  if (!id || !name) {
    return undefined;
  }
  return {
    id,
    name,
    logo: pickString(record.logoUrl, record.logo_url, record.logo, record.avatarUrl, record.avatar_url) ?? "",
  };
}

function mapDepartment(record: Record<string, unknown>): OrgDepartment | undefined {
  const id = pickString(record.departmentId, record.department_id, record.id, record.nodeId, record.node_id);
  const name = pickString(
    record.name,
    record.displayName,
    record.display_name,
    record.departmentName,
    record.department_name,
    record.title,
    record.label,
  );
  if (!id || !name) {
    return undefined;
  }
  return {
    id,
    name,
    parentId: pickString(record.parentDepartmentId, record.parent_department_id, record.parentId, record.parent_id) ?? null,
    count: pickNumber(
      record.memberCount,
      record.member_count,
      record.usersCount,
      record.users_count,
      record.userCount,
      record.user_count,
    ) ?? 0,
    ...(pickString(record.organizationId, record.organization_id)
      ? { organizationId: pickString(record.organizationId, record.organization_id) }
      : {}),
  };
}

function mapMember(record: Record<string, unknown>, departmentId: string): OrgMember | undefined {
  const id = pickString(record.userId, record.user_id, record.personId, record.person_id, record.id);
  if (!id) {
    return undefined;
  }
  const name = pickString(record.displayName, record.display_name, record.name, record.nickname, record.username, id) ?? id;
  return {
    id,
    name,
    avatar: pickString(record.avatarUrl, record.avatar_url, record.avatar) ?? "",
    jobTitle: pickString(record.positionName, record.position_name, record.position, record.jobTitle, record.job_title) ?? "",
    departmentId: pickString(record.departmentId, record.department_id) ?? departmentId,
  };
}

function uniqueById<T extends { id: string }>(items: T[]): T[] {
  const byId = new Map<string, T>();
  for (const item of items) {
    byId.set(item.id, item);
  }
  return Array.from(byId.values());
}

async function fetchAllDepartments(): Promise<OrgDepartment[]> {
  if (allDepartmentsCache && isCacheFresh(allDepartmentsCachedAt)) {
    return allDepartmentsCache;
  }
  const client = getIamAppSdkClient();
  const records = await fetchCursorPages((cursor) =>
    client.iam.departments.list({
      pageSize: DIRECTORY_PAGE_SIZE,
      ...(cursor ? { cursor } : {}),
    }),
  );
  allDepartmentsCache = uniqueById(
    records.map(mapDepartment).filter((department): department is OrgDepartment => department !== undefined),
  );
  allDepartmentsCachedAt = Date.now();
  return allDepartmentsCache;
}

/**
 * The wire records are fallback-grouped into the requested department only
 * when *no* record carries a department id (single-scope backend response).
 */
function mapMembersForDepartment(records: Record<string, unknown>[], departmentId: string): OrgMember[] {
  const hasExplicitDepartment = records.some(
    (record) => pickString(record.departmentId, record.department_id) !== undefined,
  );
  const members = records
    .map((record) => mapMember(record, departmentId))
    .filter((member): member is OrgMember => member !== undefined);
  if (!hasExplicitDepartment) {
    return uniqueById(members);
  }
  return uniqueById(members.filter((member) => member.departmentId === departmentId));
}

async function fetchDepartmentMembers(orgId: string | undefined, departmentId: string): Promise<OrgMember[]> {
  const cacheKey = `${orgId ?? "*"}:${departmentId}`;
  const cached = membersByDepartmentCache.get(cacheKey);
  if (cached && isCacheFresh(cached.cachedAt)) {
    return cached.members;
  }
  const client = getIamAppSdkClient();
  const records = await fetchCursorPages((cursor) =>
    client.iam.departmentAssignments.list({
      pageSize: DIRECTORY_PAGE_SIZE,
      ...(cursor ? { cursor } : {}),
    }),
  );
  const scoped = orgId
    ? records.filter((record) => {
      const recordOrganizationId = pickString(record.organizationId, record.organization_id);
      return recordOrganizationId === undefined || recordOrganizationId === orgId;
    })
    : records;
  const members = mapMembersForDepartment(scoped, departmentId);
  membersByDepartmentCache.set(cacheKey, { members, cachedAt: Date.now() });
  return members;
}

async function fetchOrganizationMembers(orgId: string): Promise<OrgMember[]> {
  const cached = membersByOrganizationCache.get(orgId);
  if (cached && isCacheFresh(cached.cachedAt)) {
    return cached.members;
  }
  const client = getIamAppSdkClient();
  const records = await fetchCursorPages((cursor) =>
    client.iam.departmentAssignments.list({
      pageSize: DIRECTORY_PAGE_SIZE,
      ...(cursor ? { cursor } : {}),
    }),
  );
  const scoped = records.filter((record) => {
    const recordOrganizationId = pickString(record.organizationId, record.organization_id);
    return recordOrganizationId === undefined || recordOrganizationId === orgId;
  });
  const members = uniqueById(
    scoped.map((record) => mapMember(record, "")).filter((member): member is OrgMember => member !== undefined),
  );
  membersByOrganizationCache.set(orgId, { members, cachedAt: Date.now() });
  return members;
}

export const OrganizationService = {
  async getOrganizations(): Promise<Organization[]> {
    try {
      const client = getIamAppSdkClient();
      const records = await fetchCursorPages((cursor) =>
        client.iam.organizations.list({
          pageSize: DIRECTORY_PAGE_SIZE,
          ...(cursor ? { cursor } : {}),
        }),
      );
      return uniqueById(
        records.map(mapOrganization).filter((organization): organization is Organization => organization !== undefined),
      );
    } catch (error) {
      console.error("Unable to load organizations", error);
      return [];
    }
  },

  async getDepartments(orgId: string, parentId: string | null): Promise<OrgDepartment[]> {
    try {
      const departments = await fetchAllDepartments();
      const scoped = departments.filter(
        (department) => !orgId || department.organizationId === undefined || department.organizationId === orgId,
      );
      return scoped
        .filter((department) => (department.parentId ?? null) === (parentId ?? null))
        .sort((left, right) => left.name.localeCompare(right.name));
    } catch (error) {
      console.error("Unable to load departments", error);
      return [];
    }
  },

  async getMembers(orgId: string, departmentId: string): Promise<OrgMember[]> {
    try {
      return await fetchDepartmentMembers(orgId, departmentId);
    } catch (error) {
      console.error("Unable to load department members", error);
      return [];
    }
  },

  async getDepartmentPath(deptId: string): Promise<OrgDepartment[]> {
    try {
      const departments = await fetchAllDepartments();
      const byId = new Map(departments.map((department) => [department.id, department]));
      const path: OrgDepartment[] = [];
      let current = byId.get(deptId);
      while (current) {
        path.unshift(current);
        current = current.parentId ? byId.get(current.parentId) : undefined;
      }
      return path;
    } catch (error) {
      console.error("Unable to resolve department path", error);
      return [];
    }
  },

  async searchMembers(orgId: string, query: string): Promise<OrgMember[]> {
    const normalizedQuery = query.trim().toLowerCase();
    if (!normalizedQuery) {
      return [];
    }
    try {
      const members = await fetchOrganizationMembers(orgId);
      return members.filter(
        (member) =>
          member.name.toLowerCase().includes(normalizedQuery)
          || member.jobTitle.toLowerCase().includes(normalizedQuery),
      );
    } catch (error) {
      console.error("Unable to search organization members", error);
      return [];
    }
  },
};

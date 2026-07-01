import {
  getAppbaseAppSdkClientWithSession,
} from '@sdkwork/im-pc-core/sdk/appbaseAppSdkClient';
import {
  readAppSdkSessionTokens,
} from '@sdkwork/im-pc-core/sdk/session';
import type { User, UserPositionAssignment, UserRoleBinding } from '@sdkwork/im-pc-types';
import { createDefaultAvatar } from './DefaultAvatarService';

export interface OrgOrganization {
  appBoundaryEnabled?: boolean;
  dataBoundaryKind?: string;
  id: string;
  name: string;
  organizationId: string;
  organizationKind?: string;
  parentOrganizationId: string | null;
  status?: string;
  tenantBoundaryKind?: string;
  tenantId?: string;
  verificationStatus?: string;
  order: number;
}

export interface OrgOrganizationNode extends OrgOrganization {
  children: OrgOrganizationNode[];
}

interface OrgOrganizationMembership {
  membershipId?: string;
  organizationId: string;
  order: number;
  primary: boolean;
  status?: string;
  userId?: string;
}

export interface OrgDepartment {
  id: string;
  name: string;
  organizationId?: string;
  parentId: string | null;
  order: number;
}

export interface OrgDepartmentNode extends OrgDepartment {
  children: OrgDepartmentNode[];
}

export type OrganizationDirectoryTreeNodeKind = 'organization' | 'department';

export interface OrganizationDirectoryTreeNode {
  children: OrganizationDirectoryTreeNode[];
  department?: OrgDepartment;
  departmentId?: string;
  id: string;
  kind: OrganizationDirectoryTreeNodeKind;
  name: string;
  order: number;
  organization?: OrgOrganization;
  organizationId?: string;
  parentId: string | null;
}

export interface OrganizationDirectoryClient {
  iam?: {
    departmentAssignments?: {
      list?: (params?: Record<string, unknown>) => Promise<unknown>;
    };
    departments?: {
      list?: (params?: Record<string, unknown>) => Promise<unknown>;
      tree?: {
        retrieve?: (params?: Record<string, unknown>) => Promise<unknown>;
      };
    };
    organizationMemberships?: {
      list?: (params?: Record<string, unknown>) => Promise<unknown>;
    };
    organizations?: {
      list?: (params?: Record<string, unknown>) => Promise<unknown>;
      tree?: {
        retrieve?: (params?: Record<string, unknown>) => Promise<unknown>;
      };
    };
    positionAssignments?: {
      list?: (params?: Record<string, unknown>) => Promise<unknown>;
    };
    positions?: {
      list?: (params?: Record<string, unknown>) => Promise<unknown>;
    };
    roleBindings?: {
      list?: (params?: Record<string, unknown>) => Promise<unknown>;
    };
    users?: {
      current?: {
        retrieve?: () => Promise<unknown>;
      };
    };
  };
  listDepartmentAssignments?: (departmentId: string, params?: Record<string, unknown>) => Promise<unknown>;
  listDepartments?: (organizationId?: string, params?: Record<string, unknown>) => Promise<unknown>;
  listOrganizations?: (params?: Record<string, unknown>) => Promise<unknown>;
}

export interface OrganizationDirectoryPermission {
  adminCapabilityAvailable: boolean;
  canAssignRoles: boolean;
  canInviteMembers: boolean;
  canManageMembers: boolean;
  currentUserId?: string;
  organizationId: string;
  organizationMembershipIds: string[];
  reason: 'role_allowed' | 'not_authenticated' | 'no_active_membership' | 'role_denied' | 'missing_admin_capability';
  roleCodes: string[];
}

export interface OrganizationDirectoryAdminCapability {
  departmentAssignments?: {
    create?: (body: Record<string, unknown>) => Promise<unknown>;
    update?: (assignmentId: string, body: Record<string, unknown>) => Promise<unknown>;
  };
  organizationMemberships?: {
    create?: (body: Record<string, unknown>) => Promise<unknown>;
    update?: (membershipId: string, body: Record<string, unknown>) => Promise<unknown>;
  };
  positionAssignments?: {
    create?: (body: Record<string, unknown>) => Promise<unknown>;
    update?: (assignmentId: string, body: Record<string, unknown>) => Promise<unknown>;
  };
  roleBindings?: {
    create?: (body: Record<string, unknown>) => Promise<unknown>;
    delete?: (roleBindingId: string) => Promise<unknown>;
  };
  users?: {
    create?: (body: Record<string, unknown>) => Promise<unknown>;
    list?: (params?: Record<string, unknown>) => Promise<unknown>;
    retrieve?: (userId: string) => Promise<unknown>;
    update?: (userId: string, body: Record<string, unknown>) => Promise<unknown>;
  };
}

export interface AddOrganizationMemberInput {
  assignmentType?: string;
  departmentId?: string;
  membershipType?: string;
  organizationId: string;
  positionId?: string;
  positionName?: string;
  roleCodes?: string[];
  userId: string;
}

export interface InviteOrganizationMemberInput {
  assignmentType?: string;
  departmentId?: string;
  displayName?: string;
  email?: string;
  membershipType?: string;
  organizationId: string;
  phone?: string;
  positionId?: string;
  positionName?: string;
  roleCodes?: string[];
}

export interface OrganizationMemberManagementResult {
  departmentAssignmentId?: string;
  invitedUserId?: string;
  organizationId: string;
  organizationMembershipId: string;
  positionAssignmentIds: string[];
  roleBindingIds: string[];
  userId: string;
}

export interface OrganizationDirectoryService {
  addOrganizationMember(input: AddOrganizationMemberInput): Promise<OrganizationMemberManagementResult>;
  getDepartments(organizationId?: string): Promise<OrgDepartment[]>;
  getDepartmentTree(organizationId?: string): Promise<OrgDepartmentNode[]>;
  getCurrentUser(): Promise<User | null>;
  getOrganizationDirectoryTree(): Promise<OrganizationDirectoryTreeNode[]>;
  getOrganizationPermissions(organizationId: string): Promise<OrganizationDirectoryPermission>;
  getOrganizations(): Promise<OrgOrganization[]>;
  getOrganizationTree(): Promise<OrgOrganizationNode[]>;
  getUsersByDepartment(departmentId: string, organizationId?: string): Promise<User[]>;
  inviteOrganizationMember(input: InviteOrganizationMemberInput): Promise<OrganizationMemberManagementResult>;
}

export interface CreateOrganizationDirectoryServiceOptions {
  admin?: OrganizationDirectoryAdminCapability;
}

function normalizeString(value: unknown): string | undefined {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function pickString(...values: unknown[]): string | undefined {
  for (const value of values) {
    const normalized = normalizeString(value);
    if (normalized) {
      return normalized;
    }
  }
  return undefined;
}

function pickNumber(...values: unknown[]): number | undefined {
  for (const value of values) {
    if (typeof value === 'number' && Number.isFinite(value)) {
      return value;
    }
    if (typeof value === 'string' && value.trim().length > 0) {
      const parsed = Number(value);
      if (Number.isFinite(parsed)) {
        return parsed;
      }
    }
  }
  return undefined;
}

function pickBoolean(...values: unknown[]): boolean | undefined {
  for (const value of values) {
    if (typeof value === 'boolean') {
      return value;
    }
    if (typeof value === 'number' && Number.isFinite(value)) {
      return value !== 0;
    }
    if (typeof value === 'string' && value.trim().length > 0) {
      const normalized = value.trim().toLowerCase();
      if (['1', 'true', 'yes', 'y', 'primary'].includes(normalized)) {
        return true;
      }
      if (['0', 'false', 'no', 'n', 'secondary'].includes(normalized)) {
        return false;
      }
    }
  }
  return undefined;
}

function pickStringArray(...values: unknown[]): string[] {
  for (const value of values) {
    if (Array.isArray(value)) {
      return value
        .map((item) => normalizeString(item))
        .filter(Boolean) as string[];
    }
    const normalized = normalizeString(value);
    if (normalized) {
      return normalized
        .split(',')
        .map((item) => item.trim())
        .filter(Boolean);
    }
  }
  return [];
}

function toRecord(value: unknown): Record<string, unknown> {
  return value && typeof value === 'object' && !Array.isArray(value)
    ? value as Record<string, unknown>
    : {};
}

function unwrapEnvelope(value: unknown): unknown {
  const record = toRecord(value);
  if (!('code' in record) && !('data' in record)) {
    return value;
  }

  const code = record.code;
  const normalizedCode = code === undefined || code === null ? '2000' : String(code).trim();
  if (!['0', '200', '2000'].includes(normalizedCode)) {
    throw new Error(String(record.message || record.msg || 'Organization directory request failed'));
  }
  return record.data;
}

function extractRecordArray(value: unknown): Record<string, unknown>[] {
  const unwrapped = unwrapEnvelope(value);
  if (Array.isArray(unwrapped)) {
    return unwrapped.map(toRecord).filter((record) => Object.keys(record).length > 0);
  }

  const record = toRecord(unwrapped);
  for (const key of [
    'items',
    'records',
    'data',
    'list',
    'rows',
    'content',
    'nodes',
    'organizations',
    'departments',
    'memberships',
    'assignments',
    'departmentAssignments',
    'organizationMemberships',
    'positionAssignments',
    'roleBindings',
  ]) {
    const nested = record[key];
    if (Array.isArray(nested)) {
      return nested.map(toRecord).filter((item) => Object.keys(item).length > 0);
    }
  }
  return [];
}

function extractRecord(value: unknown): Record<string, unknown> {
  return toRecord(unwrapEnvelope(value));
}

function createSearchKey(value: string): string {
  return value
    .trim()
    .toLowerCase()
    .normalize('NFKD')
    .replace(/[^\da-z]+/gu, '');
}

function createAvatar(_seed: string): string {
  return createDefaultAvatar('user');
}

function mapOrganizationRecord(record: Record<string, unknown>): OrgOrganization | undefined {
  const organizationId = pickString(record.organizationId, record.organization_id, record.id);
  const name = pickString(record.name, record.displayName, record.display_name, record.organizationName, record.organization_name);
  if (!organizationId || !name) {
    return undefined;
  }

  return {
    appBoundaryEnabled: typeof record.appBoundaryEnabled === 'boolean' ? record.appBoundaryEnabled : undefined,
    dataBoundaryKind: pickString(record.dataBoundaryKind, record.data_boundary_kind),
    id: pickString(record.id) ?? organizationId,
    name,
    organizationId,
    organizationKind: pickString(record.organizationKind, record.organization_kind, record.type),
    parentOrganizationId: pickString(
      record.parentOrganizationId,
      record.parent_organization_id,
      record.parentId,
      record.parent_id,
    ) ?? null,
    status: pickString(record.status),
    tenantBoundaryKind: pickString(record.tenantBoundaryKind, record.tenant_boundary_kind),
    tenantId: pickString(record.tenantId, record.tenant_id),
    verificationStatus: pickString(record.verificationStatus, record.verification_status),
    order: pickNumber(record.order, record.sortOrder, record.sort_order, record.rank) ?? 0,
  };
}

function mapOrganizationTreeRecord(record: Record<string, unknown>): OrgOrganizationNode | undefined {
  const organization = mapOrganizationRecord(record);
  if (!organization) {
    return undefined;
  }

  const children = Array.isArray(record.children)
    ? record.children.map(toRecord).map(mapOrganizationTreeRecord).filter(Boolean) as OrgOrganizationNode[]
    : [];

  return {
    ...organization,
    children: sortOrganizationNodes(children),
  };
}

function mapOrganizationMembershipRecord(record: Record<string, unknown>): OrgOrganizationMembership | undefined {
  const organizationId = pickString(record.organizationId, record.organization_id);
  if (!organizationId) {
    return undefined;
  }

  return {
    membershipId: pickString(record.membershipId, record.membership_id, record.organizationMembershipId, record.organization_membership_id, record.id),
    organizationId,
    order: pickNumber(record.order, record.sortOrder, record.sort_order, record.rank) ?? 0,
    primary: pickBoolean(record.primary, record.isPrimary, record.is_primary, record.primaryMembership, record.primary_membership) ?? false,
    status: pickString(record.status),
    userId: pickString(record.userId, record.user_id),
  };
}

function mapDepartmentRecord(record: Record<string, unknown>): OrgDepartment | undefined {
  const id = pickString(record.departmentId, record.department_id, record.id, record.nodeId, record.node_id);
  const name = pickString(record.name, record.displayName, record.display_name, record.departmentName, record.department_name, record.title, record.label);
  if (!id || !name) {
    return undefined;
  }

  return {
    id,
    name,
    organizationId: pickString(record.organizationId, record.organization_id),
    parentId: pickString(
      record.parentDepartmentId,
      record.parent_department_id,
      record.parentId,
      record.parent_id,
    ) ?? null,
    order: pickNumber(record.order, record.sortOrder, record.sort_order, record.rank) ?? 0,
  };
}

function mapDepartmentTreeRecord(record: Record<string, unknown>): OrgDepartmentNode | undefined {
  const department = mapDepartmentRecord(record);
  if (!department) {
    return undefined;
  }

  const children = Array.isArray(record.children)
    ? record.children.map(toRecord).map(mapDepartmentTreeRecord).filter(Boolean) as OrgDepartmentNode[]
    : [];

  return {
    ...department,
    children: sortDepartmentNodes(children),
  };
}

function normalizeAssignmentStatus(value: unknown): User['status'] {
  const status = pickString(value)?.toLowerCase();
  if (status === 'active' || status === 'enabled' || status === 'online') {
    return 'online';
  }
  if (status === 'busy') {
    return 'busy';
  }
  if (status === 'away') {
    return 'away';
  }
  return 'offline';
}

function mapUserRecord(record: Record<string, unknown>): User | undefined {
  const userId = pickString(record.userId, record.user_id, record.id, record.accountId, record.account_id);
  if (!userId) {
    return undefined;
  }

  const name = pickString(record.displayName, record.display_name, record.name, record.nickname, record.username, userId) ?? userId;
  return {
    avatar: pickString(record.avatarUrl, record.avatar_url, record.avatar) ?? createAvatar(userId),
    ...(pickString(record.email) ? { email: pickString(record.email) } : {}),
    id: userId,
    name,
    ...(pickString(record.phone, record.mobile) ? { phone: pickString(record.phone, record.mobile) } : {}),
    py: createSearchKey(name),
    status: normalizeAssignmentStatus(record.status),
  };
}

function mapAssignmentRecord(record: Record<string, unknown>, departmentId: string): User | undefined {
  const userId = pickString(record.userId, record.user_id, record.personId, record.person_id, record.id);
  if (!userId) {
    return undefined;
  }

  const name = pickString(record.displayName, record.display_name, record.name, record.nickname, record.username, userId) ?? userId;
  return {
    assignmentType: pickString(record.assignmentType, record.assignment_type),
    id: userId,
    name,
    avatar: pickString(record.avatarUrl, record.avatar_url, record.avatar) ?? createAvatar(userId),
    status: normalizeAssignmentStatus(record.status),
    email: pickString(record.email),
    phone: pickString(record.phone),
    departmentAssignmentId: pickString(record.assignmentId, record.assignment_id, record.departmentAssignmentId, record.department_assignment_id),
    departmentId: pickString(record.departmentId, record.department_id) ?? departmentId,
    organizationId: pickString(record.organizationId, record.organization_id),
    organizationMembershipId: pickString(record.membershipId, record.membership_id, record.organizationMembershipId, record.organization_membership_id),
    position: pickString(record.positionName, record.position_name, record.position, record.jobTitle, record.job_title),
    roleCodes: pickStringArray(record.roleCodes, record.role_codes, record.roles),
    py: createSearchKey(name),
  };
}

function mapPositionAssignmentRecord(record: Record<string, unknown>): UserPositionAssignment | undefined {
  const positionAssignmentId = pickString(record.positionAssignmentId, record.position_assignment_id, record.id);
  if (!positionAssignmentId) {
    return undefined;
  }

  return {
    positionAssignmentId,
    positionId: pickString(record.positionId, record.position_id),
    positionName: pickString(record.positionName, record.position_name, record.name),
    status: pickString(record.status),
  };
}

function mapRoleBindingRecord(record: Record<string, unknown>): UserRoleBinding | undefined {
  const roleBindingId = pickString(record.roleBindingId, record.role_binding_id, record.id);
  const roleCode = pickString(record.roleCode, record.role_code, record.code);
  if (!roleBindingId || !roleCode) {
    return undefined;
  }

  return {
    roleBindingId,
    roleCode,
    scopeId: pickString(record.scopeId, record.scope_id),
    scopeKind: pickString(record.scopeKind, record.scope_kind),
    status: pickString(record.status),
  };
}

const ORGANIZATION_ADMIN_ROLE_CODES = new Set([
  'admin',
  'owner',
  'tenant.admin',
  'tenant.owner',
  'org.admin',
  'org.owner',
  'organization.admin',
  'organization.owner',
  'iam.admin',
  'iam.organization.admin',
  'super_admin',
  'super.admin',
]);

function sortOrganizationNodes(nodes: OrgOrganizationNode[]): OrgOrganizationNode[] {
  return [...nodes].sort((left, right) => left.order - right.order || left.name.localeCompare(right.name));
}

function sortDepartmentNodes(nodes: OrgDepartmentNode[]): OrgDepartmentNode[] {
  return [...nodes].sort((left, right) => left.order - right.order || left.name.localeCompare(right.name));
}

function sortDepartments(departments: OrgDepartment[]): OrgDepartment[] {
  return [...departments].sort((left, right) => {
    if (left.parentId === right.parentId) {
      return left.order - right.order || left.name.localeCompare(right.name);
    }
    return (left.parentId ?? '').localeCompare(right.parentId ?? '');
  });
}

function buildOrganizationTree(organizations: OrgOrganization[]): OrgOrganizationNode[] {
  const byId = new Map<string, OrgOrganizationNode>();
  for (const organization of organizations) {
    byId.set(organization.organizationId, {
      ...organization,
      children: [],
    });
  }

  const roots: OrgOrganizationNode[] = [];
  for (const node of byId.values()) {
    const parentId = node.parentOrganizationId;
    const parent = parentId ? byId.get(parentId) : undefined;
    if (parent) {
      parent.children.push(node);
    } else {
      roots.push(node);
    }
  }

  const sortRecursively = (nodes: OrgOrganizationNode[]): OrgOrganizationNode[] => sortOrganizationNodes(nodes).map((node) => ({
    ...node,
    children: sortRecursively(node.children),
  }));
  return sortRecursively(roots);
}

function buildDepartmentTree(departments: OrgDepartment[]): OrgDepartmentNode[] {
  const byId = new Map<string, OrgDepartmentNode>();
  for (const department of departments) {
    byId.set(department.id, {
      ...department,
      children: [],
    });
  }

  const roots: OrgDepartmentNode[] = [];
  for (const node of byId.values()) {
    const parent = node.parentId ? byId.get(node.parentId) : undefined;
    if (parent) {
      parent.children.push(node);
    } else {
      roots.push(node);
    }
  }

  const sortRecursively = (nodes: OrgDepartmentNode[]): OrgDepartmentNode[] => sortDepartmentNodes(nodes).map((node) => ({
    ...node,
    children: sortRecursively(node.children),
  }));
  return sortRecursively(roots);
}

function flattenOrganizationNodes(nodes: OrgOrganizationNode[]): OrgOrganizationNode[] {
  return nodes.flatMap((node) => [node, ...flattenOrganizationNodes(node.children)]);
}

function flattenDepartmentNodes(nodes: OrgDepartmentNode[]): OrgDepartmentNode[] {
  return nodes.flatMap((node) => [node, ...flattenDepartmentNodes(node.children)]);
}

function hasUnscopedDepartmentNodes(nodes: OrgDepartmentNode[]): boolean {
  return flattenDepartmentNodes(nodes).some((node) => !node.organizationId);
}

function sortOrganizationDirectoryNodes(nodes: OrganizationDirectoryTreeNode[]): OrganizationDirectoryTreeNode[] {
  const kindOrder = (node: OrganizationDirectoryTreeNode): number => (node.kind === 'organization' ? 0 : 1);
  return [...nodes].sort((left, right) => (
    left.order - right.order
    || kindOrder(left) - kindOrder(right)
    || left.name.localeCompare(right.name)
  ));
}

function toOrganizationValue(node: OrgOrganizationNode): OrgOrganization {
  const { children: _children, ...organization } = node;
  return organization;
}

function toDepartmentValue(node: OrgDepartmentNode): OrgDepartment {
  const { children: _children, ...department } = node;
  return department;
}

function withDepartmentOrganization(
  node: OrgDepartmentNode,
  fallbackOrganizationId?: string,
): OrgDepartmentNode {
  const organizationId = node.organizationId ?? fallbackOrganizationId;
  return {
    ...node,
    ...(organizationId ? { organizationId } : {}),
    children: node.children.map((child) => withDepartmentOrganization(child, organizationId)),
  };
}

function departmentDirectoryNode(node: OrgDepartmentNode): OrganizationDirectoryTreeNode {
  const departmentId = node.id;
  const organizationScope = node.organizationId ?? 'unscoped';
  return {
    children: sortOrganizationDirectoryNodes(node.children.map(departmentDirectoryNode)),
    department: toDepartmentValue(node),
    departmentId,
    id: `department:${organizationScope}:${departmentId}`,
    kind: 'department',
    name: node.name,
    order: node.order,
    organizationId: node.organizationId,
    parentId: node.parentId,
  };
}

function organizationDirectoryNode(
  node: OrgOrganizationNode,
  departmentRootsByOrganizationId: Map<string, OrgDepartmentNode[]>,
): OrganizationDirectoryTreeNode {
  const organizationId = node.organizationId;
  const organizationChildren = node.children.map((child) => organizationDirectoryNode(child, departmentRootsByOrganizationId));
  const departmentChildren = (departmentRootsByOrganizationId.get(organizationId) ?? []).map(departmentDirectoryNode);
  return {
    children: sortOrganizationDirectoryNodes([...organizationChildren, ...departmentChildren]),
    id: `organization:${organizationId}`,
    kind: 'organization',
    name: node.name,
    order: node.order,
    organization: toOrganizationValue(node),
    organizationId,
    parentId: node.parentOrganizationId,
  };
}

function buildOrganizationDirectoryTree(
  organizationTree: OrgOrganizationNode[],
  departmentTree: OrgDepartmentNode[],
): OrganizationDirectoryTreeNode[] {
  const organizations = flattenOrganizationNodes(organizationTree);
  const knownOrganizationIds = new Set(organizations.map((organization) => organization.organizationId));
  const departmentRootsByOrganizationId = new Map<string, OrgDepartmentNode[]>();
  const orphanDepartmentRoots: OrgDepartmentNode[] = [];

  for (const departmentRoot of departmentTree) {
    const departmentWithOrganization = withDepartmentOrganization(departmentRoot);
    const organizationId = departmentWithOrganization.organizationId;
    if (organizationId && knownOrganizationIds.has(organizationId)) {
      const roots = departmentRootsByOrganizationId.get(organizationId) ?? [];
      roots.push(departmentWithOrganization);
      departmentRootsByOrganizationId.set(organizationId, roots);
    } else {
      orphanDepartmentRoots.push(departmentWithOrganization);
    }
  }

  const organizationNodes = organizationTree.map((node) => organizationDirectoryNode(node, departmentRootsByOrganizationId));
  const orphanDepartmentNodes = orphanDepartmentRoots.map(departmentDirectoryNode);
  return sortOrganizationDirectoryNodes([...organizationNodes, ...orphanDepartmentNodes]);
}

function uniqueById<T extends { id: string }>(items: T[]): T[] {
  const byId = new Map<string, T>();
  for (const item of items) {
    byId.set(item.id, item);
  }
  return [...byId.values()];
}

function uniqueUsersByAssignment(users: User[]): User[] {
  const byKey = new Map<string, User>();
  for (const user of users) {
    byKey.set(user.departmentAssignmentId ?? user.id, user);
  }
  return [...byKey.values()];
}

function normalizeRoleCode(roleCode: string): string {
  return roleCode.trim().toLowerCase().replace(/[\s:_-]+/gu, '.');
}

function isOrganizationAdminRole(roleCode: string): boolean {
  const normalized = normalizeRoleCode(roleCode);
  return ORGANIZATION_ADMIN_ROLE_CODES.has(normalized)
    || normalized.endsWith('.admin')
    || normalized.endsWith('.owner');
}

function isActiveStatus(status?: string): boolean {
  return !status || ['active', 'enabled', 'acting'].includes(status.toLowerCase());
}

const DIRECTORY_BROWSE_PERMISSIONS = [
  'iam.organizations.read',
  'iam.departments.read',
] as const;

function readSessionPermissionScope(): string[] {
  const session = readAppSdkSessionTokens();
  return pickStringArray(
    session?.context?.permissionScope,
    toRecord(session?.context).permission_scope,
  );
}

function hasPermissionInScope(permissionScope: string[], permission: string): boolean {
  if (permissionScope.includes(permission)) {
    return true;
  }
  return permissionScope.some((scopeEntry) => {
    if (scopeEntry === '*' || scopeEntry === 'iam.*') {
      return permission.startsWith('iam.');
    }
    if (!scopeEntry.endsWith('.*')) {
      return false;
    }
    const prefix = scopeEntry.slice(0, -2);
    return permission === prefix || permission.startsWith(`${prefix}.`);
  });
}

function hasDirectoryBrowsePermission(permissionScope: string[] = readSessionPermissionScope()): boolean {
  return DIRECTORY_BROWSE_PERMISSIONS.some((permission) => hasPermissionInScope(permissionScope, permission));
}

function hasRoleBindingsReadPermission(permissionScope: string[] = readSessionPermissionScope()): boolean {
  return hasPermissionInScope(permissionScope, 'iam.role_bindings.read');
}

async function listRoleBindingsSafely(
  listRoleBindings: () => Promise<unknown>,
): Promise<UserRoleBinding[]> {
  if (!hasRoleBindingsReadPermission()) {
    return [];
  }
  const response = await safeDirectoryRequest(listRoleBindings);
  if (!response) {
    return [];
  }
  return extractRecordArray(response)
    .map(mapRoleBindingRecord)
    .filter(Boolean) as UserRoleBinding[];
}

function isDirectoryAccessError(error: unknown): boolean {
  const message = error instanceof Error ? error.message.toLowerCase() : String(error ?? '').toLowerCase();
  return message.includes('403')
    || message.includes('forbidden')
    || message.includes('401')
    || message.includes('unauthorized')
    || message.includes('permission')
    || message.includes('organization directory request failed');
}

async function safeDirectoryRequest<T>(request: () => Promise<T>): Promise<T | undefined> {
  try {
    return await request();
  } catch (error) {
    if (isDirectoryAccessError(error)) {
      return undefined;
    }
    throw error;
  }
}

function organizationFromMembership(
  membership: OrgOrganizationMembership,
  index: number,
): OrgOrganization {
  const organizationId = membership.organizationId;
  return {
    id: organizationId,
    name: organizationId,
    organizationId,
    parentOrganizationId: null,
    order: membership.order ?? index,
    status: membership.status,
  };
}

function pickCreatedId(record: Record<string, unknown>, ...keys: string[]): string | undefined {
  return pickString(...keys.map((key) => record[key]));
}

class SdkworkOrganizationDirectoryService implements OrganizationDirectoryService {
  private readonly explicitDepartmentOrganizationById = new Map<string, string>();

  constructor(
    private readonly getClient: () => OrganizationDirectoryClient = getAppbaseAppSdkClientWithSession as () => OrganizationDirectoryClient,
    private readonly options: CreateOrganizationDirectoryServiceOptions = {},
  ) {}

  private client(): OrganizationDirectoryClient {
    return this.getClient();
  }

  private admin(): OrganizationDirectoryAdminCapability | undefined {
    return this.options.admin;
  }

  private hasAdminMemberCapability(): boolean {
    return typeof this.admin()?.organizationMemberships?.create === 'function';
  }

  private hasAdminInviteCapability(): boolean {
    return this.hasAdminMemberCapability()
      && typeof this.admin()?.users?.create === 'function';
  }

  async getCurrentUser(): Promise<User | null> {
    const client = this.client();
    if (client.iam?.users?.current?.retrieve) {
      const user = mapUserRecord(extractRecord(await client.iam.users.current.retrieve()));
      if (user) {
        return user;
      }
    }

    const session = readAppSdkSessionTokens();
    const sessionUser = mapUserRecord(toRecord(session?.user));
    return sessionUser ?? null;
  }

  async getOrganizationPermissions(organizationId: string): Promise<OrganizationDirectoryPermission> {
    const resolvedOrganizationId = pickString(organizationId);
    if (!resolvedOrganizationId) {
      throw new Error('organizationId is required');
    }

    const currentUser = await this.getCurrentUser();
    if (!currentUser) {
      return {
        adminCapabilityAvailable: this.hasAdminMemberCapability(),
        canAssignRoles: false,
        canInviteMembers: false,
        canManageMembers: false,
        organizationId: resolvedOrganizationId,
        organizationMembershipIds: [],
        reason: 'not_authenticated',
        roleCodes: [],
      };
    }

    const memberships = await this.listActiveOrganizationMemberships(resolvedOrganizationId, currentUser.id);
    const membershipIds = memberships
      .map((membership) => membership.membershipId)
      .filter(Boolean) as string[];
    if (membershipIds.length === 0) {
      return {
        adminCapabilityAvailable: this.hasAdminMemberCapability(),
        canAssignRoles: false,
        canInviteMembers: false,
        canManageMembers: false,
        currentUserId: currentUser.id,
        organizationId: resolvedOrganizationId,
        organizationMembershipIds: [],
        reason: 'no_active_membership',
        roleCodes: [],
      };
    }

    const client = this.client();
    const roleCodes = new Set<string>();
    if (client.iam?.roleBindings?.list) {
      for (const membershipId of membershipIds) {
        const bindings = await listRoleBindingsSafely(() => client.iam.roleBindings.list({
          principalId: membershipId,
          scopeKind: 'organization',
          scopeId: resolvedOrganizationId,
        }));
        for (const binding of bindings) {
          if (isActiveStatus(binding.status)) {
            roleCodes.add(binding.roleCode);
          }
        }
      }
    }

    const sortedRoleCodes = [...roleCodes].sort((left, right) => left.localeCompare(right));
    const hasAdminRole = sortedRoleCodes.some(isOrganizationAdminRole);
    const adminCapabilityAvailable = this.hasAdminMemberCapability();
    const canManageMembers = hasAdminRole && adminCapabilityAvailable;
    return {
      adminCapabilityAvailable,
      canAssignRoles: hasAdminRole && typeof this.admin()?.roleBindings?.create === 'function',
      canInviteMembers: hasAdminRole && this.hasAdminInviteCapability(),
      canManageMembers,
      currentUserId: currentUser.id,
      organizationId: resolvedOrganizationId,
      organizationMembershipIds: membershipIds,
      reason: hasAdminRole
        ? (adminCapabilityAvailable ? 'role_allowed' : 'missing_admin_capability')
        : 'role_denied',
      roleCodes: sortedRoleCodes,
    };
  }

  async getOrganizations(): Promise<OrgOrganization[]> {
    const membershipOrganizations = await this.getOrganizationsFromMemberships();
    const listedOrganizations = await safeDirectoryRequest(() => this.fetchOrganizationsList());
    if (listedOrganizations && listedOrganizations.length > 0) {
      return listedOrganizations;
    }
    return membershipOrganizations;
  }

  async getOrganizationTree(): Promise<OrgOrganizationNode[]> {
    if (hasDirectoryBrowsePermission()) {
      const treeNodes = await safeDirectoryRequest(() => this.fetchOrganizationTree());
      if (treeNodes && treeNodes.length > 0) {
        return treeNodes;
      }
    }

    const organizations = await this.getOrganizations();
    if (organizations.length > 0) {
      return buildOrganizationTree(organizations);
    }
    return [];
  }

  async getDepartments(organizationId?: string): Promise<OrgDepartment[]> {
    const explicitOrganizationId = pickString(organizationId);
    const listedDepartments = await safeDirectoryRequest(() => this.fetchDepartments(explicitOrganizationId));
    if (listedDepartments) {
      return listedDepartments;
    }
    return [];
  }

  async getDepartmentTree(organizationId?: string): Promise<OrgDepartmentNode[]> {
    const explicitOrganizationId = pickString(organizationId);
    const params = {
      ...(explicitOrganizationId ? { organizationId: explicitOrganizationId } : {}),
    };

    if (hasDirectoryBrowsePermission()) {
      const treeNodes = await safeDirectoryRequest(async () => {
        const client = this.client();
        if (!client.iam?.departments?.tree?.retrieve) {
          return undefined;
        }
        const response = await client.iam.departments.tree.retrieve(params);
        const nodes = extractRecordArray(response)
          .map(mapDepartmentTreeRecord)
          .filter(Boolean) as OrgDepartmentNode[];
        if (nodes.length === 0) {
          return undefined;
        }
        this.registerDepartmentOrganizationScope(nodes, explicitOrganizationId);
        const scopedNodes = explicitOrganizationId
          ? nodes.map((node) => withDepartmentOrganization(node, explicitOrganizationId))
          : nodes;
        return sortDepartmentNodes(scopedNodes);
      });
      if (treeNodes && treeNodes.length > 0) {
        return treeNodes;
      }
    }

    return buildDepartmentTree(await this.getDepartments(explicitOrganizationId));
  }

  private async fetchOrganizationsList(): Promise<OrgOrganization[]> {
    const client = this.client();
    const response = client.iam?.organizations?.list
      ? await client.iam.organizations.list({})
      : await client.listOrganizations?.({});
    return uniqueById(
      extractRecordArray(response)
        .map(mapOrganizationRecord)
        .filter(Boolean) as OrgOrganization[],
    ).sort((left, right) => left.order - right.order || left.name.localeCompare(right.name));
  }

  private async fetchOrganizationTree(): Promise<OrgOrganizationNode[]> {
    const client = this.client();
    if (!client.iam?.organizations?.tree?.retrieve) {
      return [];
    }
    const response = await client.iam.organizations.tree.retrieve({});
    const nodes = extractRecordArray(response)
      .map(mapOrganizationTreeRecord)
      .filter(Boolean) as OrgOrganizationNode[];
    return nodes.length > 0 ? sortOrganizationNodes(nodes) : [];
  }

  private async fetchDepartments(organizationId?: string): Promise<OrgDepartment[]> {
    const explicitOrganizationId = pickString(organizationId);
    const params = {
      ...(explicitOrganizationId ? { organizationId: explicitOrganizationId } : {}),
    };
    const client = this.client();
    const response = client.iam?.departments?.list
      ? await client.iam.departments.list(params)
      : await client.listDepartments?.(explicitOrganizationId, params);
    const departments = extractRecordArray(response)
      .map(mapDepartmentRecord)
      .filter(Boolean) as OrgDepartment[];
    const scopedDepartments = explicitOrganizationId
      ? departments.map((department) => ({
          ...department,
          organizationId: department.organizationId ?? explicitOrganizationId,
        }))
      : departments;
    if (explicitOrganizationId) {
      for (const department of scopedDepartments) {
        if (department.organizationId) {
          this.explicitDepartmentOrganizationById.set(department.id, department.organizationId);
        }
      }
    }
    return sortDepartments(uniqueById(scopedDepartments));
  }

  private registerDepartmentOrganizationScope(
    nodes: OrgDepartmentNode[],
    explicitOrganizationId?: string,
  ): void {
    const register = (department: OrgDepartmentNode) => {
      if (explicitOrganizationId) {
        const departmentOrganizationId = department.organizationId ?? explicitOrganizationId;
        if (departmentOrganizationId) {
          this.explicitDepartmentOrganizationById.set(department.id, departmentOrganizationId);
        }
      }
      for (const child of department.children) {
        register(child);
      }
    };
    for (const node of nodes) {
      register(node);
    }
  }

  private async getOrganizationsFromMemberships(): Promise<OrgOrganization[]> {
    const memberships = await this.listCurrentUserOrganizationMemberships();
    if (memberships.length === 0) {
      return [];
    }
    return uniqueById(
      memberships
        .map((membership, index) => organizationFromMembership(membership, index))
        .filter(Boolean) as OrgOrganization[],
    ).sort((left, right) => left.order - right.order || left.name.localeCompare(right.name));
  }

  private async listCurrentUserOrganizationMemberships(): Promise<OrgOrganizationMembership[]> {
    const currentUser = await this.getCurrentUser();
    const userId = pickString(currentUser?.id);
    const client = this.client();
    if (!userId || !client.iam?.organizationMemberships?.list) {
      return [];
    }

    const memberships = await safeDirectoryRequest(async () => extractRecordArray(
      await client.iam.organizationMemberships.list({ userId }),
    ));
    if (!memberships) {
      return [];
    }

    return memberships
      .map(mapOrganizationMembershipRecord)
      .filter(Boolean)
      .filter((membership) => isActiveStatus(membership?.status)) as OrgOrganizationMembership[];
  }

  async getOrganizationDirectoryTree(): Promise<OrganizationDirectoryTreeNode[]> {
    const organizationTree = await this.getOrganizationTree();
    let departmentTree = await this.getDepartmentTree();
    const organizations = flattenOrganizationNodes(organizationTree);
    if (organizations.length > 1 && hasUnscopedDepartmentNodes(departmentTree)) {
      const scopedDepartmentTrees = await Promise.all(
        organizations.map(async (organization) => this.getDepartmentTree(organization.organizationId).catch(() => [])),
      );
      const scopedDepartmentTree = scopedDepartmentTrees.flat();
      if (scopedDepartmentTree.length > 0) {
        departmentTree = scopedDepartmentTree;
      }
    }
    return buildOrganizationDirectoryTree(organizationTree, departmentTree);
  }

  async getUsersByDepartment(departmentId: string, organizationId?: string): Promise<User[]> {
    const normalizedDepartmentId = departmentId.trim();
    if (!normalizedDepartmentId) {
      return [];
    }

    const explicitOrganizationId = pickString(organizationId) ?? this.explicitDepartmentOrganizationById.get(normalizedDepartmentId);
    const params = {
      departmentId: normalizedDepartmentId,
      ...(explicitOrganizationId ? { organizationId: explicitOrganizationId } : {}),
    };
    const client = this.client();
    const response = client.iam?.departmentAssignments?.list
      ? await client.iam.departmentAssignments.list(params)
      : await client.listDepartmentAssignments?.(normalizedDepartmentId, params);
    const assignmentUsers = uniqueUsersByAssignment(
      extractRecordArray(response)
        .map((record) => mapAssignmentRecord(record, normalizedDepartmentId))
        .filter(Boolean) as User[],
    );

    const users = await Promise.all(assignmentUsers.map(async (user) => this.enrichAssignmentUser(user)));
    return users.sort((left, right) => left.name.localeCompare(right.name));
  }

  async addOrganizationMember(input: AddOrganizationMemberInput): Promise<OrganizationMemberManagementResult> {
    return this.createOrganizationMember(input);
  }

  async inviteOrganizationMember(input: InviteOrganizationMemberInput): Promise<OrganizationMemberManagementResult> {
    const organizationId = pickString(input.organizationId);
    if (!organizationId) {
      throw new Error('organizationId is required');
    }
    const email = pickString(input.email);
    const phone = pickString(input.phone);
    if (!email && !phone) {
      throw new Error('email or phone is required to invite an organization member');
    }
    await this.assertCanManageMembers(organizationId, { invite: true });

    const admin = this.admin();
    if (!admin?.users?.create) {
      throw new Error('organization member invitation capability is not available');
    }

    const userRecord = extractRecord(await admin.users.create({
      ...(email ? { email } : {}),
      ...(phone ? { phone } : {}),
      ...(pickString(input.displayName) ? { displayName: pickString(input.displayName) } : {}),
      status: 'invited',
    }));
    const userId = pickCreatedId(userRecord, 'userId', 'user_id', 'id');
    if (!userId) {
      throw new Error('organization member invitation did not return a user id');
    }

    const result = await this.createOrganizationMember({
      assignmentType: input.assignmentType,
      departmentId: input.departmentId,
      membershipType: input.membershipType,
      organizationId,
      positionId: input.positionId,
      positionName: input.positionName,
      roleCodes: input.roleCodes,
      userId,
    }, { skipPermissionCheck: true });
    return {
      ...result,
      invitedUserId: userId,
    };
  }

  private async listActiveOrganizationMemberships(organizationId: string, userId: string): Promise<OrgOrganizationMembership[]> {
    const client = this.client();
    if (!client.iam?.organizationMemberships?.list) {
      return [];
    }

    const memberships = extractRecordArray(await client.iam.organizationMemberships.list({
      organizationId,
      userId,
    }))
      .map(mapOrganizationMembershipRecord)
      .filter(Boolean) as OrgOrganizationMembership[];
    return memberships.filter((membership) => (
      membership.organizationId === organizationId
      && isActiveStatus(membership.status)
    ));
  }

  private async assertCanManageMembers(
    organizationId: string,
    options: { invite?: boolean } = {},
  ): Promise<void> {
    const permission = await this.getOrganizationPermissions(organizationId);
    if (!permission.roleCodes.some(isOrganizationAdminRole)) {
      throw new Error('current user is not allowed to manage organization members');
    }
    if (!permission.adminCapabilityAvailable) {
      throw new Error('organization member management capability is not available');
    }
    if (options.invite && !permission.canInviteMembers) {
      throw new Error('organization member invitation capability is not available');
    }
  }

  private async createOrganizationMember(
    input: AddOrganizationMemberInput,
    options: { skipPermissionCheck?: boolean } = {},
  ): Promise<OrganizationMemberManagementResult> {
    const organizationId = pickString(input.organizationId);
    const userId = pickString(input.userId);
    if (!organizationId) {
      throw new Error('organizationId is required');
    }
    if (!userId) {
      throw new Error('userId is required');
    }
    if (!options.skipPermissionCheck) {
      await this.assertCanManageMembers(organizationId);
    }

    const admin = this.admin();
    if (!admin?.organizationMemberships?.create) {
      throw new Error('organization member management capability is not available');
    }

    const membershipRecord = extractRecord(await admin.organizationMemberships.create({
      organizationId,
      userId,
      membershipType: pickString(input.membershipType) ?? 'employee',
      status: 'active',
    }));
    const organizationMembershipId = pickCreatedId(
      membershipRecord,
      'organizationMembershipId',
      'organization_membership_id',
      'membershipId',
      'membership_id',
      'id',
    );
    if (!organizationMembershipId) {
      throw new Error('organization membership creation did not return a membership id');
    }

    const departmentAssignmentId = await this.createDepartmentAssignment({
      assignmentType: input.assignmentType,
      departmentId: input.departmentId,
      organizationId,
      organizationMembershipId,
      userId,
    });
    const positionAssignmentIds = await this.createPositionAssignments({
      departmentAssignmentId,
      departmentId: input.departmentId,
      organizationId,
      positionId: input.positionId,
      positionName: input.positionName,
      userId,
    });
    const roleBindingIds = await this.createMemberRoleBindings({
      departmentAssignmentId,
      organizationId,
      organizationMembershipId,
      roleCodes: input.roleCodes ?? [],
    });

    return {
      ...(departmentAssignmentId ? { departmentAssignmentId } : {}),
      organizationId,
      organizationMembershipId,
      positionAssignmentIds,
      roleBindingIds,
      userId,
    };
  }

  private async createDepartmentAssignment(input: {
    assignmentType?: string;
    departmentId?: string;
    organizationId: string;
    organizationMembershipId: string;
    userId: string;
  }): Promise<string | undefined> {
    const departmentId = pickString(input.departmentId);
    if (!departmentId) {
      return undefined;
    }
    const admin = this.admin();
    if (!admin?.departmentAssignments?.create) {
      throw new Error('organization department assignment capability is not available');
    }

    const assignmentRecord = extractRecord(await admin.departmentAssignments.create({
      assignmentType: pickString(input.assignmentType) ?? 'primary',
      departmentId,
      organizationId: input.organizationId,
      organizationMembershipId: input.organizationMembershipId,
      status: 'active',
      userId: input.userId,
    }));
    const departmentAssignmentId = pickCreatedId(
      assignmentRecord,
      'departmentAssignmentId',
      'department_assignment_id',
      'assignmentId',
      'assignment_id',
      'id',
    );
    if (!departmentAssignmentId) {
      throw new Error('department assignment creation did not return an assignment id');
    }
    return departmentAssignmentId;
  }

  private async createPositionAssignments(input: {
    departmentAssignmentId?: string;
    departmentId?: string;
    organizationId: string;
    positionId?: string;
    positionName?: string;
    userId: string;
  }): Promise<string[]> {
    const positionId = pickString(input.positionId);
    const positionName = pickString(input.positionName);
    if (!positionId && !positionName) {
      return [];
    }
    if (!input.departmentAssignmentId) {
      throw new Error('department assignment is required before assigning a position');
    }
    const admin = this.admin();
    if (!admin?.positionAssignments?.create) {
      throw new Error('organization position assignment capability is not available');
    }

    const positionAssignmentRecord = extractRecord(await admin.positionAssignments.create({
      departmentAssignmentId: input.departmentAssignmentId,
      ...(pickString(input.departmentId) ? { departmentId: pickString(input.departmentId) } : {}),
      organizationId: input.organizationId,
      ...(positionId ? { positionId } : {}),
      ...(positionName ? { positionName } : {}),
      status: 'active',
      userId: input.userId,
    }));
    const positionAssignmentId = pickCreatedId(
      positionAssignmentRecord,
      'positionAssignmentId',
      'position_assignment_id',
      'assignmentId',
      'assignment_id',
      'id',
    );
    if (!positionAssignmentId) {
      throw new Error('position assignment creation did not return an assignment id');
    }
    return [positionAssignmentId];
  }

  private async createMemberRoleBindings(input: {
    departmentAssignmentId?: string;
    organizationId: string;
    organizationMembershipId: string;
    roleCodes: string[];
  }): Promise<string[]> {
    const roleCodes = [...new Set(input.roleCodes.map((roleCode) => pickString(roleCode)).filter(Boolean) as string[])];
    if (roleCodes.length === 0) {
      return [];
    }
    const admin = this.admin();
    if (!admin?.roleBindings?.create) {
      throw new Error('organization role binding capability is not available');
    }

    const roleBindingIds: string[] = [];
    for (const roleCode of roleCodes) {
      const departmentScoped = Boolean(
        input.departmentAssignmentId
        && /^(department|dept)\./u.test(normalizeRoleCode(roleCode)),
      );
      const roleBindingRecord = extractRecord(await admin.roleBindings.create({
        principalKind: departmentScoped ? 'department_assignment' : 'organization_membership',
        principalId: departmentScoped ? input.departmentAssignmentId : input.organizationMembershipId,
        roleCode,
        scopeKind: departmentScoped ? 'department_assignment' : 'organization',
        scopeId: departmentScoped ? input.departmentAssignmentId : input.organizationId,
        status: 'active',
      }));
      const roleBindingId = pickCreatedId(roleBindingRecord, 'roleBindingId', 'role_binding_id', 'id');
      if (roleBindingId) {
        roleBindingIds.push(roleBindingId);
      }
    }
    return roleBindingIds;
  }

  private async enrichAssignmentUser(user: User): Promise<User> {
    if (!user.departmentAssignmentId) {
      return user;
    }

    const client = this.client();
    const positionAssignments = client.iam?.positionAssignments?.list
      ? await client.iam.positionAssignments.list({
          departmentAssignmentId: user.departmentAssignmentId,
          userId: user.id,
        }).then((response) => extractRecordArray(response)
          .map(mapPositionAssignmentRecord)
          .filter(Boolean) as UserPositionAssignment[])
      : [];
    const activePositionAssignments = positionAssignments.filter((assignment) => (
      !assignment.status || ['active', 'acting'].includes(assignment.status.toLowerCase())
    ));

    const roleBindings = client.iam?.roleBindings?.list
      ? await listRoleBindingsSafely(() => client.iam.roleBindings.list({
          scopeKind: 'department_assignment',
          scopeId: user.departmentAssignmentId,
        }))
      : [];
    const activeRoleBindings = roleBindings.filter((binding) => (
      !binding.status || binding.status.toLowerCase() === 'active'
    ));
    const roleCodes = [...new Set([
      ...activeRoleBindings.map((binding) => binding.roleCode),
      ...(user.roleCodes ?? []),
    ])].sort((left, right) => left.localeCompare(right));
    const primaryPosition = activePositionAssignments.find((assignment) => assignment.positionName)?.positionName;

    return {
      ...user,
      ...(primaryPosition ? { position: primaryPosition } : {}),
      ...(activePositionAssignments.length > 0 ? { positionAssignments: activePositionAssignments } : {}),
      ...(activeRoleBindings.length > 0 ? { roleBindings: activeRoleBindings } : {}),
      ...(roleCodes.length > 0 ? { roleCodes } : {}),
    };
  }
}

export function createSdkworkOrganizationDirectoryService(
  getClient?: () => OrganizationDirectoryClient,
  options?: CreateOrganizationDirectoryServiceOptions,
): OrganizationDirectoryService {
  return new SdkworkOrganizationDirectoryService(getClient, options);
}

export const organizationDirectoryService = createSdkworkOrganizationDirectoryService();

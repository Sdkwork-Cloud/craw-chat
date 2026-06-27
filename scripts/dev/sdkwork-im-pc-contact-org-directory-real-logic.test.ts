import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkContactService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ContactService';
import {
  createSdkworkOrganizationDirectoryService,
  type OrganizationDirectoryClient,
} from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/OrganizationDirectoryService';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

const fakeImClient = {
  social: {
    contacts: {
      async list() {
        return { items: [], hasMore: false };
      },
    },
  },
} as unknown as ImSdkClient;

function read(relativePath: string): string {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

function assertNoTenantParam(value: Record<string, unknown> | undefined, message: string): void {
  assert.equal(value?.tenantId, undefined, message);
}

function assertNoOrganizationParam(value: Record<string, unknown> | undefined, message: string): void {
  assert.equal(value?.organizationId, undefined, message);
}

interface SimplifiedDirectoryNode {
  children: SimplifiedDirectoryNode[];
  departmentId?: string;
  kind: string;
  name: string;
  organizationId?: string;
}

function simplifyDirectoryNode(node: {
  children: Array<{
    children: unknown[];
    departmentId?: string;
    kind: string;
    name: string;
    organizationId?: string;
  }>;
  departmentId?: string;
  kind: string;
  name: string;
  organizationId?: string;
}): SimplifiedDirectoryNode {
  return {
    children: node.children.map((child) => simplifyDirectoryNode(child as Parameters<typeof simplifyDirectoryNode>[0])),
    ...(node.departmentId ? { departmentId: node.departmentId } : {}),
    kind: node.kind,
    name: node.name,
    ...(node.organizationId ? { organizationId: node.organizationId } : {}),
  };
}

function collectDirectoryNodeIds(nodes: Array<{ children: unknown[]; id: string }>): string[] {
  return nodes.flatMap((node) => [
    node.id,
    ...collectDirectoryNodeIds(node.children as Array<{ children: unknown[]; id: string }>),
  ]);
}

async function main(): Promise<void> {
  const organizationCalls: string[] = [];
  const organizationClient = {
    iam: {
      organizations: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.organizations.list params');
          organizationCalls.push(`iam.organizations.list:${params?.tenantId ?? ''}`);
          return [
            {
              organizationId: '300002',
              tenantId: '100001',
              name: 'SDKWork Group',
              parentOrganizationId: null,
              organizationKind: 'group',
              tenantBoundaryKind: 'root_tenant',
              verificationStatus: 'verified',
              appBoundaryEnabled: true,
              dataBoundaryKind: 'tenant_shared',
              order: 0,
            },
            {
              organizationId: '300001',
              tenantId: '100001',
              name: 'SDKWork Cloud Company',
              parentOrganizationId: '300002',
              organizationKind: 'company',
              tenantBoundaryKind: 'sub_tenant',
              verificationStatus: 'verified',
              appBoundaryEnabled: true,
              dataBoundaryKind: 'organization_isolated',
              order: 10,
            },
          ];
        },
      },
      departments: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.departments.list params');
          assertNoOrganizationParam(params, 'current organization context must not be sent as iam.departments.list params');
          organizationCalls.push(`iam.departments.list:${params?.organizationId ?? ''}`);
          return [
            {
              departmentId: 'dept-root',
              organizationId: '300001',
              name: 'Company Headquarters',
              parentDepartmentId: null,
              departmentKind: 'business_unit',
              order: 0,
            },
            {
              departmentId: 'dept-rd',
              organizationId: '300001',
              name: 'Research',
              parentDepartmentId: 'dept-root',
              departmentKind: 'department',
              order: 20,
            },
          ];
        },
      },
      departmentAssignments: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.departmentAssignments.list params');
          assertNoOrganizationParam(params, 'current organization context must not be sent as iam.departmentAssignments.list params');
          organizationCalls.push(`iam.departmentAssignments.list:${params?.departmentId ?? ''}`);
          return params?.departmentId === 'dept-rd'
            ? [
                {
                  assignmentId: 'assign-alice-rd',
                  membershipId: 'membership-alice-company',
                  organizationId: '300001',
                  departmentId: 'dept-rd',
                  userId: 'u_alice',
                  displayName: 'Alice',
                  avatarUrl: 'https://example.com/alice.png',
                  email: 'alice@example.com',
                  phone: '13800000001',
                  positionName: 'Engineer',
                  roleCodes: ['org.member', 'department.engineer'],
                  assignmentType: 'primary',
                  status: 'active',
                },
              ]
            : [];
        },
      },
      organizationMemberships: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.organizationMemberships.list params');
          organizationCalls.push(`iam.organizationMemberships.list:${params?.organizationId ?? ''}`);
          return [];
        },
      },
      positions: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.positions.list params');
          organizationCalls.push(`iam.positions.list:${params?.organizationId ?? ''}`);
          return [];
        },
      },
      positionAssignments: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.positionAssignments.list params');
          organizationCalls.push(`iam.positionAssignments.list:${params?.departmentAssignmentId ?? ''}`);
          return [];
        },
      },
      roleBindings: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.roleBindings.list params');
          organizationCalls.push(`iam.roleBindings.list:${params?.scopeId ?? ''}`);
          return [];
        },
      },
    },
    async listOrganizations(params) {
      assertNoTenantParam(params, 'current tenant context must not be sent through compat listOrganizations params');
      organizationCalls.push(`compat.listOrganizations:${params?.tenantId ?? ''}`);
      return [
        {
          organizationId: '300001',
          tenantId: '100001',
          name: 'SDKWork Cloud Company',
          parentOrganizationId: '300002',
        },
      ];
    },
    async listDepartments(organizationId) {
      organizationCalls.push(`compat.listDepartments:${organizationId}`);
      return [];
    },
    async listDepartmentAssignments(departmentId) {
      organizationCalls.push(`compat.listDepartmentAssignments:${departmentId}`);
      return [];
    },
  } satisfies OrganizationDirectoryClient;

  const forbiddenPortalAppClient = {
    portal: {
      home: {
        async retrieve() {
          throw new Error('portal.home.retrieve must not back contact organization directory');
        },
      },
    },
  };

  const organizationDirectoryService = createSdkworkOrganizationDirectoryService(() => organizationClient);
  const directoryBackedService = createSdkworkContactService(
    () => fakeImClient,
    () => forbiddenPortalAppClient,
    () => organizationDirectoryService,
  );

  assert.deepEqual(
    await directoryBackedService.getDepartments(),
    [
      { id: 'dept-root', name: 'Company Headquarters', organizationId: '300001', parentId: null, order: 0 },
      { id: 'dept-rd', name: 'Research', organizationId: '300001', parentId: 'dept-root', order: 20 },
    ],
    'contact org directory must map departments from the independent Organization/Department directory client',
  );
  assert.deepEqual(
    await directoryBackedService.getUsersByDepartment('dept-rd'),
    [
      {
        assignmentType: 'primary',
        avatar: 'https://example.com/alice.png',
        departmentAssignmentId: 'assign-alice-rd',
        departmentId: 'dept-rd',
        email: 'alice@example.com',
        id: 'u_alice',
        name: 'Alice',
        organizationId: '300001',
        organizationMembershipId: 'membership-alice-company',
        phone: '13800000001',
        position: 'Engineer',
        py: 'alice',
        roleCodes: ['department.engineer', 'org.member'],
        status: 'online',
      },
    ],
    'contact org directory users must come from department assignments instead of IM friendship contacts',
  );
  assert.deepEqual(
    organizationCalls,
    [
      'iam.departments.list:',
      'iam.departmentAssignments.list:dept-rd',
      'iam.positionAssignments.list:assign-alice-rd',
      'iam.roleBindings.list:assign-alice-rd',
    ],
    'contact org directory must read through the independent organization directory client',
  );

  const productDirectoryCalls: string[] = [];
  const productDirectoryService = createSdkworkOrganizationDirectoryService(() => ({
    iam: {
      organizations: {
        async list() {
          productDirectoryCalls.push('iam.organizations.list');
          return [];
        },
        tree: {
          async retrieve(params) {
            assertNoTenantParam(params, 'current tenant context must not be sent as iam.organizations.tree.retrieve params');
            productDirectoryCalls.push(`iam.organizations.tree.retrieve:${params?.tenantId ?? ''}`);
            return {
              items: [
                {
                  organizationId: '300002',
                  tenantId: '100001',
                  name: 'SDKWork Group',
                  parentOrganizationId: null,
                  organizationKind: 'group',
                  tenantBoundaryKind: 'root_tenant',
                  verificationStatus: 'verified',
                  status: 'active',
                  order: 0,
                  children: [
                    {
                      organizationId: '300001',
                      tenantId: '100001',
                      name: 'SDKWork Cloud Company',
                      parentOrganizationId: '300002',
                      organizationKind: 'company',
                      tenantBoundaryKind: 'operating_subject',
                      verificationStatus: 'verified',
                      dataBoundaryKind: 'organization_isolated',
                      appBoundaryEnabled: true,
                      status: 'active',
                      order: 10,
                      children: [],
                    },
                  ],
                },
              ],
            };
          },
        },
      },
      departments: {
        async list() {
          productDirectoryCalls.push('iam.departments.list');
          return [];
        },
        tree: {
          async retrieve(params) {
            assertNoTenantParam(params, 'current tenant context must not be sent as iam.departments.tree.retrieve params');
            productDirectoryCalls.push(`iam.departments.tree.retrieve:${params?.organizationId ?? ''}`);
            return {
              items: [
                {
                  departmentId: 'dept-root',
                  tenantId: '100001',
                  organizationId: '300001',
                  name: 'Company Headquarters',
                  parentDepartmentId: null,
                  departmentKind: 'headquarters',
                  status: 'active',
                  order: 0,
                  children: [
                    {
                      departmentId: 'dept-rd',
                      tenantId: '100001',
                      organizationId: '300001',
                      name: 'Research',
                      parentDepartmentId: 'dept-root',
                      departmentKind: 'department',
                      status: 'active',
                      order: 20,
                      children: [],
                    },
                  ],
                },
              ],
            };
          },
        },
      },
      departmentAssignments: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.departmentAssignments.list params');
          productDirectoryCalls.push(`iam.departmentAssignments.list:${params?.organizationId ?? ''}:${params?.departmentId ?? ''}`);
          return [
            {
              assignmentId: 'assign-alice-rd',
              membershipId: 'membership-alice-company',
              organizationId: '300001',
              departmentId: 'dept-rd',
              userId: 'u_alice',
              displayName: 'Alice',
              email: 'alice@example.com',
              phone: '13800000001',
              avatarUrl: 'https://example.com/alice.png',
              assignmentType: 'primary',
              status: 'active',
              positionId: 'pos-engineer',
              positionName: 'Engineer',
              roleCodes: ['org.member'],
            },
          ];
        },
      },
      positionAssignments: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.positionAssignments.list params');
          productDirectoryCalls.push(`iam.positionAssignments.list:${params?.departmentAssignmentId ?? ''}`);
          return [
            {
              positionAssignmentId: 'pos-assign-alice-principal',
              departmentAssignmentId: 'assign-alice-rd',
              tenantId: '100001',
              organizationId: '300001',
              departmentId: 'dept-rd',
              userId: 'u_alice',
              positionId: 'pos-engineer',
              positionName: 'Principal Engineer',
              status: 'active',
            },
          ];
        },
      },
      roleBindings: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.roleBindings.list params');
          productDirectoryCalls.push(`iam.roleBindings.list:${params?.scopeKind ?? ''}:${params?.scopeId ?? ''}:${params?.principalId ?? ''}`);
          return [
            {
              roleBindingId: 'rb-alice-rd-engineer',
              tenantId: '100001',
              roleCode: 'department.engineer',
              principalKind: 'department_assignment',
              principalId: 'assign-alice-rd',
              scopeKind: 'department_assignment',
              scopeId: 'assign-alice-rd',
              status: 'active',
            },
          ];
        },
      },
    },
  }) satisfies OrganizationDirectoryClient);

  assert.deepEqual(
    await productDirectoryService.getOrganizationTree(),
    [
      {
        appBoundaryEnabled: undefined,
        children: [
          {
            appBoundaryEnabled: true,
            children: [],
            dataBoundaryKind: 'organization_isolated',
            id: '300001',
            name: 'SDKWork Cloud Company',
            order: 10,
            organizationId: '300001',
            organizationKind: 'company',
            parentOrganizationId: '300002',
            status: 'active',
            tenantBoundaryKind: 'operating_subject',
            tenantId: '100001',
            verificationStatus: 'verified',
          },
        ],
        dataBoundaryKind: undefined,
        id: '300002',
        name: 'SDKWork Group',
        order: 0,
        organizationId: '300002',
        organizationKind: 'group',
        parentOrganizationId: null,
        status: 'active',
        tenantBoundaryKind: 'root_tenant',
        tenantId: '100001',
        verificationStatus: 'verified',
      },
    ],
    'contact org directory must expose the organization hierarchy as organizations, not departments folded into iam_organizations',
  );
  assert.deepEqual(
    await productDirectoryService.getDepartmentTree('300001'),
    [
      {
        children: [
          {
            children: [],
            id: 'dept-rd',
            name: 'Research',
            order: 20,
            organizationId: '300001',
            parentId: 'dept-root',
          },
        ],
        id: 'dept-root',
        name: 'Company Headquarters',
        order: 0,
        organizationId: '300001',
        parentId: null,
      },
    ],
    'contact org directory must expose departments through /departments hierarchy independent of organization hierarchy',
  );
  assert.deepEqual(
    (await productDirectoryService.getOrganizationDirectoryTree()).map(simplifyDirectoryNode),
    [
      {
        children: [
          {
            children: [
              {
                children: [
                  {
                    children: [],
                    departmentId: 'dept-rd',
                    kind: 'department',
                    name: 'Research',
                    organizationId: '300001',
                  },
                ],
                departmentId: 'dept-root',
                kind: 'department',
                name: 'Company Headquarters',
                organizationId: '300001',
              },
            ],
            kind: 'organization',
            name: 'SDKWork Cloud Company',
            organizationId: '300001',
          },
        ],
        kind: 'organization',
        name: 'SDKWork Group',
        organizationId: '300002',
      },
    ],
    'contact organization directory must merge organizations and departments into one address-book tree',
  );
  assert.deepEqual(
    await productDirectoryService.getUsersByDepartment('dept-rd'),
    [
      {
        assignmentType: 'primary',
        avatar: 'https://example.com/alice.png',
        departmentAssignmentId: 'assign-alice-rd',
        departmentId: 'dept-rd',
        email: 'alice@example.com',
        id: 'u_alice',
        name: 'Alice',
        organizationId: '300001',
        organizationMembershipId: 'membership-alice-company',
        phone: '13800000001',
        position: 'Principal Engineer',
        positionAssignments: [
          {
            positionAssignmentId: 'pos-assign-alice-principal',
            positionId: 'pos-engineer',
            positionName: 'Principal Engineer',
            status: 'active',
          },
        ],
        py: 'alice',
        roleBindings: [
          {
            roleBindingId: 'rb-alice-rd-engineer',
            roleCode: 'department.engineer',
            scopeId: 'assign-alice-rd',
            scopeKind: 'department_assignment',
            status: 'active',
          },
        ],
        roleCodes: ['department.engineer', 'org.member'],
        status: 'online',
      },
    ],
    'contact org directory members must carry organization membership, department assignment, position assignment, and scoped role binding context',
  );
  assert.deepEqual(
    productDirectoryCalls,
    [
      'iam.organizations.tree.retrieve:',
      'iam.departments.tree.retrieve:org-company',
      'iam.organizations.tree.retrieve:',
      'iam.departments.tree.retrieve:',
      'iam.departmentAssignments.list:org-company:dept-rd',
      'iam.positionAssignments.list:assign-alice-rd',
      'iam.roleBindings.list:department_assignment:assign-alice-rd:',
    ],
    'contact org directory product view must use organization tree, department tree, position assignment, and role binding SDK APIs',
  );

  const duplicateDepartmentIdDirectoryService = createSdkworkOrganizationDirectoryService(() => ({
    iam: {
      organizations: {
        tree: {
          async retrieve() {
            return {
              items: [
                {
                  organizationId: '300001',
                  name: 'Organization A',
                  parentOrganizationId: null,
                  order: 0,
                  children: [],
                },
                {
                  organizationId: '300002',
                  name: 'Organization B',
                  parentOrganizationId: null,
                  order: 10,
                  children: [],
                },
              ],
            };
          },
        },
      },
      departments: {
        tree: {
          async retrieve() {
            return {
              items: [
                {
                  departmentId: 'dept-root',
                  organizationId: '300001',
                  name: 'Headquarters',
                  parentDepartmentId: null,
                  order: 0,
                  children: [],
                },
                {
                  departmentId: 'dept-root',
                  organizationId: '300002',
                  name: 'Headquarters',
                  parentDepartmentId: null,
                  order: 0,
                  children: [],
                },
              ],
            };
          },
        },
      },
      departmentAssignments: {
        async list(params) {
          duplicateDepartmentIdCalls.push(`iam.departmentAssignments.list:${params?.organizationId ?? ''}:${params?.departmentId ?? ''}`);
          return [];
        },
      },
    },
  }) satisfies OrganizationDirectoryClient);
  const duplicateDepartmentIdCalls: string[] = [];
  const duplicateDepartmentIdNodeIds = collectDirectoryNodeIds(
    await duplicateDepartmentIdDirectoryService.getOrganizationDirectoryTree(),
  );
  assert.equal(
    new Set(duplicateDepartmentIdNodeIds).size,
    duplicateDepartmentIdNodeIds.length,
    'organization directory tree node ids must stay unique when different organizations reuse department ids',
  );
  assert.deepEqual(
    await duplicateDepartmentIdDirectoryService.getUsersByDepartment('dept-root', '300001'),
    [],
    'department member lookup must accept the selected department organization when department ids are reused across organizations',
  );
  assert.deepEqual(
    duplicateDepartmentIdCalls,
    ['iam.departmentAssignments.list:org-a:dept-root'],
    'department member lookup must pass the selected organization id instead of relying on a department-id-only cache',
  );

  const unscopedDepartmentDirectoryService = createSdkworkOrganizationDirectoryService(() => ({
    iam: {
      organizations: {
        tree: {
          async retrieve() {
            return {
              items: [
                {
                  organizationId: '300003',
                  name: 'Root Organization',
                  parentOrganizationId: null,
                  order: 0,
                  children: [
                    {
                      organizationId: '300001',
                      name: 'Company Organization',
                      parentOrganizationId: '300003',
                      order: 10,
                      children: [],
                    },
                  ],
                },
              ],
            };
          },
        },
      },
      departments: {
        tree: {
          async retrieve(params) {
            unscopedDepartmentCalls.push(`iam.departments.tree.retrieve:${params?.organizationId ?? ''}`);
            if (params?.organizationId === '300003') {
              return { items: [] };
            }
            return {
              items: [
                {
                  departmentId: 'dept-unscoped',
                  name: 'Unscoped Department',
                  parentDepartmentId: null,
                  order: 0,
                  children: [],
                },
              ],
            };
          },
        },
      },
    },
  }) satisfies OrganizationDirectoryClient);
  const unscopedDepartmentCalls: string[] = [];
  const unscopedDirectoryTree = (await unscopedDepartmentDirectoryService.getOrganizationDirectoryTree()).map(simplifyDirectoryNode);
  assert.deepEqual(
    unscopedDirectoryTree,
    [
      {
        children: [
          {
            children: [
              {
                children: [],
                departmentId: 'dept-unscoped',
                kind: 'department',
                name: 'Unscoped Department',
                organizationId: '300001',
              },
            ],
            kind: 'organization',
            name: 'Company Organization',
            organizationId: '300001',
          },
        ],
        kind: 'organization',
        name: 'Root Organization',
        organizationId: '300003',
      },
    ],
    'organization directory must resolve unscoped department trees through scoped organization department reads',
  );
  assert.deepEqual(
    unscopedDepartmentCalls,
    [
      'iam.departments.tree.retrieve:',
      'iam.departments.tree.retrieve:org-root',
      'iam.departments.tree.retrieve:org-company',
    ],
    'organization directory must re-read department trees by organization when the global department tree has no organization scope',
  );

  const memberManagementCalls: string[] = [];
  const memberManagementAdminCalls: string[] = [];
  const memberManagementClient = {
    iam: {
      users: {
        current: {
          async retrieve() {
            memberManagementCalls.push('iam.users.current.retrieve');
            return {
              userId: 'u_admin',
              displayName: 'Organization Admin',
              email: 'admin@example.com',
              avatarUrl: 'https://example.com/admin.png',
              status: 'active',
            };
          },
        },
      },
      organizationMemberships: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.organizationMemberships.list params');
          memberManagementCalls.push(`iam.organizationMemberships.list:${params?.organizationId ?? ''}:${params?.userId ?? ''}`);
          if (params?.userId === 'u_admin') {
            return [
              {
                membershipId: 'membership-admin-company',
                tenantId: '100001',
                organizationId: '300001',
                userId: 'u_admin',
                primary: true,
                status: 'active',
              },
            ];
          }
          return [];
        },
      },
      roleBindings: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.roleBindings.list params');
          memberManagementCalls.push(`iam.roleBindings.list:${params?.scopeKind ?? ''}:${params?.scopeId ?? ''}:${params?.principalId ?? ''}`);
          if (params?.principalId === 'membership-admin-company') {
            return [
              {
                roleBindingId: 'rb-admin-company',
                tenantId: '100001',
                roleCode: 'org.admin',
                principalKind: 'organization_membership',
                principalId: 'membership-admin-company',
                scopeKind: 'organization',
                scopeId: '300001',
                status: 'active',
              },
            ];
          }
          return [];
        },
      },
    },
  } satisfies OrganizationDirectoryClient;
  const memberManagementAdmin = {
    users: {
      async create(body) {
        assertNoTenantParam(body, 'current tenant context must not be sent in admin.users.create body');
        memberManagementAdminCalls.push(`admin.users.create:${body.email ?? body.phone ?? ''}`);
        return {
          userId: 'u_invited',
          displayName: body.displayName,
          email: body.email,
          status: 'invited',
        };
      },
    },
    organizationMemberships: {
      async create(body) {
        assertNoTenantParam(body, 'current tenant context must not be sent in admin.organizationMemberships.create body');
        memberManagementAdminCalls.push(`admin.organizationMemberships.create:${body.organizationId}:${body.userId}`);
        return {
          membershipId: body.userId === 'u_invited' ? 'membership-invited-company' : 'membership-charlie-company',
          ...body,
        };
      },
    },
    departmentAssignments: {
      async create(body) {
        assertNoTenantParam(body, 'current tenant context must not be sent in admin.departmentAssignments.create body');
        memberManagementAdminCalls.push(`admin.departmentAssignments.create:${body.departmentId}:${body.organizationMembershipId}`);
        return {
          assignmentId: body.userId === 'u_invited' ? 'assign-invited-rd' : 'assign-charlie-rd',
          ...body,
        };
      },
    },
    positionAssignments: {
      async create(body) {
        assertNoTenantParam(body, 'current tenant context must not be sent in admin.positionAssignments.create body');
        memberManagementAdminCalls.push(`admin.positionAssignments.create:${body.departmentAssignmentId}:${body.positionId ?? ''}`);
        return {
          positionAssignmentId: 'pos-assign-charlie',
          ...body,
        };
      },
    },
    roleBindings: {
      async create(body) {
        assertNoTenantParam(body, 'current tenant context must not be sent in admin.roleBindings.create body');
        memberManagementAdminCalls.push(`admin.roleBindings.create:${body.scopeKind}:${body.scopeId}:${body.roleCode}`);
        return {
          roleBindingId: body.roleCode === 'org.member' ? 'rb-org-member' : 'rb-dept-engineer',
          ...body,
        };
      },
    },
  };
  const memberManagementDirectoryService = createSdkworkOrganizationDirectoryService(() => memberManagementClient, {
    admin: memberManagementAdmin,
  });

  assert.deepEqual(
    await memberManagementDirectoryService.getCurrentUser(),
    {
      avatar: 'https://example.com/admin.png',
      email: 'admin@example.com',
      id: 'u_admin',
      name: 'Organization Admin',
      py: 'organizationadmin',
      status: 'online',
    },
    'organization contacts view must read the logged-in IAM user through iam.users.current.retrieve',
  );
  assert.deepEqual(
    await memberManagementDirectoryService.getOrganizationPermissions('300001'),
    {
      adminCapabilityAvailable: true,
      canAssignRoles: true,
      canInviteMembers: true,
      canManageMembers: true,
      currentUserId: 'u_admin',
      organizationId: '300001',
      organizationMembershipIds: ['membership-admin-company'],
      reason: 'role_allowed',
      roleCodes: ['org.admin'],
    },
    'organization contacts view must derive admin member-management permissions from scoped organization role bindings',
  );
  assert.deepEqual(
    await memberManagementDirectoryService.addOrganizationMember({
      assignmentType: 'secondary',
      departmentId: 'dept-rd',
      membershipType: 'employee',
      organizationId: '300001',
      positionId: 'pos-engineer',
      roleCodes: ['org.member', 'department.engineer'],
      userId: 'u_charlie',
    }),
    {
      departmentAssignmentId: 'assign-charlie-rd',
      organizationId: '300001',
      organizationMembershipId: 'membership-charlie-company',
      positionAssignmentIds: ['pos-assign-charlie'],
      roleBindingIds: ['rb-org-member', 'rb-dept-engineer'],
      userId: 'u_charlie',
    },
    'organization contacts view must add a member through membership, department assignment, position assignment, and scoped role binding APIs',
  );
  assert.deepEqual(
    await memberManagementDirectoryService.inviteOrganizationMember({
      assignmentType: 'primary',
      departmentId: 'dept-rd',
      displayName: 'Invited User',
      email: 'invite@example.com',
      membershipType: 'employee',
      organizationId: '300001',
      roleCodes: ['org.member'],
    }),
    {
      departmentAssignmentId: 'assign-invited-rd',
      invitedUserId: 'u_invited',
      organizationId: '300001',
      organizationMembershipId: 'membership-invited-company',
      positionAssignmentIds: [],
      roleBindingIds: ['rb-org-member'],
      userId: 'u_invited',
    },
    'organization contacts view must invite unknown people through the IAM user capability before attaching organization membership',
  );
  assert.deepEqual(
    memberManagementAdminCalls,
    [
      'admin.organizationMemberships.create:org-company:u_charlie',
      'admin.departmentAssignments.create:dept-rd:membership-charlie-company',
      'admin.positionAssignments.create:assign-charlie-rd:pos-engineer',
      'admin.roleBindings.create:organization:org-company:org.member',
      'admin.roleBindings.create:department_assignment:assign-charlie-rd:department.engineer',
      'admin.users.create:invite@example.com',
      'admin.organizationMemberships.create:org-company:u_invited',
      'admin.departmentAssignments.create:dept-rd:membership-invited-company',
      'admin.roleBindings.create:organization:org-company:org.member',
    ],
    'organization member management must use injected IAM admin capabilities instead of handwritten backend HTTP',
  );

  const deniedAdminCalls: string[] = [];
  const deniedDirectoryService = createSdkworkOrganizationDirectoryService(() => ({
    iam: {
      users: {
        current: {
          async retrieve() {
            return { userId: 'u_member', displayName: 'Member User', status: 'active' };
          },
        },
      },
      organizationMemberships: {
        async list() {
          return [
            {
              membershipId: 'membership-member-company',
              organizationId: '300001',
              userId: 'u_member',
              status: 'active',
            },
          ];
        },
      },
      roleBindings: {
        async list() {
          return [
            {
              roleBindingId: 'rb-member-company',
              roleCode: 'org.member',
              scopeKind: 'organization',
              scopeId: '300001',
              status: 'active',
            },
          ];
        },
      },
    },
  }) satisfies OrganizationDirectoryClient, {
    admin: {
      organizationMemberships: {
        async create(body) {
          assertNoTenantParam(body, 'current tenant context must not be sent in denied admin body');
          deniedAdminCalls.push(`admin.organizationMemberships.create:${body.userId}`);
          return body;
        },
      },
    },
  });
  await assert.rejects(
    () => deniedDirectoryService.addOrganizationMember({
      organizationId: '300001',
      userId: 'u_blocked',
    }),
    /not allowed to manage organization members/u,
    'organization member management must be rejected before mutation when the logged-in user has no admin role',
  );
  assert.deepEqual(deniedAdminCalls, [], 'permission-denied organization member management must not call admin mutation capabilities');

  const emptyOrganizationDirectoryService = createSdkworkOrganizationDirectoryService(() => ({
    iam: {
      departments: {
        async list() {
          return [];
        },
      },
      departmentAssignments: {
        async list() {
          return [];
        },
      },
    },
    async listDepartments() {
      return [];
    },
    async listDepartmentAssignments() {
      return [];
    },
  }));
  const emptyDirectoryService = createSdkworkContactService(
    () => fakeImClient,
    () => forbiddenPortalAppClient,
    () => emptyOrganizationDirectoryService,
  );

  assert.deepEqual(
    await emptyDirectoryService.getDepartments(),
    [],
    'contact org directory must not synthesize departments when the organization directory has no records',
  );
  assert.deepEqual(
    await emptyDirectoryService.getUsersByDepartment('300003'),
    [],
    'contact org directory users must not fall back to IM contacts for a synthetic org-root department',
  );

  const multiOrganizationCalls: string[] = [];
  const multiOrganizationDirectoryService = createSdkworkOrganizationDirectoryService(() => ({
    iam: {
      organizations: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.organizations.list params');
          multiOrganizationCalls.push(`iam.organizations.list:${params?.tenantId ?? ''}`);
          return [
            {
              organizationId: '300002',
              tenantId: '100001',
              name: 'SDKWork Group',
              parentOrganizationId: null,
              organizationKind: 'group',
              tenantBoundaryKind: 'root_tenant',
              verificationStatus: 'verified',
              status: 'active',
              order: 0,
            },
            {
              organizationId: '300001',
              tenantId: '100001',
              name: 'SDKWork Cloud Company',
              parentOrganizationId: '300002',
              organizationKind: 'company',
              tenantBoundaryKind: 'operating_subject',
              verificationStatus: 'verified',
              status: 'active',
              order: 10,
            },
          ];
        },
      },
      organizationMemberships: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.organizationMemberships.list params');
          multiOrganizationCalls.push(`iam.organizationMemberships.list:${params?.userId ?? ''}`);
          return [
            {
              membershipId: 'membership-bob-group',
              tenantId: '100001',
              organizationId: '300002',
              userId: 'u_bob',
              membershipType: 'employee',
              status: 'active',
              primary: false,
            },
            {
              membershipId: 'membership-bob-company',
              tenantId: '100001',
              organizationId: '300001',
              userId: 'u_bob',
              membershipType: 'employee',
              status: 'active',
              primary: true,
            },
          ];
        },
      },
      departments: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.departments.list params');
          assertNoOrganizationParam(params, 'current organization context must not be sent as iam.departments.list params');
          multiOrganizationCalls.push(`iam.departments.list:${params?.organizationId ?? ''}`);
          return [
            {
              departmentId: 'dept-company-root',
              tenantId: '100001',
              organizationId: '300001',
              name: 'Company Headquarters',
              parentDepartmentId: null,
              departmentKind: 'headquarters',
              status: 'active',
              order: 0,
            },
          ];
        },
      },
      departmentAssignments: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.departmentAssignments.list params');
          multiOrganizationCalls.push(`iam.departmentAssignments.list:${params?.organizationId ?? ''}:${params?.departmentId ?? ''}`);
          return [];
        },
      },
    },
  }));

  assert.deepEqual(
    await multiOrganizationDirectoryService.getDepartments(),
    [
      {
        id: 'dept-company-root',
        name: 'Company Headquarters',
        organizationId: '300001',
        parentId: null,
        order: 0,
      },
    ],
    'contact org directory must resolve the active organization membership before listing departments in a multi-organization tenant',
  );
  assert.deepEqual(
    multiOrganizationCalls,
    [
      'iam.departments.list:',
    ],
    'contact org directory must rely on request Context for current organization instead of resolving and passing it as params',
  );

  const orgContainerSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/contacts/OrgContainer.tsx');
  assert.match(
    orgContainerSource,
    /organizationDirectoryService\.getOrganizationDirectoryTree\(\)/u,
    'OrgContainer must load the unified organization/department directory tree from OrganizationDirectoryService',
  );
  assert.doesNotMatch(
    orgContainerSource,
    /organizationDirectoryService\.getOrganizations\(\)|organizationDirectoryService\.getOrganizationTree\(\)|organizationDirectoryService\.getDepartmentTree\(/u,
    'OrgContainer must not keep separate organization and department tree loading paths',
  );
  assert.doesNotMatch(
    orgContainerSource,
    /visibleOrganizationTree|visibleDepartmentTree|renderOrganizationNode|renderDepartmentNode/u,
    'OrgContainer must render one unified organization-directory tree instead of separate organization and department trees',
  );

  console.log('sdkwork-im-pc contact org directory real-logic contract passed');
}

void main();

import React, { useEffect, useMemo, useState } from 'react';
import { Building2, Briefcase, FolderTree, MessageSquare, Search, UserPlus, Users, X } from 'lucide-react';
import { useTranslation } from 'react-i18next';
import { Avatar, cn } from '@sdkwork/im-pc-commons';
import { toast } from '../Toast';
import { organizationDirectoryService } from '../../services/OrganizationDirectoryService';
import type { User as UserType } from '@sdkwork/im-pc-types';
import type {
  OrgDepartment,
  OrgOrganization,
  OrganizationDirectoryPermission,
  OrganizationDirectoryTreeNode,
} from '../../services/OrganizationDirectoryService';

type OrgContainerProps = {
  onUserSelect: (user: UserType, deptName: string) => void;
  selectedUserId: string | null;
  onSendMessage?: (user: UserType) => void;
  searchQuery?: string;
};

type MemberManagementMode = 'add' | 'invite';

type MemberManagementForm = {
  assignmentType: string;
  departmentId: string;
  displayName: string;
  email: string;
  memberTarget: string;
  membershipType: string;
  phone: string;
  positionId: string;
  positionName: string;
  roleCodes: string;
};

const EMPTY_MEMBER_MANAGEMENT_FORM: MemberManagementForm = {
  assignmentType: 'primary',
  departmentId: '',
  displayName: '',
  email: '',
  memberTarget: '',
  membershipType: 'employee',
  phone: '',
  positionId: '',
  positionName: '',
  roleCodes: 'org.member',
};

function normalizeSearch(value: string): string {
  return value.trim().toLowerCase();
}

function matchesQuery(query: string, ...values: Array<string | null | undefined>): boolean {
  if (!query) {
    return true;
  }
  return values.some((value) => value?.toLowerCase().includes(query));
}

function filterDirectoryTree(
  nodes: OrganizationDirectoryTreeNode[],
  query: string,
): OrganizationDirectoryTreeNode[] {
  if (!query) {
    return nodes;
  }

  return nodes
    .map((node) => {
      const children = filterDirectoryTree(node.children, query);
      const matches = matchesQuery(
        query,
        node.name,
        node.kind,
        node.organizationId,
        node.departmentId,
        node.organization?.organizationKind,
        node.organization?.tenantBoundaryKind,
        node.organization?.dataBoundaryKind,
        node.organization?.verificationStatus,
        node.organization?.status,
      );
      return children.length > 0 || matches ? { ...node, children } : null;
    })
    .filter(Boolean) as OrganizationDirectoryTreeNode[];
}

function flattenDirectoryOrganizations(nodes: OrganizationDirectoryTreeNode[]): OrgOrganization[] {
  return nodes.flatMap((node) => [
    ...(node.organization ? [node.organization] : []),
    ...flattenDirectoryOrganizations(node.children),
  ]);
}

function flattenDirectoryDepartments(nodes: OrganizationDirectoryTreeNode[]): OrgDepartment[] {
  return nodes.flatMap((node) => [
    ...(node.department ? [node.department] : []),
    ...flattenDirectoryDepartments(node.children),
  ]);
}

function firstOrganizationNode(nodes: OrganizationDirectoryTreeNode[]): OrganizationDirectoryTreeNode | null {
  for (const node of nodes) {
    if (node.kind === 'organization') {
      return node;
    }
    const child = firstOrganizationNode(node.children);
    if (child) {
      return child;
    }
  }
  return null;
}

function firstDepartmentNode(nodes: OrganizationDirectoryTreeNode[]): OrganizationDirectoryTreeNode | null {
  for (const node of nodes) {
    if (node.kind === 'department') {
      return node;
    }
    const child = firstDepartmentNode(node.children);
    if (child) {
      return child;
    }
  }
  return null;
}

function findOrganizationForNode(
  node: OrganizationDirectoryTreeNode,
  organizations: OrgOrganization[],
): OrgOrganization | null {
  if (node.organization) {
    return node.organization;
  }
  if (!node.organizationId) {
    return null;
  }
  return organizations.find((organization) => organization.organizationId === node.organizationId) ?? null;
}

function memberMatchesQuery(user: UserType, query: string): boolean {
  return matchesQuery(
    query,
    user.name,
    user.email,
    user.phone,
    user.position,
    user.departmentAssignmentId,
    user.organizationMembershipId,
    ...(user.roleCodes ?? []),
    ...(user.roleBindings ?? []).map((binding) => binding.roleCode),
    ...(user.positionAssignments ?? []).map((assignment) => assignment.positionName),
  );
}

function parseRoleCodes(value: string): string[] {
  return value
    .split(/[,\s，、]+/u)
    .map((item) => item.trim())
    .filter(Boolean);
}

function errorMessage(error: unknown, fallback: string): string {
  return error instanceof Error && error.message.trim().length > 0 ? error.message : fallback;
}

function boundaryTranslationKey(organization?: OrgOrganization): string | null {
  if (organization?.tenantBoundaryKind === 'operating_subject') {
    return 'contacts.organizationDirectory.boundary.operatingSubject';
  }
  if (organization?.dataBoundaryKind === 'organization_isolated') {
    return 'contacts.organizationDirectory.boundary.organizationIsolated';
  }
  if (organization?.dataBoundaryKind === 'regulated_isolated') {
    return 'contacts.organizationDirectory.boundary.regulatedIsolated';
  }
  if (organization?.appBoundaryEnabled) {
    return 'contacts.organizationDirectory.boundary.appBoundary';
  }
  return null;
}

export const OrgContainer: React.FC<OrgContainerProps> = ({
  onUserSelect,
  selectedUserId,
  onSendMessage,
  searchQuery = '',
}) => {
  const { t } = useTranslation();
  const [directoryTree, setDirectoryTree] = useState<OrganizationDirectoryTreeNode[]>([]);
  const [organizations, setOrganizations] = useState<OrgOrganization[]>([]);
  const [allDepartments, setAllDepartments] = useState<OrgDepartment[]>([]);
  const [currentOrganization, setCurrentOrganization] = useState<OrgOrganization | null>(null);
  const [currentDepartment, setCurrentDepartment] = useState<OrgDepartment | null>(null);
  const [selectedNodeId, setSelectedNodeId] = useState<string | null>(null);
  const [users, setUsers] = useState<UserType[]>([]);
  const [selectedUser, setSelectedUser] = useState<UserType | null>(null);
  const [organizationPermission, setOrganizationPermission] = useState<OrganizationDirectoryPermission | null>(null);
  const [loading, setLoading] = useState(true);
  const [membersLoading, setMembersLoading] = useState(false);
  const [localSearch, setLocalSearch] = useState('');
  const [memberManagementMode, setMemberManagementMode] = useState<MemberManagementMode>('add');
  const [memberManagementOpen, setMemberManagementOpen] = useState(false);
  const [memberManagementForm, setMemberManagementForm] = useState<MemberManagementForm>(EMPTY_MEMBER_MANAGEMENT_FORM);
  const [memberSaving, setMemberSaving] = useState(false);

  const activeSearchQuery = normalizeSearch(localSearch || searchQuery);
  const visibleDirectoryTree = useMemo(
    () => filterDirectoryTree(directoryTree, activeSearchQuery),
    [activeSearchQuery, directoryTree],
  );
  const visibleUsers = useMemo(
    () => users.filter((user) => memberMatchesQuery(user, activeSearchQuery)),
    [activeSearchQuery, users],
  );
  const currentOrganizationDepartments = useMemo(
    () => {
      if (!currentOrganization) {
        return allDepartments;
      }
      return allDepartments.filter((department) => department.organizationId === currentOrganization.organizationId);
    },
    [allDepartments, currentOrganization],
  );
  const canManageMembers = organizationPermission?.canManageMembers === true;
  const canInviteMembers = organizationPermission?.canInviteMembers === true;
  const memberManagementUnavailable = organizationPermission?.reason === 'missing_admin_capability';
  const directoryNodeCount = organizations.length + allDepartments.length;

  useEffect(() => {
    void loadRoot();
  }, []);

  const loadRoot = async () => {
    setLoading(true);
    try {
      const tree = await organizationDirectoryService.getOrganizationDirectoryTree();
      await organizationDirectoryService.getCurrentUser().catch(() => null);
      const nextOrganizations = flattenDirectoryOrganizations(tree);
      const nextDepartments = flattenDirectoryDepartments(tree);
      setDirectoryTree(tree);
      setOrganizations(nextOrganizations);
      setAllDepartments(nextDepartments);
      setCurrentOrganization(null);
      setCurrentDepartment(null);
      setSelectedNodeId(null);
      setUsers([]);
      setSelectedUser(null);
      setOrganizationPermission(null);
      setMemberManagementOpen(false);

      const defaultNode = firstDepartmentNode(tree) ?? firstOrganizationNode(tree);
      if (defaultNode) {
        await selectDirectoryNode(defaultNode, {
          departments: nextDepartments,
          manageLoading: false,
          organizations: nextOrganizations,
        });
      }
    } catch {
      setDirectoryTree([]);
      setOrganizations([]);
      setAllDepartments([]);
      setCurrentOrganization(null);
      setCurrentDepartment(null);
      setSelectedNodeId(null);
      setUsers([]);
      setOrganizationPermission(null);
      setMemberManagementOpen(false);
      toast(t('contacts.organizationDirectory.toast.loadTreeFailed'), 'error');
    } finally {
      setLoading(false);
    }
  };

  const selectDirectoryNode = async (
    node: OrganizationDirectoryTreeNode,
    options: {
      departments?: OrgDepartment[];
      manageLoading?: boolean;
      organizations?: OrgOrganization[];
    } = {},
  ) => {
    const knownOrganizations = options.organizations ?? organizations;
    const knownDepartments = options.departments ?? allDepartments;
    const manageLoading = options.manageLoading ?? true;
    const organization = findOrganizationForNode(node, knownOrganizations);
    const department = node.department ?? null;
    const selectableDepartments = organization
      ? knownDepartments.filter((item) => item.organizationId === organization.organizationId)
      : knownDepartments;

    if (manageLoading) {
      setLoading(true);
    }
    setSelectedNodeId(node.id);
    setSelectedUser(null);
    setCurrentOrganization(organization);
    setCurrentDepartment(department);
    setOrganizationPermission(null);
    setMemberManagementOpen(false);

    try {
      const permission = organization
        ? await organizationDirectoryService.getOrganizationPermissions(organization.organizationId).catch(() => null)
        : null;
      setOrganizationPermission(permission);
      if (department) {
        setMemberManagementForm({
          ...EMPTY_MEMBER_MANAGEMENT_FORM,
          departmentId: department.id,
        });
        await loadDepartmentMembers(department);
      } else {
        setMemberManagementForm({
          ...EMPTY_MEMBER_MANAGEMENT_FORM,
          departmentId: selectableDepartments[0]?.id ?? '',
        });
        setUsers([]);
      }
    } catch {
      setUsers([]);
      setOrganizationPermission(null);
      setMemberManagementOpen(false);
      toast(t('contacts.organizationDirectory.toast.loadTreeFailed'), 'error');
    } finally {
      if (manageLoading) {
        setLoading(false);
      }
    }
  };

  const loadDepartmentMembers = async (department: OrgDepartment) => {
    setMembersLoading(true);
    setSelectedUser(null);
    setCurrentDepartment(department);
    try {
      const deptUsers = await organizationDirectoryService.getUsersByDepartment(department.id, department.organizationId);
      setUsers(deptUsers);
    } catch {
      setUsers([]);
      toast(t('contacts.organizationDirectory.toast.loadMembersFailed'), 'error');
    } finally {
      setMembersLoading(false);
    }
  };

  const handleNavigate = (node: OrganizationDirectoryTreeNode) => {
    void selectDirectoryNode(node);
  };

  const openMemberManagement = (mode: MemberManagementMode) => {
    if (!currentOrganization) {
      toast(t('contacts.organizationDirectory.toast.selectOrganization'), 'error');
      return;
    }
    if (!canManageMembers) {
      toast(
        memberManagementUnavailable
          ? t('contacts.organizationDirectory.toast.managementUnavailable')
          : t('contacts.organizationDirectory.toast.noManagePermission'),
        'error',
      );
      return;
    }
    if (mode === 'invite' && !canInviteMembers) {
      toast(t('contacts.organizationDirectory.toast.inviteUnavailable'), 'error');
      return;
    }

    const departmentId = currentDepartment?.id ?? currentOrganizationDepartments[0]?.id ?? '';
    setMemberManagementMode(mode);
    setMemberManagementForm({
      ...EMPTY_MEMBER_MANAGEMENT_FORM,
      departmentId,
    });
    setMemberManagementOpen(true);
  };

  const closeMemberManagement = () => {
    setMemberManagementOpen(false);
    setMemberSaving(false);
  };

  const updateMemberManagementForm = (field: keyof MemberManagementForm, value: string) => {
    setMemberManagementForm((form) => ({ ...form, [field]: value }));
  };

  const handleMemberManagementSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (!currentOrganization) {
      toast(t('contacts.organizationDirectory.toast.selectOrganization'), 'error');
      return;
    }
    if (!canManageMembers) {
      toast(t('contacts.organizationDirectory.toast.noManagePermission'), 'error');
      return;
    }

    const departmentId = memberManagementForm.departmentId.trim();
    if (!departmentId) {
      toast(t('contacts.organizationDirectory.toast.selectDepartment'), 'error');
      return;
    }
    const selectedDepartment = currentOrganizationDepartments.find((item) => item.id === departmentId);
    if (!selectedDepartment) {
      toast(t('contacts.organizationDirectory.toast.selectDepartment'), 'error');
      return;
    }

    const organizationId = currentOrganization.organizationId;
    setMemberSaving(true);
    try {
      const sharedInput = {
        assignmentType: memberManagementForm.assignmentType.trim() || undefined,
        departmentId,
        membershipType: memberManagementForm.membershipType.trim() || undefined,
        organizationId,
        positionId: memberManagementForm.positionId.trim() || undefined,
        positionName: memberManagementForm.positionName.trim() || undefined,
        roleCodes: parseRoleCodes(memberManagementForm.roleCodes),
      };

      if (memberManagementMode === 'invite') {
        await organizationDirectoryService.inviteOrganizationMember({
          ...sharedInput,
          displayName: memberManagementForm.displayName.trim() || undefined,
          email: memberManagementForm.email.trim() || undefined,
          phone: memberManagementForm.phone.trim() || undefined,
        });
      } else {
        const userId = memberManagementForm.memberTarget.trim();
        if (!userId) {
          toast(t('contacts.organizationDirectory.toast.enterUserId'), 'error');
          setMemberSaving(false);
          return;
        }
        await organizationDirectoryService.addOrganizationMember({
          ...sharedInput,
          userId,
        });
      }

      setMemberManagementOpen(false);
      setMemberManagementForm({
        ...EMPTY_MEMBER_MANAGEMENT_FORM,
        departmentId,
      });
      await loadDepartmentMembers(selectedDepartment);
      toast(t('contacts.organizationDirectory.toast.memberSubmitted'), 'success');
    } catch (error) {
      toast(errorMessage(error, t('contacts.organizationDirectory.toast.memberSubmitFailed')), 'error');
    } finally {
      setMemberSaving(false);
    }
  };

  const handleUserSelect = (user: UserType) => {
    setSelectedUser(user);
    onUserSelect(user, currentDepartmentName());
  };

  const handleSendMessage = (user: UserType) => {
    handleUserSelect(user);
    if (onSendMessage) {
      onSendMessage(user);
    } else {
      toast(t('contacts.organizationDirectory.toast.chatUnavailable'), 'error');
    }
  };

  const currentDepartmentName = () => (
    currentDepartment?.name
    || currentOrganization?.name
    || t('contacts.organizationDirectory.unknownOrganization')
  );

  const renderDirectoryTreeNode = (node: OrganizationDirectoryTreeNode, depth = 0): React.ReactNode => {
    const selected = selectedNodeId === node.id;
    const boundaryKey = boundaryTranslationKey(node.organization);
    const childCount = node.children.length;
    return (
      <div key={node.id}>
        <button
          type="button"
          className={cn(
            'group flex w-full items-center gap-2 px-3 py-2 text-left transition-colors hover:bg-white/5',
            selected && 'bg-white/10 hover:bg-white/10',
          )}
          style={{ paddingLeft: `${12 + depth * 16}px` }}
          onClick={() => handleNavigate(node)}
        >
          {node.kind === 'organization' ? (
            <Building2 size={14} className="shrink-0 text-gray-500" />
          ) : (
            <FolderTree size={14} className="shrink-0 text-gray-500" />
          )}
          <span className={cn('min-w-0 flex-1 truncate text-[13px]', selected ? 'text-gray-100' : 'text-gray-300')}>
            {node.name}
          </span>
          {boundaryKey && (
            <span className="shrink-0 rounded bg-cyan-400/10 px-1.5 py-0.5 text-[10px] text-cyan-300">
              {t(boundaryKey)}
            </span>
          )}
          {node.kind === 'department' && childCount > 0 && (
            <span className="shrink-0 rounded bg-white/5 px-1.5 py-0.5 text-[10px] text-gray-500">
              {childCount}
            </span>
          )}
        </button>
        {node.children.map((child) => renderDirectoryTreeNode(child, depth + 1))}
      </div>
    );
  };

  return (
    <div className="flex flex-1 min-w-0 bg-[#1e1e1e]">
      <div className="flex w-[330px] shrink-0 flex-col bg-[#202020]">
        <div className="px-4 py-3">
          <div className="mb-3 flex items-center justify-between">
            <div className="flex items-center gap-2 text-sm font-medium text-gray-200">
              <Building2 size={16} className="text-gray-500" />
              {t('contacts.organizationDirectory.treeTitle')}
            </div>
            <span className="text-xs text-gray-500">{directoryNodeCount}</span>
          </div>
          <div className="relative">
            <Search size={14} className="absolute left-2.5 top-1/2 -translate-y-1/2 text-gray-500" />
            <input
              value={localSearch}
              onChange={(event) => setLocalSearch(event.target.value)}
              placeholder={searchQuery || t('contacts.organizationDirectory.searchPlaceholder')}
              aria-label={t('contacts.organizationDirectory.searchPlaceholder')}
              className="h-8 w-full rounded-md bg-[#181818] pl-8 pr-2 text-[13px] text-gray-200 outline-none transition-colors placeholder:text-gray-600 focus:bg-[#161616]"
            />
          </div>
        </div>

        <div className="min-h-0 flex-1 overflow-y-auto py-2 custom-scrollbar">
          {loading ? (
            <div className="px-4 py-3 text-sm text-gray-500">{t('contacts.organizationDirectory.loading')}</div>
          ) : visibleDirectoryTree.length > 0 ? (
            visibleDirectoryTree.map((node) => renderDirectoryTreeNode(node))
          ) : (
            <div className="px-4 py-6 text-center text-sm text-gray-500">{t('contacts.organizationDirectory.emptyTree')}</div>
          )}
        </div>
      </div>

      <div className="flex min-w-0 flex-1 flex-col">
        <div className="flex h-10 shrink-0 items-center justify-between px-4">
          <div className="flex min-w-0 items-center gap-2 text-sm font-medium text-gray-300">
            <Users size={15} className="shrink-0 text-gray-500" />
            <span className="truncate">{currentDepartment?.name ?? t('contacts.organizationDirectory.membersTitle')}</span>
          </div>
          <div className="flex shrink-0 items-center gap-2">
            <span className="text-xs text-gray-500">{visibleUsers.length}</span>
            {currentOrganization && canManageMembers && (
              <button
                type="button"
                className="flex h-7 w-7 items-center justify-center rounded-md text-gray-400 transition-colors hover:bg-indigo-500/10 hover:text-indigo-300"
                title={t('contacts.organizationDirectory.addMember')}
                aria-label={t('contacts.organizationDirectory.addMember')}
                onClick={() => openMemberManagement('add')}
              >
                <UserPlus size={15} />
              </button>
            )}
          </div>
        </div>

        {memberManagementOpen && currentOrganization && (
          <form
            className="shrink-0 bg-[#1b1b1b] px-4 py-3"
            onSubmit={handleMemberManagementSubmit}
          >
            <div className="mb-3 flex items-center justify-between gap-3">
              <div className="flex min-w-0 items-center gap-2">
                <UserPlus size={15} className="shrink-0 text-indigo-300" />
                <span className="truncate text-sm font-medium text-gray-200">
                  {memberManagementMode === 'invite'
                    ? t('contacts.organizationDirectory.inviteMember')
                    : t('contacts.organizationDirectory.addMember')}
                </span>
              </div>
              <button
                type="button"
                className="flex h-7 w-7 items-center justify-center rounded-md text-gray-500 transition-colors hover:bg-white/5 hover:text-gray-200"
                title={t('contacts.organizationDirectory.close')}
                aria-label={t('contacts.organizationDirectory.close')}
                onClick={closeMemberManagement}
              >
                <X size={15} />
              </button>
            </div>

            <div className="mb-3 flex rounded-md bg-[#151515] p-0.5">
              <button
                type="button"
                className={cn(
                  'h-7 flex-1 rounded px-2 text-xs transition-colors',
                  memberManagementMode === 'add' ? 'bg-white/10 text-gray-100' : 'text-gray-500 hover:text-gray-300',
                )}
                onClick={() => setMemberManagementMode('add')}
              >
                {t('contacts.organizationDirectory.add')}
              </button>
              <button
                type="button"
                className={cn(
                  'h-7 flex-1 rounded px-2 text-xs transition-colors',
                  memberManagementMode === 'invite' ? 'bg-white/10 text-gray-100' : 'text-gray-500 hover:text-gray-300',
                )}
                disabled={!canInviteMembers}
                onClick={() => openMemberManagement('invite')}
              >
                {t('contacts.organizationDirectory.invite')}
              </button>
            </div>

            <div className="grid grid-cols-2 gap-2">
              <select
                value={memberManagementForm.departmentId}
                onChange={(event) => updateMemberManagementForm('departmentId', event.target.value)}
                aria-label={t('contacts.organizationDirectory.selectDepartment')}
                className="h-8 min-w-0 rounded-md bg-[#181818] px-2 text-[12px] text-gray-200 outline-none focus:bg-[#161616]"
              >
                {currentOrganizationDepartments.map((department) => (
                  <option key={department.id} value={department.id}>
                    {department.name}
                  </option>
                ))}
              </select>
              <select
                value={memberManagementForm.assignmentType}
                onChange={(event) => updateMemberManagementForm('assignmentType', event.target.value)}
                aria-label={t('contacts.organizationDirectory.assignmentType')}
                className="h-8 min-w-0 rounded-md bg-[#181818] px-2 text-[12px] text-gray-200 outline-none focus:bg-[#161616]"
              >
                <option value="primary">{t('contacts.organizationDirectory.assignment.primary')}</option>
                <option value="secondary">{t('contacts.organizationDirectory.assignment.secondary')}</option>
                <option value="acting">{t('contacts.organizationDirectory.assignment.acting')}</option>
              </select>
              {memberManagementMode === 'add' ? (
                <input
                  value={memberManagementForm.memberTarget}
                  onChange={(event) => updateMemberManagementForm('memberTarget', event.target.value)}
                  placeholder={t('contacts.organizationDirectory.userIdPlaceholder')}
                  className="col-span-2 h-8 min-w-0 rounded-md bg-[#181818] px-2 text-[12px] text-gray-200 outline-none placeholder:text-gray-600 focus:bg-[#161616]"
                />
              ) : (
                <>
                  <input
                    value={memberManagementForm.displayName}
                    onChange={(event) => updateMemberManagementForm('displayName', event.target.value)}
                    placeholder={t('contacts.organizationDirectory.displayNamePlaceholder')}
                    className="h-8 min-w-0 rounded-md bg-[#181818] px-2 text-[12px] text-gray-200 outline-none placeholder:text-gray-600 focus:bg-[#161616]"
                  />
                  <input
                    value={memberManagementForm.email}
                    onChange={(event) => updateMemberManagementForm('email', event.target.value)}
                    placeholder={t('contacts.organizationDirectory.emailPlaceholder')}
                    className="h-8 min-w-0 rounded-md bg-[#181818] px-2 text-[12px] text-gray-200 outline-none placeholder:text-gray-600 focus:bg-[#161616]"
                  />
                </>
              )}
              <input
                value={memberManagementForm.positionId}
                onChange={(event) => updateMemberManagementForm('positionId', event.target.value)}
                placeholder={t('contacts.organizationDirectory.positionIdPlaceholder')}
                className="h-8 min-w-0 rounded-md bg-[#181818] px-2 text-[12px] text-gray-200 outline-none placeholder:text-gray-600 focus:bg-[#161616]"
              />
              <input
                value={memberManagementForm.roleCodes}
                onChange={(event) => updateMemberManagementForm('roleCodes', event.target.value)}
                placeholder={t('contacts.organizationDirectory.roleCodesPlaceholder')}
                className="h-8 min-w-0 rounded-md bg-[#181818] px-2 text-[12px] text-gray-200 outline-none placeholder:text-gray-600 focus:bg-[#161616]"
              />
            </div>

            <div className="mt-3 flex items-center justify-between gap-3">
              <div className="min-w-0 truncate text-[11px] text-gray-500">
                {organizationPermission?.roleCodes.join(', ') || currentOrganization.name}
              </div>
              <button
                type="submit"
                disabled={memberSaving || currentOrganizationDepartments.length === 0}
                className="h-8 shrink-0 rounded-md bg-indigo-500 px-3 text-xs font-medium text-white transition-colors hover:bg-indigo-400 disabled:cursor-not-allowed disabled:bg-gray-700 disabled:text-gray-400"
              >
                {memberSaving ? t('contacts.organizationDirectory.submitting') : t('contacts.organizationDirectory.submit')}
              </button>
            </div>
          </form>
        )}

        {!canManageMembers && memberManagementUnavailable && (
          <div className="shrink-0 px-4 py-2 text-xs text-amber-300">
            {t('contacts.organizationDirectory.managementUnavailable')}
          </div>
        )}

        <div className="min-h-0 flex-1 overflow-y-auto p-2 custom-scrollbar">
          {membersLoading ? (
            <div className="p-6 text-center text-sm text-gray-500">{t('contacts.organizationDirectory.loadingMembers')}</div>
          ) : visibleUsers.length > 0 ? (
            <div className="flex flex-col gap-1">
              {visibleUsers.map((user) => (
                <div
                  key={user.departmentAssignmentId ?? user.id}
                  className={cn(
                    'group flex cursor-pointer items-center gap-3 rounded-md px-3 py-2.5 transition-colors hover:bg-white/5',
                    (selectedUserId === user.id || selectedUser?.departmentAssignmentId === user.departmentAssignmentId) && 'bg-white/10 hover:bg-white/10',
                  )}
                  onClick={() => handleUserSelect(user)}
                >
                  <Avatar src={user.avatar} alt={user.name} className="h-9 w-9 shrink-0 rounded bg-[#2b2b2d]" />
                  <div className="min-w-0 flex-1">
                    <div className="flex min-w-0 items-center gap-2">
                      <span className="truncate text-[14px] font-medium text-gray-200">{user.name}</span>
                      {user.assignmentType && (
                        <span className="shrink-0 rounded bg-white/5 px-1.5 py-0.5 text-[10px] text-gray-500">
                          {user.assignmentType}
                        </span>
                      )}
                    </div>
                    <div className="mt-0.5 flex min-w-0 items-center gap-2 text-[12px] text-gray-500">
                      {user.position && (
                        <span className="inline-flex min-w-0 items-center gap-1 truncate">
                          <Briefcase size={12} className="shrink-0" />
                          <span className="truncate">{user.position}</span>
                        </span>
                      )}
                      {user.email && <span className="truncate">{user.email}</span>}
                    </div>
                    {(user.roleCodes?.length || user.roleBindings?.length || user.positionAssignments?.length) ? (
                      <div className="mt-1 flex flex-wrap gap-1">
                        {(user.roleCodes ?? []).slice(0, 3).map((roleCode) => (
                          <span key={roleCode} className="rounded bg-indigo-400/10 px-1.5 py-0.5 text-[10px] text-indigo-300">
                            {roleCode}
                          </span>
                        ))}
                        {user.roleBindings && user.roleBindings.length > 3 && (
                          <span className="rounded bg-white/5 px-1.5 py-0.5 text-[10px] text-gray-500">
                            +{user.roleBindings.length - 3}
                          </span>
                        )}
                        {user.positionAssignments && user.positionAssignments.length > 1 && (
                          <span className="rounded bg-white/5 px-1.5 py-0.5 text-[10px] text-gray-500">
                            {t('contacts.organizationDirectory.positionCount', { count: user.positionAssignments.length })}
                          </span>
                        )}
                      </div>
                    ) : null}
                  </div>
                  <button
                    type="button"
                    className="flex h-8 w-8 shrink-0 items-center justify-center rounded-md text-gray-500 opacity-0 transition-colors hover:bg-indigo-500/10 hover:text-indigo-300 group-hover:opacity-100"
                    title={t('contacts.organizationDirectory.sendMessage')}
                    aria-label={t('contacts.organizationDirectory.sendMessage')}
                    onClick={(event) => {
                      event.stopPropagation();
                      handleSendMessage(user);
                    }}
                  >
                    <MessageSquare size={15} />
                  </button>
                </div>
              ))}
            </div>
          ) : (
            <div className="p-8 text-center text-sm text-gray-500">{t('contacts.organizationDirectory.emptyMembers')}</div>
          )}
        </div>
      </div>
    </div>
  );
};

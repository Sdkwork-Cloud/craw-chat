export interface OrgDepartment {
  id: string;
  name: string;
  parentId: string | null;
  count: number;
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

export class OrganizationCapabilityUnavailableError extends Error {
  constructor() {
    super("Organization directory is not exposed by an approved owner SDK.");
    this.name = "OrganizationCapabilityUnavailableError";
  }
}

function unavailable<T>(): Promise<T> {
  return Promise.reject(new OrganizationCapabilityUnavailableError());
}

export const OrganizationService = {
  getOrganizations(): Promise<Organization[]> {
    return unavailable();
  },
  getDepartments(
    _organizationId: string,
    _parentId: string | null,
  ): Promise<OrgDepartment[]> {
    return unavailable();
  },
  getMembers(
    _organizationId: string,
    _departmentId: string,
  ): Promise<OrgMember[]> {
    return unavailable();
  },
  getDepartmentPath(_departmentId: string): Promise<OrgDepartment[]> {
    return unavailable();
  },
  searchMembers(_organizationId: string, _query: string): Promise<OrgMember[]> {
    return unavailable();
  },
};

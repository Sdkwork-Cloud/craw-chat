import type { User } from "@sdkwork/im-h5-types";

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

const mockOrgs: Organization[] = [
  { id: "org1", name: "Sdkwork IM H5 Tech", logo: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/org1/100x100.png" },
  { id: "org2", name: "Acme Corp", logo: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/org2/100x100.png" },
];

const mockDepts: OrgDepartment[] = [
  { id: "d1", name: "Engineering", parentId: null, count: 42 },
  { id: "d2", name: "Frontend", parentId: "d1", count: 12 },
  { id: "d3", name: "Backend", parentId: "d1", count: 30 },
  { id: "d4", name: "Design", parentId: null, count: 5 },
  { id: "d10", name: "Sales", parentId: null, count: 10 },
  { id: "d11", name: "NA Sales", parentId: "d10", count: 4 },
];

const mockMembers: OrgMember[] = [
  { id: "m1", name: "Alex Chen", avatar: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/alex/100x100.png", jobTitle: "Senior Engineer", departmentId: "d2" },
  { id: "m2", name: "Sarah Jenkins", avatar: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/sarah/100x100.png", jobTitle: "Engineeer", departmentId: "d2" },
  { id: "m3", name: "David Lee", avatar: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/david/100x100.png", jobTitle: "Backend Lead", departmentId: "d3" },
  { id: "m4", name: "Emily Chen", avatar: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/emily/100x100.png", jobTitle: "Designer", departmentId: "d4" },
  { id: "m5", name: "Michael Brown", avatar: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/michael/100x100.png", jobTitle: "Sales Director", departmentId: "d10" },
  { id: "m6", name: "Alice Wong", avatar: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/a1/100x100.png", jobTitle: "Sales Rep", departmentId: "d11" },
];

export const OrganizationService = {
  async getOrganizations(): Promise<Organization[]> {
    return new Promise((resolve) => setTimeout(() => resolve(mockOrgs), 300));
  },

  async getDepartments(orgId: string, parentId: string | null): Promise<OrgDepartment[]> {
    // In a real app we'd map this by orgId, but for mock we just use the same data
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve(mockDepts.filter(d => d.parentId === parentId));
      }, 300);
    });
  },

  async getMembers(orgId: string, departmentId: string): Promise<OrgMember[]> {
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve(mockMembers.filter(m => m.departmentId === departmentId));
      }, 300);
    });
  },

  async getDepartmentPath(deptId: string): Promise<OrgDepartment[]> {
    // return root to dept
    const path: OrgDepartment[] = [];
    let currentId: string | null = deptId;
    
    while (currentId) {
      const dept = mockDepts.find(d => d.id === currentId);
      if (dept) {
        path.unshift(dept);
        currentId = dept.parentId;
      } else {
        break;
      }
    }
    return new Promise((resolve) => setTimeout(() => resolve(path), 100));
  },

  async searchMembers(orgId: string, query: string): Promise<OrgMember[]> {
    return new Promise((resolve) => {
      setTimeout(() => {
        const q = query.toLowerCase();
        resolve(mockMembers.filter(m => m.name.toLowerCase().includes(q) || m.jobTitle.toLowerCase().includes(q)));
      }, 300);
    });
  }
};

export interface User {
  id: string;
  chatId?: string;
  conversationId?: string;
  directChatId?: string;
  name: string;
  avatar?: string;
  status?: 'online' | 'offline' | 'busy' | 'away';
  email?: string;
  departmentId?: string;
  departmentAssignmentId?: string;
  organizationId?: string;
  organizationMembershipId?: string;
  assignmentType?: string;
  position?: string;
  positionAssignments?: UserPositionAssignment[];
  roleBindings?: UserRoleBinding[];
  roleCodes?: string[];
  phone?: string;
  company?: string;
  location?: string;
  motto?: string;
  py?: string; // Pinyin for search/sorting
}

export interface UserPositionAssignment {
  positionAssignmentId: string;
  positionId?: string;
  positionName?: string;
  status?: string;
}

export interface UserRoleBinding {
  roleBindingId: string;
  roleCode: string;
  scopeId?: string;
  scopeKind?: string;
  status?: string;
}

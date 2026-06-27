export interface ContactPreferencesView {
  tenantId: string;
  ownerUserId: string;
  targetUserId: string;
  isStarred: boolean;
  remark: string;
  isBlocked: boolean;
  updatedAt: string;
}

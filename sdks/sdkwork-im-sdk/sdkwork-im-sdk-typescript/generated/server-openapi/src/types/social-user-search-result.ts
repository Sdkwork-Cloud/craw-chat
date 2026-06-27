export interface SocialUserSearchResult {
  tenantId: string;
  userId: string;
  chatId: string;
  displayName: string;
  relationshipState: string;
  avatarUrl?: string | null;
  email?: string | null;
  phone?: string | null;
  metadata?: Record<string, unknown>;
}

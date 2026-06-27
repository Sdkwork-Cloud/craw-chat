export interface ContactRecommendationView {
  tenantId: string;
  ownerUserId: string;
  targetUserId: string;
  recommendationId: string;
  targetConversationId?: string | null;
  createdAt: string;
}

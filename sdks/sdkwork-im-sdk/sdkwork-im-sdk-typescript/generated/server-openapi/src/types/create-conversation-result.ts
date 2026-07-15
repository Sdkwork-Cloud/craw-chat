export interface CreateConversationResult {
  conversationId: string;
  eventId: string;
  requestKey?: string;
  deliveryStatus?: 'applied' | 'replayed';
  proofVersion?: string;
  /** Present only when initializeKnowledgebase was true. A failed value means group creation succeeded but the optional remote Knowledgebase provisioning attempt did not complete; the group owner can retry from the Knowledgebase action. */
  knowledgebaseInitialization?: 'active' | 'provisioning' | 'failed';
}

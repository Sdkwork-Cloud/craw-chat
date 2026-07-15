import type { GroupKnowledgebaseLifecycleState } from './group-knowledgebase-lifecycle-state';

export interface GroupKnowledgebaseLaunchResponse {
  conversationId: string;
  lifecycleState: GroupKnowledgebaseLifecycleState;
  spaceId?: string;
  spaceUuid?: string;
  /** Opaque one-time ticket consumed only by sdkwork-knowledgebase through IM internal RPC. */
  launchTicket?: string;
  expiresAt?: string;
  membershipEpoch: string;
  upstreamLinkGeneration: string;
  provisioningOperationId?: string;
}

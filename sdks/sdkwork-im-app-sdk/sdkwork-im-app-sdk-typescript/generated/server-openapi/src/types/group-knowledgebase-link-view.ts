import type { GroupKnowledgebaseLifecycleState } from './group-knowledgebase-lifecycle-state';

export interface GroupKnowledgebaseLinkView {
  conversationId: string;
  spaceId?: string;
  spaceUuid?: string;
  lifecycleState: GroupKnowledgebaseLifecycleState;
  provisioningOperationId?: string;
  membershipEpoch: string;
  upstreamLinkGeneration: string;
  lastErrorCode?: string;
}

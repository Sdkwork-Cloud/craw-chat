import type { GroupKnowledgebaseLinkView } from './group-knowledgebase-link-view';

export interface ConversationsKnowledgebaseCreateResponse201 {
  code: 0;
  data: unknown & { item: GroupKnowledgebaseLinkView; };
  /** Server-owned request correlation id. */
  traceId: string;
}

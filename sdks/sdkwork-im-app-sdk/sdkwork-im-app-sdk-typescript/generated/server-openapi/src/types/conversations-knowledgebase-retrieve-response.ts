import type { GroupKnowledgebaseLinkView } from './group-knowledgebase-link-view';

export interface ConversationsKnowledgebaseRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}

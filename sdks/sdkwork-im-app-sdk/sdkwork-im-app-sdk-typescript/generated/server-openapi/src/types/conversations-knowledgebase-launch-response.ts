import type { GroupKnowledgebaseLaunchResponse } from './group-knowledgebase-launch-response';

export interface ConversationsKnowledgebaseLaunchResponse {
  code: 0;
  data: unknown & { item: GroupKnowledgebaseLaunchResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}

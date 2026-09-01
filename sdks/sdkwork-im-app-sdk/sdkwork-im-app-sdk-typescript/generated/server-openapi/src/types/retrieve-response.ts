import type { NotificationTask } from './notification-task';

export interface RetrieveResponse {
  code: 0;
  data: unknown & { item: NotificationTask; };
  /** Server-owned request correlation id. */
  traceId: string;
}

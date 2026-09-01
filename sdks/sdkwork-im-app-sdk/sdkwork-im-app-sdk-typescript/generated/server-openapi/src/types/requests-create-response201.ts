import type { NotificationRequestResponse } from './notification-request-response';

export interface RequestsCreateResponse201 {
  code: 0;
  data: unknown & { item: NotificationRequestResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}

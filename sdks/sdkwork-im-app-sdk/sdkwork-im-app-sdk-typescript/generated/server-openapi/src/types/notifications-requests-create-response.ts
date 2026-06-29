import type { NotificationRequestResponse } from './notification-request-response';

export interface NotificationsRequestsCreateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}

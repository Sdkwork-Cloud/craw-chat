import type { NotificationTask } from './notification-task';
import type { PageInfo } from './page-info';

export interface NotificationsListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}

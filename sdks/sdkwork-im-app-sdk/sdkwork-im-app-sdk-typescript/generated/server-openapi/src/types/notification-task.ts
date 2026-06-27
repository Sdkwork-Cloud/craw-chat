import type { NotificationStatus } from './notification-status';

export interface NotificationTask {
  tenantId: string;
  notificationId: string;
  sourceEventId: string;
  sourceEventType: string;
  category: string;
  channel: string;
  recipientId: string;
  recipientKind: string;
  status: NotificationStatus;
  title?: string;
  body?: string;
  payload?: string;
  requestedAt: string;
  dispatchedAt?: string;
  failureReason?: string;
}

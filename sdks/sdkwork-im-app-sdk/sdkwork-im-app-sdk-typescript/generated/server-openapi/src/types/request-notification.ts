export interface RequestNotification {
  notificationId: string;
  sourceEventId: string;
  sourceEventType: string;
  category: string;
  channel: string;
  recipientId: string;
  recipientKind: string;
  title?: string;
  body?: string;
  payload?: string;
}

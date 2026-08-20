export interface RetentionPurgeResponse {
  generatedAt: string;
  batchSize: string;
  commitJournalDeleted?: string;
  conversationMessagesDeleted?: string;
  messageMediaRefsDeleted?: string;
  outboxEventsDeleted?: string;
  inboxEventsDeleted?: string;
  realtimeDeviceEventsDeleted?: string;
  rtcSessionsDeleted?: string;
  invitationsDeleted?: string;
  auditRecordsDeleted?: string;
}

export interface PortalAuditRecordView {
  recordId: string;
  action: string;
  actorId: string;
  recordedAt: string;
  severity: 'critical' | 'high' | 'warning' | 'informational';
}

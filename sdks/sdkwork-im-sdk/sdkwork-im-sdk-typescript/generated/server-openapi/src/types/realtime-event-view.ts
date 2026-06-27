export interface RealtimeEventView {
  eventId: string;
  scope: string;
  scopeId: string;
  eventType: string;
  payload?: string | null;
  occurredAt: string;
}

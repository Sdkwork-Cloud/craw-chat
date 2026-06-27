export interface RtcSignalSender {
  id: string;
  kind: string;
  memberId?: string | null;
  deviceId?: string | null;
  sessionId?: string | null;
  metadata: Record<string, unknown>;
}

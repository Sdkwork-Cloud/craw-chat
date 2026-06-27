export interface CreateRtcSessionRequest {
  rtcSessionId: string;
  conversationId?: string | null;
  rtcMode: string;
}

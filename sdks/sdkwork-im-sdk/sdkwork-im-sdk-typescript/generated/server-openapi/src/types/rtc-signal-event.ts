import type { RtcSignalSender } from './rtc-signal-sender';

export interface RtcSignalEvent {
  tenantId: string;
  rtcSessionId: string;
  signalSeq: string;
  conversationId?: string | null;
  rtcMode: string;
  signalType: string;
  schemaRef?: string | null;
  payload: string;
  sender: RtcSignalSender;
  signalingStreamId?: string | null;
  occurredAt: string;
}

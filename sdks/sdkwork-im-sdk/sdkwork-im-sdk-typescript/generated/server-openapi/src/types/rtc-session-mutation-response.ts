export interface RtcSessionMutationResponse {
  tenantId: string;
  rtcSessionId: string;
  conversationId?: string | null;
  initiatorId: string;
  initiatorKind: string;
  providerPluginId?: string | null;
  providerSessionId?: string | null;
  accessEndpoint?: string | null;
  providerRegion?: string | null;
  rtcMode: string;
  state: string;
  signalingStreamId?: string | null;
  artifactMessageId?: string | null;
  startedAt: string;
  endedAt?: string;
  requestKey: string;
  deliveryStatus: 'applied' | 'replayed';
  proofVersion: string;
}

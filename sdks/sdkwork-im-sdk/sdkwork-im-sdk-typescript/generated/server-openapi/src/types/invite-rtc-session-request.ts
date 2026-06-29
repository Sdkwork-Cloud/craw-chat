export interface InviteRtcSessionRequest {
  signalingStreamId?: string | null;
  /** Principal IDs invited to the call session. Recorded in the session's invited_ids list so subsequent accept/reject/end authorization checks can admit them. */
  participantIds?: string[];
}

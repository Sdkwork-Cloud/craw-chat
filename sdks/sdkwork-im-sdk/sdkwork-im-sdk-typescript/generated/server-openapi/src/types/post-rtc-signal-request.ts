export interface PostRtcSignalRequest {
  signalType: string;
  schemaRef?: string | null;
  payload: string;
  signalingStreamId?: string | null;
}

export interface StreamRefContentPart {
  kind: 'stream_ref';
  streamId: string;
  streamType: string;
  state: string;
}

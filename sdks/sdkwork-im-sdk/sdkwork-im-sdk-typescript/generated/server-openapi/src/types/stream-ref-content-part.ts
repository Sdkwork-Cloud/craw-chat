export interface StreamRefContentPart {
  kind: 'stream_ref';
  streamId: string | null;
  streamType: string | null;
  state: string | null;
}

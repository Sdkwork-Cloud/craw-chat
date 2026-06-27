export interface SignalContentPart {
  kind: 'signal';
  signalType: string | null;
  schemaRef?: string | null;
  payload: string | null;
}

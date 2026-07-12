export interface SignalContentPart {
  kind: 'signal';
  signalType: string;
  schemaRef?: string | null;
  payload: string;
}

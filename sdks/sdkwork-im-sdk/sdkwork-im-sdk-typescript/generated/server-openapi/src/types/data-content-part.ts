export interface DataContentPart {
  kind: 'data';
  schemaRef: string | null;
  encoding: string | null;
  payload: string | null;
}

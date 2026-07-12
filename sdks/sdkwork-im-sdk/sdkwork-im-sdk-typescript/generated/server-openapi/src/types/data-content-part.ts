export interface DataContentPart {
  kind: 'data';
  schemaRef: string;
  encoding: string;
  payload: string;
}

import type { StringMap } from './string-map';

export interface AppendAgentResponseDeltaRequest {
  frameSeq: string;
  frameType: string;
  schemaRef?: string;
  encoding: string;
  payload: string;
  attributes?: StringMap;
}

import type { Sender } from './sender';
import type { StringMap } from './string-map';

export interface StreamFrame {
  tenantId: string;
  streamId: string;
  streamType: string;
  scopeKind: string;
  scopeId: string;
  frameSeq: string;
  frameType: string;
  schemaRef?: string;
  encoding: string;
  payload: string;
  sender: Sender;
  attributes: StringMap;
  occurredAt: string;
}

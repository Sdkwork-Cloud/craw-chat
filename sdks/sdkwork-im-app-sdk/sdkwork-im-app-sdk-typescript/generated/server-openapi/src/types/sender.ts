import type { StringMap } from './string-map';

export interface Sender {
  id: string;
  kind: string;
  memberId?: string;
  deviceId?: string;
  sessionId?: string;
  metadata: StringMap;
}

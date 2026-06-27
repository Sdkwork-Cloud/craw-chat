import type { StringMap } from './string-map';

export interface AgentSubject {
  agent_id: string;
  session_id?: string;
  metadata: StringMap;
}

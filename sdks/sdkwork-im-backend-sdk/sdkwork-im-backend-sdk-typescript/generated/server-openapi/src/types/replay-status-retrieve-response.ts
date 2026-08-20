import type { JournalReplayStatusView } from './journal-replay-status-view';

export interface ReplayStatusRetrieveResponse {
  code: 0;
  data: unknown & { item: JournalReplayStatusView; };
  /** Server-owned request correlation id. */
  traceId: string;
}

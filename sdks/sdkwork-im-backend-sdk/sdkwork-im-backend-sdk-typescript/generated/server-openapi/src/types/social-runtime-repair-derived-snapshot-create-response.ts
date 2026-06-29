import type { SocialRuntimeRepairResponse } from './social-runtime-repair-response';

export interface SocialRuntimeRepairDerivedSnapshotCreateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}

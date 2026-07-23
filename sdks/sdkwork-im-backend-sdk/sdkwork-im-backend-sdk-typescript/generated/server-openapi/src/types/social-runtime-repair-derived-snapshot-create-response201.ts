import type { SocialRuntimeRepairResponse } from './social-runtime-repair-response';

export interface SocialRuntimeRepairDerivedSnapshotCreateResponse201 {
  code: 0;
  data: unknown & { item: SocialRuntimeRepairResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}

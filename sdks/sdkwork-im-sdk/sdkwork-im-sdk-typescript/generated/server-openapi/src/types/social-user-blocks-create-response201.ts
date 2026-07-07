import type { OpenApiUserBlockResponse } from './open-api-user-block-response';

export interface SocialUserBlocksCreateResponse201 {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}

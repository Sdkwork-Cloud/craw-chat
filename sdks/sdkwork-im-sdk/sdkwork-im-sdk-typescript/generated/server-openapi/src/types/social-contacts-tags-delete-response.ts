import type { DeleteContactTagResponse } from './delete-contact-tag-response';

export interface SocialContactsTagsDeleteResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}

import type { ContactTagView } from './contact-tag-view';

export interface SocialContactsTagsCreateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}

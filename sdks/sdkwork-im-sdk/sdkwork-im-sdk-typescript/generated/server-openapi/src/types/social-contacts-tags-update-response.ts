import type { ContactTagView } from './contact-tag-view';

export interface SocialContactsTagsUpdateResponse {
  code: 0;
  data: unknown & { item: ContactTagView; };
  /** Server-owned request correlation id. */
  traceId: string;
}

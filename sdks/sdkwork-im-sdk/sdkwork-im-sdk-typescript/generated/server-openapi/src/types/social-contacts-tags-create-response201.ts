import type { ContactTagView } from './contact-tag-view';

export interface SocialContactsTagsCreateResponse201 {
  code: 0;
  data: unknown & { item: ContactTagView; };
  /** Server-owned request correlation id. */
  traceId: string;
}

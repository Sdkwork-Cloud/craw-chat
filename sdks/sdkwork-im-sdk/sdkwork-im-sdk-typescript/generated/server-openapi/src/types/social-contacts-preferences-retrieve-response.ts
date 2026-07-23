import type { ContactPreferencesView } from './contact-preferences-view';

export interface SocialContactsPreferencesRetrieveResponse {
  code: 0;
  data: unknown & { item: ContactPreferencesView; };
  /** Server-owned request correlation id. */
  traceId: string;
}

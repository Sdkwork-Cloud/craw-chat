import type { ContactPreferencesView } from './contact-preferences-view';

export interface SocialContactsPreferencesUpdateResponse {
  code: 0;
  data: unknown & { item: ContactPreferencesView; };
  /** Server-owned request correlation id. */
  traceId: string;
}

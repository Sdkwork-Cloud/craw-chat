import type { ContactView } from './contact-view';

export interface ContactsResponse {
  items: ContactView[];
  nextCursor?: string | null;
  hasMore: boolean;
}

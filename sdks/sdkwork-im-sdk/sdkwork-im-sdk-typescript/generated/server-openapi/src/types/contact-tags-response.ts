import type { ContactTagView } from './contact-tag-view';

export interface ContactTagsResponse {
  items: ContactTagView[];
  nextCursor?: string | null;
  hasMore: boolean;
}

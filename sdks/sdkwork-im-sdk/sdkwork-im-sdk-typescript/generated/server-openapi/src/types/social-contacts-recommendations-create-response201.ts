import type { ContactRecommendationView } from './contact-recommendation-view';

export interface SocialContactsRecommendationsCreateResponse201 {
  code: 0;
  data: unknown & { item: ContactRecommendationView; };
  /** Server-owned request correlation id. */
  traceId: string;
}

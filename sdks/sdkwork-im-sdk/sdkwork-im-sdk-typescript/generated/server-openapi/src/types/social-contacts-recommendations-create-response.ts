import type { ContactRecommendationView } from './contact-recommendation-view';

export interface SocialContactsRecommendationsCreateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}

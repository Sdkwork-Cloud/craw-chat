import type { WelcomeEnsureView } from './welcome-ensure-view';

export interface ChatMeWelcomeEnsureResponse {
  code: 0;
  data: unknown & { item: WelcomeEnsureView; };
  /** Server-owned request correlation id. */
  traceId: string;
}

/**
 * PC login-time system-agent welcome delivery.
 *
 * Aligned with the H5 surface (`AuthGate` → `ensureChatWelcomeMessage`):
 * once an authenticated session is established, `POST
 * /im/v3/api/chat/me/welcome/ensure` is invoked idempotently. The
 * conversation service deduplicates through a deterministic
 * `client_msg_id = "welcome:{user_id}"` plus a persisted welcome marker, so
 * repeated calls (refresh, re-login, token rotation) never deliver a second
 * welcome message.
 *
 * On success (sent or already existing) a `SDKWORK_IM_INBOX_REFRESH_EVENT`
 * is dispatched so in-place chat pages reload their first inbox page — the
 * welcome conversation is created asynchronously after login, so the initial
 * page load may have missed it, and the new conversation is not part of the
 * realtime subscription set yet.
 *
 * Failures are logged but never thrown: the welcome message must not block
 * the login flow, and the next session change or page refresh retries the
 * idempotent endpoint.
 */

import { getImSdkClientWithSession } from './imSdkClient';
import { isAppSdkSessionAuthenticated, readAppSdkSessionTokens } from './session';

/** Dispatched by `ensurePcWelcomeMessage` when the welcome conversation is ready. */
export const SDKWORK_IM_INBOX_REFRESH_EVENT = 'sdkwork-im-pc:inbox-refresh';

function notifyInboxRefresh(): void {
  if (typeof window === 'undefined') {
    return;
  }
  window.dispatchEvent(new CustomEvent(SDKWORK_IM_INBOX_REFRESH_EVENT));
}

export async function ensurePcWelcomeMessage(): Promise<void> {
  const session = readAppSdkSessionTokens();
  if (!isAppSdkSessionAuthenticated(session)) {
    return;
  }
  try {
    const client = getImSdkClientWithSession(session);
    await client.chat.me.welcome.ensure();
    notifyInboxRefresh();
  } catch (error) {
    console.error('[sdkwork-im-pc] welcome/ensure failed', error);
  }
}

import { registerImH5SessionLogoutHandler } from '@sdkwork/im-h5-core/session';
import { getImAppAuthRuntime, resetImAppAuthRuntime } from './imAppAuthRuntime';
import { resetTokenManagerBinding } from './tokenManager';

/**
 * Binds the app-owned IAM logout executor to the shared session port.
 *
 * Feature packages request logout through `requestImH5SessionLogout`; this
 * handler performs the real flow:
 *
 * 1. `service.auth.sessions.current.delete()` revokes the server session and
 *    (in its `finally`) clears the local token store, context store and token
 *    manager through `runtime.clearSession()` — the session bridge then
 *    removes the persisted session and emits the session-changed event that
 *    lets AuthGate fall back to the login screen.
 * 2. The runtime composition and token-manager binding are dropped so no
 *    stale credential state survives across re-login.
 *
 * The caller owns the returned unregister function (see AuthGate).
 */
export function bindImH5SessionLogoutHandler(): () => void {
  return registerImH5SessionLogoutHandler(async () => {
    try {
      await getImAppAuthRuntime().runtime.service.auth.sessions.current.delete();
    } finally {
      resetImAppAuthRuntime();
      resetTokenManagerBinding();
    }
  });
}

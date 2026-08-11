import { useAppStore } from "@sdkwork/im-h5-core";
import { requestImH5SessionLogout } from "@sdkwork/im-h5-core/session";
import { UserCapabilityUnavailableError } from "./UserCapabilityUnavailableError";

/**
 * Fail-closed auth service.
 *
 * The real login / registration / verification-code / password-reset flows are
 * owned by `@sdkwork/iam-h5-auth` (wired in the app root AuthGate through
 * `createImH5AuthController` against the IAM app-api runtime). This legacy
 * service must never fabricate sessions, tokens, or verification codes — every
 * credential-flow method throws a typed `UserCapabilityUnavailableError` so
 * callers surface a typed unavailable state instead of fake success.
 *
 * `getCurrentUser` and `logout` are real: they read the authenticated IAM
 * current user from the app store (populated by AuthGate via
 * `iam.users.current.retrieve()`) and request the IAM session revoke through
 * the app-owned logout handler.
 */
export const AuthService = {
  async login(
    _phone: string,
    _password?: string,
    _code?: string,
  ): Promise<never> {
    throw new UserCapabilityUnavailableError("Password/code login");
  },

  async register(
    _phone: string,
    _code: string,
    _password?: string,
  ): Promise<never> {
    throw new UserCapabilityUnavailableError("Account registration");
  },

  async resetPassword(
    _phone: string,
    _code: string,
    _newPassword: string,
  ): Promise<never> {
    throw new UserCapabilityUnavailableError("Password reset");
  },

  async sendCode(_phone: string): Promise<never> {
    throw new UserCapabilityUnavailableError("Verification code delivery");
  },

  async logout(): Promise<void> {
    await requestImH5SessionLogout();
  },

  /**
   * Real IAM current user (id / name / avatar) as resolved by AuthGate from
   * `iamAppSdkClient.iam.users.current.retrieve()`. `null` when no session
   * exists or the profile has not been loaded yet.
   */
  getCurrentUser() {
    const user = useAppStore.getState().currentUser;
    if (!user) {
      return null;
    }
    return {
      id: user.id,
      name: user.name,
      ...(user.avatar ? { avatar: user.avatar } : {}),
    };
  },
};

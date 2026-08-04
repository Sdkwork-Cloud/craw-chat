import { UserCapabilityUnavailableError } from "./UserCapabilityUnavailableError";

export interface AuthUser {
  id: string;
  phone: string;
  token: string;
}

const AUTH_SESSION_STORAGE_KEY = "auth_user";

/**
 * Legacy user authentication is excluded from release pending IAM security
 * review (PRD §4 "Current H5 Delivery Boundary"). Root authentication goes
 * through the approved appbase IAM runtime.
 *
 * Every credential flow fails closed: no demo credentials, no locally minted
 * tokens, and no browser storage acting as a session authority. Callers
 * surface the capability error to the user instead of fabricating success.
 */
export const AuthService = {
  async login(
    _phone: string,
    _password?: string,
    _code?: string,
  ): Promise<AuthUser> {
    throw new UserCapabilityUnavailableError("legacy User auth");
  },

  async register(
    _phone: string,
    _code: string,
    _password?: string,
  ): Promise<AuthUser> {
    throw new UserCapabilityUnavailableError("legacy User registration");
  },

  async resetPassword(
    _phone: string,
    _code: string,
    _newPassword: string,
  ): Promise<boolean> {
    throw new UserCapabilityUnavailableError("legacy User password reset");
  },

  async sendCode(_phone: string): Promise<boolean> {
    throw new UserCapabilityUnavailableError("legacy User verification code");
  },

  async logout(): Promise<void> {
    // Best-effort cleanup of any stale locally persisted session residue;
    // this must never mint or validate a session.
    try {
      localStorage.removeItem(AUTH_SESSION_STORAGE_KEY);
    } catch {
      // storage may be unavailable; logout is still idempotent
    }
  },

  getCurrentUser(): AuthUser | null {
    // No browser storage may act as a session authority for this surface.
    return null;
  },
};

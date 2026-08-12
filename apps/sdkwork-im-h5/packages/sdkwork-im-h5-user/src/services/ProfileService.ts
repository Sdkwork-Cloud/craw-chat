import { getIamAppSdkClient } from "@sdkwork/im-h5-core/sdk";
import type { User } from "@sdkwork/im-h5-types";

import { UserCapabilityUnavailableError } from "./UserCapabilityUnavailableError";

export interface UserProfile extends User {
  wechatId: string;
  phone: string;
  gender: string;
  region: string;
  signature: string;
  beans: number;
}

const INITIAL_PROFILE: UserProfile = {
  id: "",
  name: "",
  avatar: "",
  status: "online",
  wechatId: "",
  phone: "",
  gender: "",
  region: "",
  signature: "",
  beans: 0,
};

export let CURRENT_USER_PROFILE: UserProfile = { ...INITIAL_PROFILE };

function readString(value: unknown): string | undefined {
  return typeof value === "string" && value.trim() ? value.trim() : undefined;
}

/** Maps the IAM current-user record onto the profile identity fields. */
function applyIamIdentity(profile: UserProfile, record: Record<string, unknown>): UserProfile {
  const id = readString(record.id) ?? readString(record.userId) ?? profile.id;
  const name = readString(record.displayName)
    ?? readString(record.nickname)
    ?? readString(record.name)
    ?? readString(record.username)
    ?? profile.name;
  return {
    ...profile,
    id,
    name,
    ...(readString(record.avatarUrl) ? { avatar: readString(record.avatarUrl) } : {}),
    ...(readString(record.email) ? { email: readString(record.email) } : {}),
    phone: readString(record.phoneNumber) ?? profile.phone,
    wechatId: readString(record.wechatId) ?? profile.wechatId,
  };
}

export const ProfileService = {
  async getUserProfile(): Promise<UserProfile> {
    const record = await getIamAppSdkClient().iam.users.current.retrieve();
    const profile = applyIamIdentity({ ...INITIAL_PROFILE }, record);
    CURRENT_USER_PROFILE = profile;
    return { ...profile };
  },

  async updateUserProfile(updates: Partial<UserProfile>): Promise<UserProfile> {
    // Wallet, contact, and preference fields have no composed owner SDK
    // surface; fail closed with a typed error before touching any network.
    const unsupportedFields = (["wechatId", "phone", "gender", "region", "signature", "beans"] as const)
      .filter((field) => updates[field] !== undefined);
    if (unsupportedFields.length > 0) {
      throw new UserCapabilityUnavailableError(`User profile ${unsupportedFields.join(", ")}`);
    }
    const current = await ProfileService.getUserProfile();
    if (updates.name && updates.name !== current.name) {
      await getIamAppSdkClient().iam.users.current.update({ displayName: updates.name });
    }
    const next: UserProfile = { ...current, ...updates };
    CURRENT_USER_PROFILE = next;
    return { ...next };
  },
};

/**
 * Wallet beans are owned by a payment/membership authority that is not
 * composed in the current H5 release; the profile surface must never
 * fabricate a balance or a recharge success.
 */
export function requireWalletCapability(): never {
  throw new UserCapabilityUnavailableError("User wallet beans");
}

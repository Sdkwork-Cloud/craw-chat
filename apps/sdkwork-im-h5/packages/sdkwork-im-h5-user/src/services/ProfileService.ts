import { getIamAppSdkClient } from "@sdkwork/im-h5-core/sdk";
import type { User } from "@sdkwork/im-h5-types";

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

const STORAGE_KEY = "sdkwork_im_h5_user_profile";

export let CURRENT_USER_PROFILE: UserProfile = { ...INITIAL_PROFILE };

function readString(value: unknown): string | undefined {
  return typeof value === "string" && value.trim() ? value.trim() : undefined;
}

const loadLocalProfile = (): Partial<UserProfile> => {
  try {
    const data = localStorage.getItem(STORAGE_KEY) ?? localStorage.getItem("clawchat_user_profile");
    if (data) {
      return JSON.parse(data) as Partial<UserProfile>;
    }
  } catch (e) {
    console.error("Failed to load profile", e);
  }
  return {};
};

const saveLocalProfile = (profile: Partial<UserProfile>) => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(profile));
  } catch (e) {
    console.error("Failed to save profile", e);
  }
};

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
    const local = loadLocalProfile();
    const merged: UserProfile = { ...INITIAL_PROFILE, ...local };
    try {
      const record = await getIamAppSdkClient().iam.users.current.retrieve();
      const profile = applyIamIdentity(merged, record);
      CURRENT_USER_PROFILE = profile;
      return { ...profile };
    } catch (error) {
      console.error("Failed to load IAM profile", error);
      CURRENT_USER_PROFILE = merged;
      return { ...merged };
    }
  },

  async updateUserProfile(updates: Partial<UserProfile>): Promise<UserProfile> {
    const current = await ProfileService.getUserProfile();
    const next: UserProfile = { ...current, ...updates };
    CURRENT_USER_PROFILE = next;
    saveLocalProfile({
      id: next.id,
      name: next.name,
      avatar: next.avatar,
      wechatId: next.wechatId,
      phone: next.phone,
      gender: next.gender,
      region: next.region,
      signature: next.signature,
      beans: next.beans,
    });
    // Persist the display name on the IAM user record when it changed.
    if (updates.name && updates.name !== current.name) {
      try {
        await getIamAppSdkClient().iam.users.current.update({ displayName: updates.name });
      } catch (error) {
        console.error("Failed to update IAM display name", error);
      }
    }
    return { ...next };
  },
};

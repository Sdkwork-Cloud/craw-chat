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

export const ProfileService = {
  async getUserProfile(): Promise<UserProfile> {
    throw new UserCapabilityUnavailableError("User profile");
  },

  async updateUserProfile(_updates: Partial<UserProfile>): Promise<UserProfile> {
    throw new UserCapabilityUnavailableError("User profile");
  },
};

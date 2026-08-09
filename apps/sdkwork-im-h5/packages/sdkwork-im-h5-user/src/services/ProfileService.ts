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
  id: "u1",
  name: "Alex Chen",
  avatar: "https://picsum.photos/seed/alex/200/200",
  status: "online",
  wechatId: "wxid_123456789",
  phone: "138****8888",
  gender: "男",
  region: "北京 海淀",
  signature: "永远年轻，永远热泪盈眶。",
  beans: 120,
};

const STORAGE_KEY = "clawchat_user_profile";

export let CURRENT_USER_PROFILE: UserProfile = { ...INITIAL_PROFILE };

const loadProfile = () => {
  try {
    const data = localStorage.getItem(STORAGE_KEY);
    if (data) {
      CURRENT_USER_PROFILE = JSON.parse(data);
    } else {
      CURRENT_USER_PROFILE = { ...INITIAL_PROFILE };
    }
  } catch (e) {
    CURRENT_USER_PROFILE = { ...INITIAL_PROFILE };
  }
  return CURRENT_USER_PROFILE;
};

const saveProfile = () => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(CURRENT_USER_PROFILE));
  } catch (e) {
    console.error("Failed to save profile", e);
  }
};

loadProfile();

export const ProfileService = {
  async getUserProfile(): Promise<UserProfile> {
    return { ...loadProfile() };
  },

  async updateUserProfile(updates: Partial<UserProfile>): Promise<UserProfile> {
    loadProfile();
    const newProfile = { ...CURRENT_USER_PROFILE, ...updates };
    CURRENT_USER_PROFILE = newProfile;
    saveProfile();
    return { ...newProfile };
  },
};

import { create } from "zustand";
import type { User } from "@sdkwork/im-h5-types";

interface AppState {
  currentUser: User | null;
  setCurrentUser: (user: User | null) => void;
}

export const useAppStore = create<AppState>((set) => ({
  currentUser: {
    id: "u1",
    name: "Alex Chen",
    avatar: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/alex/200x200.png",
    status: "online",
  },
  setCurrentUser: (user) => set({ currentUser: user }),
}));

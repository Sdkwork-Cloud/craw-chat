import { create } from "zustand";
import type { ImH5SessionUser } from "../session";

interface AppState {
  currentUser: ImH5SessionUser | null;
  setCurrentUser: (user: ImH5SessionUser | null) => void;
}

export const useAppStore = create<AppState>((set) => ({
  currentUser: null,
  setCurrentUser: (user) => set({ currentUser: user }),
}));

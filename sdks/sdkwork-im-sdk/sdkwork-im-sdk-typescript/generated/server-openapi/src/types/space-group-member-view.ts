export interface SpaceGroupMemberView {
  userId: string;
  role: string;
  nickname?: string | null;
  muteUntil?: string;
  joinedAt: string;
}

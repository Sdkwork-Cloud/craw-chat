export interface SpaceInviteView {
  /** Invitation identifier, passed as the inviteCode path parameter. */
  invitationId: string;
  inviterUserId: string;
  inviteeUserId?: string | null;
  targetType: string;
  targetId: string;
  role: string;
  status: 'pending' | 'accepted' | 'declined' | 'expired' | 'canceled';
  createdAt: string;
}

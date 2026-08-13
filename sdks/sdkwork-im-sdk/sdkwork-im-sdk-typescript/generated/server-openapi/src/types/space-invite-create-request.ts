export interface SpaceInviteCreateRequest {
  inviteeUserId?: string;
  inviteeEmail?: string;
  inviteePhone?: string;
  targetType: 'space';
  targetId: string;
  role?: 'admin' | 'member' | 'guest';
  message?: string | null;
  expiresAt?: string | null;
}

/** Invitation to join the space. At least one of inviteeUserId, inviteeEmail, or inviteePhone is required. */
export interface SpaceInviteCreateRequest {
  /** Invitee user id when the invitation is addressed to a registered user. */
  inviteeUserId?: string;
  /** Invitee email when the invitation is delivered by email. */
  inviteeEmail?: string;
  /** Invitee phone when the invitation is delivered by phone. */
  inviteePhone?: string;
  /** Invitation target type. Only space invitations are supported. */
  targetType: 'space';
  /** Invitation target id, must equal the spaceId path parameter. */
  targetId: string;
  /** Role granted to the invitee on acceptance. Defaults to member. */
  role?: 'admin' | 'member' | 'guest';
  /** Optional personal message shown to the invitee. */
  message?: string | null;
  /** RFC3339 expiry instant. Must be in the future when provided. */
  expiresAt?: string | null;
}

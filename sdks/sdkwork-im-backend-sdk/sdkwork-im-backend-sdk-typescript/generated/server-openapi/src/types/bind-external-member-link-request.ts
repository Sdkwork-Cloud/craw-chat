export interface BindExternalMemberLinkRequest {
  connectionId: string;
  eventId: string;
  externalDisplayName?: string | null;
  externalMemberId: string;
  linkId: string;
  linkedAt: string;
  localActorId: string;
  localActorKind: string;
}

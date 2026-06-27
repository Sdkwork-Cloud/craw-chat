export interface EstablishExternalConnectionRequest {
  connectionId: string;
  connectionKind: string;
  establishedAt: string;
  eventId: string;
  externalOrgName?: string | null;
  externalTenantId: string;
}

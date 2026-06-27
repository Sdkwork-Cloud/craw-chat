export interface QuotaProfileResponse {
  maxConcurrentSessionsPerTenant: string;
  maxInflightMessages: string;
  maxPayloadBytes: string;
  maxSubscriptionsPerSession: string;
  profileId: string;
}

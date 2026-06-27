export interface ImH5AppSession {
  accessToken: string;
  authToken: string;
  tenantId: string;
  organizationId: string;
  userId: string;
}

export const DEFAULT_APP_SESSION: ImH5AppSession = {
  accessToken: "dev-access-token",
  authToken: "dev-auth-token",
  tenantId: "100001",
  organizationId: "0",
  userId: "user",
};

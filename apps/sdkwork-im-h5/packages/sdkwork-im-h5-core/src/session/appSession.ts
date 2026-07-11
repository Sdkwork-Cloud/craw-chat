export interface ImH5AppSession {
  accessToken: string;
  authToken: string;
  tenantId: string;
  organizationId: string;
  userId: string;
}

export const DEFAULT_APP_SESSION: ImH5AppSession = {
  accessToken: "",
  authToken: "",
  tenantId: "",
  organizationId: "",
  userId: "",
};

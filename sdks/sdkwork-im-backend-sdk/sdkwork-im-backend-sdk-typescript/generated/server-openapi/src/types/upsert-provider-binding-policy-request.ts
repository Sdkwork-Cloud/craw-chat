export interface UpsertProviderBindingPolicyRequest {
  domain: string;
  expectedBaseVersion?: string | null;
  pluginId: string;
  tenantId?: string | null;
}

export interface ProviderBindingItem {
  domain: string;
  defaultPluginId: string | null;
  selectedPluginId: string | null;
  selectionSource: string;
  tenantOverrideAllowed: boolean;
}

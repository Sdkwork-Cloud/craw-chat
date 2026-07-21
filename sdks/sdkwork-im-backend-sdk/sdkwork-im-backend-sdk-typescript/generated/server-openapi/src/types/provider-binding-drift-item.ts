export interface ProviderBindingDriftItem {
  tenantId: string;
  domain: string;
  baselineSelectedPluginId: string | null;
  selectedPluginId: string | null;
  baselineSelectionSource: string;
  selectionSource: string;
  driftKind: string;
}

package types


type ProviderBindingDriftItem struct {
	TenantId string `json:"tenantId"`
	Domain string `json:"domain"`
	BaselineSelectedPluginId string `json:"baselineSelectedPluginId"`
	SelectedPluginId string `json:"selectedPluginId"`
	BaselineSelectionSource string `json:"baselineSelectionSource"`
	SelectionSource string `json:"selectionSource"`
	DriftKind string `json:"driftKind"`
}

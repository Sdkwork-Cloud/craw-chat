package types


type ProviderBindingItem struct {
	Domain string `json:"domain"`
	DefaultPluginId string `json:"defaultPluginId"`
	SelectedPluginId string `json:"selectedPluginId"`
	SelectionSource string `json:"selectionSource"`
	TenantOverrideAllowed bool `json:"tenantOverrideAllowed"`
}

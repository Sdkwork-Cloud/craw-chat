package types


type UpsertProviderBindingPolicyRequest struct {
	Domain string `json:"domain"`
	ExpectedBaseVersion int `json:"expectedBaseVersion"`
	PluginId string `json:"pluginId"`
	TenantId string `json:"tenantId"`
}

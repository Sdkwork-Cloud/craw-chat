package types


type UpsertProviderBindingPolicyRequest struct {
	Domain string `json:"domain"`
	ExpectedBaseVersion string `json:"expectedBaseVersion"`
	PluginId string `json:"pluginId"`
	TenantId string `json:"tenantId"`
}

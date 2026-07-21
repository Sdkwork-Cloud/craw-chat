package types


type ProviderBindingSnapshot struct {
	InterfaceVersion string `json:"interfaceVersion"`
	TenantId string `json:"tenantId"`
	EffectiveBindings []ProviderBindingItem `json:"effectiveBindings"`
	Precedence []string `json:"precedence"`
}

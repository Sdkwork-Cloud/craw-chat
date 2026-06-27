package types


type ProtocolSchemaResponse struct {
	BindingProtocols []string `json:"bindingProtocols"`
	Kind string `json:"kind"`
	RequiredCapabilities []string `json:"requiredCapabilities"`
	Schema string `json:"schema"`
	Stage string `json:"stage"`
	SupportedConsumers []string `json:"supportedConsumers"`
}

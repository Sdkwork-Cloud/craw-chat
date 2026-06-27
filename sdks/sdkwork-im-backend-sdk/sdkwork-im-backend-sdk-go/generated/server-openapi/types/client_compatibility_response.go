package types


type ClientCompatibilityResponse struct {
	BlockedExperimentalCapabilities []string `json:"blockedExperimentalCapabilities"`
	ClientType string `json:"clientType"`
	MinimumProtocolVersion string `json:"minimumProtocolVersion"`
	SupportedBindings []string `json:"supportedBindings"`
	SupportedCapabilities []string `json:"supportedCapabilities"`
	SupportedCodecs []string `json:"supportedCodecs"`
}

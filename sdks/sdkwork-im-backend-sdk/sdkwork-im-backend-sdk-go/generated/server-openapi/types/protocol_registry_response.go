package types


type ProtocolRegistryResponse struct {
	Bindings []string `json:"bindings"`
	Codecs []string `json:"codecs"`
	CompatibilityMatrix []ClientCompatibilityResponse `json:"compatibilityMatrix"`
	ProtocolVersion string `json:"protocolVersion"`
	Schemas []ProtocolSchemaResponse `json:"schemas"`
}

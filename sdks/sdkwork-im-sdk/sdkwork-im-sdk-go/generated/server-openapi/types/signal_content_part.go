package types


type SignalContentPart struct {
	Kind string `json:"kind"`
	SignalType string `json:"signalType"`
	SchemaRef string `json:"schemaRef"`
	Payload string `json:"payload"`
}

package types


type DataContentPart struct {
	Kind string `json:"kind"`
	SchemaRef string `json:"schemaRef"`
	Encoding string `json:"encoding"`
	Payload string `json:"payload"`
}

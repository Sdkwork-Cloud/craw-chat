package types


type StreamFrame struct {
	TenantId string `json:"tenantId"`
	StreamId string `json:"streamId"`
	StreamType string `json:"streamType"`
	ScopeKind string `json:"scopeKind"`
	ScopeId string `json:"scopeId"`
	FrameSeq int `json:"frameSeq"`
	FrameType string `json:"frameType"`
	SchemaRef string `json:"schemaRef"`
	Encoding string `json:"encoding"`
	Payload string `json:"payload"`
	Sender Sender `json:"sender"`
	Attributes StringMap `json:"attributes"`
	OccurredAt string `json:"occurredAt"`
}

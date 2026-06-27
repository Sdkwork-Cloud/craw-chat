package types


type StreamSession struct {
	TenantId string `json:"tenantId"`
	StreamId string `json:"streamId"`
	StreamType string `json:"streamType"`
	ScopeKind string `json:"scopeKind"`
	ScopeId string `json:"scopeId"`
	DurabilityClass StreamDurabilityClass `json:"durabilityClass"`
	OrderingScope string `json:"orderingScope"`
	SchemaRef string `json:"schemaRef"`
	State StreamSessionState `json:"state"`
	LastFrameSeq int `json:"lastFrameSeq"`
	LastCheckpointSeq int `json:"lastCheckpointSeq"`
	ResultMessageId string `json:"resultMessageId"`
	OpenedAt string `json:"openedAt"`
	ClosedAt string `json:"closedAt"`
	ExpiresAt string `json:"expiresAt"`
}

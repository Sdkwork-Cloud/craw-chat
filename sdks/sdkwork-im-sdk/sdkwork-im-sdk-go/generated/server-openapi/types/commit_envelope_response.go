package types


type CommitEnvelopeResponse struct {
	EventId string `json:"eventId"`
	TenantId string `json:"tenantId"`
	EventType string `json:"eventType"`
	EventVersion int `json:"eventVersion"`
	AggregateType string `json:"aggregateType"`
	AggregateId string `json:"aggregateId"`
	ScopeType string `json:"scopeType"`
	ScopeId string `json:"scopeId"`
	OrderingKey string `json:"orderingKey"`
	OrderingSeq int `json:"orderingSeq"`
	CausationId string `json:"causationId"`
	CorrelationId string `json:"correlationId"`
	IdempotencyKey string `json:"idempotencyKey"`
	Actor EventActor `json:"actor"`
	OccurredAt string `json:"occurredAt"`
	CommittedAt string `json:"committedAt"`
	PayloadSchema string `json:"payloadSchema"`
	Payload string `json:"payload"`
	RetentionClass string `json:"retentionClass"`
	AuditClass string `json:"auditClass"`
}

package types


type RtcSignalEvent struct {
	TenantId string `json:"tenantId"`
	RtcSessionId string `json:"rtcSessionId"`
	SignalSeq int `json:"signalSeq"`
	ConversationId string `json:"conversationId"`
	RtcMode string `json:"rtcMode"`
	SignalType string `json:"signalType"`
	SchemaRef string `json:"schemaRef"`
	Payload string `json:"payload"`
	Sender RtcSignalSender `json:"sender"`
	SignalingStreamId string `json:"signalingStreamId"`
	OccurredAt string `json:"occurredAt"`
}

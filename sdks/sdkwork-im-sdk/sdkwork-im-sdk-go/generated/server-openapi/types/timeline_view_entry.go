package types


type TimelineViewEntry struct {
	TenantId string `json:"tenantId"`
	ConversationId string `json:"conversationId"`
	MessageId string `json:"messageId"`
	MessageSeq int `json:"messageSeq"`
	Summary string `json:"summary"`
	Sender Sender `json:"sender"`
	Body MessageBody `json:"body"`
	MessageType MessageType `json:"messageType"`
	DeliveryMode string `json:"deliveryMode"`
	ClientMsgId string `json:"clientMsgId"`
	StreamSessionId string `json:"streamSessionId"`
	RtcSessionId string `json:"rtcSessionId"`
	OccurredAt string `json:"occurredAt"`
	CommittedAt string `json:"committedAt"`
}

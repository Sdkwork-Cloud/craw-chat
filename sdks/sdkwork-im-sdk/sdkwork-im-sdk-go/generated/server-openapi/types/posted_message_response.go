package types


type PostedMessageResponse struct {
	ConversationId string `json:"conversationId"`
	MessageId string `json:"messageId"`
	MessageSeq int `json:"messageSeq"`
	Body MessageBody `json:"body"`
	OccurredAt string `json:"occurredAt"`
}

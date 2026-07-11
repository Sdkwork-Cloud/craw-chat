package types


type MessageMutationResult struct {
	ConversationId string `json:"conversationId"`
	MessageId string `json:"messageId"`
	MessageSeq int `json:"messageSeq"`
	EventId string `json:"eventId"`
}

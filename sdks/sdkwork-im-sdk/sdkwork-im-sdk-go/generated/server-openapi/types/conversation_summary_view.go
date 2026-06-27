package types


type ConversationSummaryView struct {
	TenantId string `json:"tenantId"`
	ConversationId string `json:"conversationId"`
	MessageCount int `json:"messageCount"`
	LastMessageSeq int `json:"lastMessageSeq"`
	LastSummary string `json:"lastSummary"`
	LastMessageAt string `json:"lastMessageAt"`
}

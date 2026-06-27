package types


type ConversationInboxEntry struct {
	TenantId string `json:"tenantId"`
	ConversationId string `json:"conversationId"`
	AgentHandoff bool `json:"agentHandoff"`
	ConversationType string `json:"conversationType"`
	DisplayName string `json:"displayName"`
	AvatarUrl string `json:"avatarUrl"`
	DisplaySource string `json:"displaySource"`
	Peer ConversationInboxPeerView `json:"peer"`
	Preferences ConversationInboxPreferencesView `json:"preferences"`
	LastActivityAt string `json:"lastActivityAt"`
	LastMessageId string `json:"lastMessageId"`
	LastSenderId string `json:"lastSenderId"`
	MessageCount int `json:"messageCount"`
	LastMessageSeq int `json:"lastMessageSeq"`
	LastSummary string `json:"lastSummary"`
	LastMessageAt string `json:"lastMessageAt"`
	UnreadCount int `json:"unreadCount"`
}

package types


type InboxResponse struct {
	Items []ConversationInboxEntry `json:"items"`
	NextCursor string `json:"nextCursor"`
	HasMore bool `json:"hasMore"`
}

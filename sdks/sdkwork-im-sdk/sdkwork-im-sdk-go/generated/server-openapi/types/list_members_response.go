package types


type ListMembersResponse struct {
	Items []ConversationMember `json:"items"`
	NextCursor string `json:"nextCursor"`
	HasMore bool `json:"hasMore"`
}

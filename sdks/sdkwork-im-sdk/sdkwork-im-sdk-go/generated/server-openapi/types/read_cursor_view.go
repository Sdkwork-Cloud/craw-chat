package types


type ReadCursorView struct {
	TenantId string `json:"tenantId"`
	ConversationId string `json:"conversationId"`
	PrincipalId string `json:"principalId"`
	ReadSeq int `json:"readSeq"`
	UpdatedAt string `json:"updatedAt"`
}

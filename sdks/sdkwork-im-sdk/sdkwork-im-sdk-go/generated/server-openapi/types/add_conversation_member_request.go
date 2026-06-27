package types


type AddConversationMemberRequest struct {
	PrincipalId string `json:"principalId"`
	PrincipalKind string `json:"principalKind"`
	Role string `json:"role"`
	Attributes map[string]interface{} `json:"attributes"`
}

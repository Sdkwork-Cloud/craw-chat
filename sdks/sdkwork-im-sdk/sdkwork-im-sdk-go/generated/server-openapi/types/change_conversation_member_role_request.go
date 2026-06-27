package types


type ChangeConversationMemberRoleRequest struct {
	MemberId string `json:"memberId"`
	Role string `json:"role"`
}

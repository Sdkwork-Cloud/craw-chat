package types


type RoomView struct {
	RoomId string `json:"roomId"`
	RoomKind string `json:"roomKind"`
	ConversationId string `json:"conversationId"`
	ActiveMemberCount int `json:"activeMemberCount"`
	MaxMembers int `json:"maxMembers"`
}

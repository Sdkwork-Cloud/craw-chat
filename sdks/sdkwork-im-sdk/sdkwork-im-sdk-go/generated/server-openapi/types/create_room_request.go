package types


type CreateRoomRequest struct {
	ConversationId string `json:"conversationId"`
	RoomId string `json:"roomId"`
	RoomKind string `json:"roomKind"`
}

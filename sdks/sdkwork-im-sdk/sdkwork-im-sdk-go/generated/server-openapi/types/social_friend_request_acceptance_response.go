package types


type SocialFriendRequestAcceptanceResponse struct {
	FriendRequest FriendRequest `json:"friendRequest"`
	Friendship Friendship `json:"friendship"`
	DirectChat DirectChat `json:"directChat"`
	Conversation CreateConversationResult `json:"conversation"`
}

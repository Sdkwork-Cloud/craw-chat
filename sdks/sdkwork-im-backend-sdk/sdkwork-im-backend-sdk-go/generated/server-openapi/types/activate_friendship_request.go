package types


type ActivateFriendshipRequest struct {
	DirectChatId string `json:"directChatId"`
	EstablishedAt string `json:"establishedAt"`
	EventId string `json:"eventId"`
	FriendshipId string `json:"friendshipId"`
	InitiatorUserId string `json:"initiatorUserId"`
	PeerUserId string `json:"peerUserId"`
}

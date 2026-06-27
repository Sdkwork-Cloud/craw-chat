package types


type ContactView struct {
	TenantId string `json:"tenantId"`
	OwnerUserId string `json:"ownerUserId"`
	TargetUserId string `json:"targetUserId"`
	ContactType string `json:"contactType"`
	RelationshipState string `json:"relationshipState"`
	FriendshipId string `json:"friendshipId"`
	DirectChatId string `json:"directChatId"`
	ConversationId string `json:"conversationId"`
	EstablishedAt string `json:"establishedAt"`
	LastInteractionAt string `json:"lastInteractionAt"`
}

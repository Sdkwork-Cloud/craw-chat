package types


type ContactView struct {
	TenantId string `json:"tenantId"`
	OwnerUserId string `json:"ownerUserId"`
	TargetUserId string `json:"targetUserId"`
	DisplayName string `json:"displayName"`
	AvatarUrl string `json:"avatarUrl"`
	ChatId string `json:"chatId"`
	ContactType string `json:"contactType"`
	RelationshipState string `json:"relationshipState"`
	FriendshipId string `json:"friendshipId"`
	DirectChatId string `json:"directChatId"`
	ConversationId string `json:"conversationId"`
	EstablishedAt string `json:"establishedAt"`
	LastInteractionAt string `json:"lastInteractionAt"`
	IsStarred bool `json:"isStarred"`
	IsBlocked bool `json:"isBlocked"`
	Remark string `json:"remark"`
	UpdatedAt string `json:"updatedAt"`
}

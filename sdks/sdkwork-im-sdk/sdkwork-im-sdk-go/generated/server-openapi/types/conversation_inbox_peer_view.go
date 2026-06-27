package types


type ConversationInboxPeerView struct {
	PrincipalKind string `json:"principalKind"`
	PrincipalId string `json:"principalId"`
	UserId string `json:"userId"`
	ChatId string `json:"chatId"`
	DisplayName string `json:"displayName"`
	AvatarUrl string `json:"avatarUrl"`
	RelationshipState string `json:"relationshipState"`
}

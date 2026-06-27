package types


type MessageFavoriteView struct {
	TenantId string `json:"tenantId"`
	PrincipalKind string `json:"principalKind"`
	PrincipalId string `json:"principalId"`
	FavoriteId string `json:"favoriteId"`
	FavoriteType MessageFavoriteType `json:"favoriteType"`
	ConversationId string `json:"conversationId"`
	MessageId string `json:"messageId"`
	MessageSeq int `json:"messageSeq"`
	Title string `json:"title"`
	ContentPreview string `json:"contentPreview"`
	SourceDisplayName string `json:"sourceDisplayName"`
	FavoritedAt string `json:"favoritedAt"`
}

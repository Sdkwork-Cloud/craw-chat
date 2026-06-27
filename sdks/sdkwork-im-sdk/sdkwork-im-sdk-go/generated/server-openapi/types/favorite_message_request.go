package types


type FavoriteMessageRequest struct {
	ConversationId string `json:"conversationId"`
	FavoriteType MessageFavoriteType `json:"favoriteType"`
	Title string `json:"title"`
	ContentPreview string `json:"contentPreview"`
	SourceDisplayName string `json:"sourceDisplayName"`
}

package types


type FavoriteMessagesResponse struct {
	Items []MessageFavoriteView `json:"items"`
	NextCursor string `json:"nextCursor"`
	HasMore bool `json:"hasMore"`
}

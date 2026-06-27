package types


type DeleteMessageFavoriteResponse struct {
	FavoriteId string `json:"favoriteId"`
	Deleted bool `json:"deleted"`
}

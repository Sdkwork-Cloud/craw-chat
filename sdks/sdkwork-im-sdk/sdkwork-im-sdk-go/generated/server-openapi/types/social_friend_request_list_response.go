package types


type SocialFriendRequestListResponse struct {
	Items []FriendRequest `json:"items"`
	NextCursor string `json:"nextCursor"`
}

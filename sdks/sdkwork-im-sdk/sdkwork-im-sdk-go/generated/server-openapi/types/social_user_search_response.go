package types


type SocialUserSearchResponse struct {
	Items []SocialUserSearchResult `json:"items"`
	NextCursor string `json:"nextCursor"`
	HasMore bool `json:"hasMore"`
}

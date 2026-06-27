package types


type ContactTagsResponse struct {
	Items []ContactTagView `json:"items"`
	NextCursor string `json:"nextCursor"`
	HasMore bool `json:"hasMore"`
}

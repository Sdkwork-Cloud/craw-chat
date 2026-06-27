package types


type ContactsResponse struct {
	Items []ContactView `json:"items"`
	NextCursor string `json:"nextCursor"`
	HasMore bool `json:"hasMore"`
}

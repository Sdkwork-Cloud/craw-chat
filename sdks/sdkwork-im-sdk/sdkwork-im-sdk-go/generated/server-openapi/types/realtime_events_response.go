package types


type RealtimeEventsResponse struct {
	Items []RealtimeEventView `json:"items"`
	NextCursor string `json:"nextCursor"`
	HasMore bool `json:"hasMore"`
}

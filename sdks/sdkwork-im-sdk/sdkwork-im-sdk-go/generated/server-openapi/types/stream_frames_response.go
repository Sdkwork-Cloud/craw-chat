package types


type StreamFramesResponse struct {
	Items []StreamFrameView `json:"items"`
	NextCursor string `json:"nextCursor"`
	HasMore bool `json:"hasMore"`
}

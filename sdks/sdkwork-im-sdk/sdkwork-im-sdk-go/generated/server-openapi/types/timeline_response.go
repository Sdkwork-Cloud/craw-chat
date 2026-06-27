package types


type TimelineResponse struct {
	Items []TimelineViewEntry `json:"items"`
	NextAfterSeq int `json:"nextAfterSeq"`
	HasMore bool `json:"hasMore"`
}

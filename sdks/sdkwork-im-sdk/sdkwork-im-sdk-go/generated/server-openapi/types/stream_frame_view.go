package types


type StreamFrameView struct {
	StreamId string `json:"streamId"`
	FrameSeq int `json:"frameSeq"`
	Payload string `json:"payload"`
	CreatedAt string `json:"createdAt"`
}

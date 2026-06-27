package types


type StreamRefContentPart struct {
	Kind string `json:"kind"`
	StreamId string `json:"streamId"`
	StreamType string `json:"streamType"`
	State string `json:"state"`
}

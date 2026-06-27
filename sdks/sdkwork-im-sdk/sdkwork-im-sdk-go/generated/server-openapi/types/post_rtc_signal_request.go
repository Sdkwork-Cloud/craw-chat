package types


type PostRtcSignalRequest struct {
	SignalType string `json:"signalType"`
	SchemaRef string `json:"schemaRef"`
	Payload string `json:"payload"`
	SignalingStreamId string `json:"signalingStreamId"`
}

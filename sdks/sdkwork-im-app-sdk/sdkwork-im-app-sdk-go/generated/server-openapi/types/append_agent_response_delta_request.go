package types


type AppendAgentResponseDeltaRequest struct {
	FrameSeq int `json:"frameSeq"`
	FrameType string `json:"frameType"`
	SchemaRef string `json:"schemaRef"`
	Encoding string `json:"encoding"`
	Payload string `json:"payload"`
	Attributes StringMap `json:"attributes"`
}

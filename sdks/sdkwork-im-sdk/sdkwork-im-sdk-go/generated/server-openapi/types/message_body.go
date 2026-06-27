package types


type MessageBody struct {
	Text string `json:"text"`
	Parts []ContentPart `json:"parts"`
	ReplyTo MessageReplyReference `json:"replyTo"`
	RenderHints map[string]interface{} `json:"renderHints"`
	Summary string `json:"summary"`
	Metadata map[string]interface{} `json:"metadata"`
}

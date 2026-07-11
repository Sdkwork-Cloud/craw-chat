package types


type EditMessageRequest struct {
	Text string `json:"text"`
	Parts []ContentPart `json:"parts"`
	ReplyTo MessageReplyReference `json:"replyTo"`
	Summary string `json:"summary"`
	RenderHints map[string]interface{} `json:"renderHints"`
	IdempotencyKey string `json:"idempotencyKey"`
}

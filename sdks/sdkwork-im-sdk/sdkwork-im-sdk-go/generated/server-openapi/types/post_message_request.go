package types


type PostMessageRequest struct {
	Text string `json:"text"`
	Parts []ContentPart `json:"parts"`
	ReplyTo MessageReplyReference `json:"replyTo"`
	ClientMsgId string `json:"clientMsgId"`
	Summary string `json:"summary"`
	RenderHints map[string]interface{} `json:"renderHints"`
}

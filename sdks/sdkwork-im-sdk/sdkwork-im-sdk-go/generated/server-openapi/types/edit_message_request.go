package types


type EditMessageRequest struct {
	Text string `json:"text"`
	Parts []ContentPart `json:"parts"`
	ReplyTo MessageReplyReference `json:"replyTo"`
}

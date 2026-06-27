package types


type MessageReplyReference struct {
	MessageId string `json:"messageId"`
	SenderDisplayName string `json:"senderDisplayName"`
	ContentPreview string `json:"contentPreview"`
}

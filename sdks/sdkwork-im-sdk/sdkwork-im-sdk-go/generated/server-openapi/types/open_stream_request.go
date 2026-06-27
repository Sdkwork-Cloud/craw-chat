package types


type OpenStreamRequest struct {
	StreamType string `json:"streamType"`
	ConversationId string `json:"conversationId"`
}

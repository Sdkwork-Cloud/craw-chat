package types


type CreateRtcSessionRequest struct {
	RtcSessionId string `json:"rtcSessionId"`
	ConversationId string `json:"conversationId"`
	RtcMode string `json:"rtcMode"`
}

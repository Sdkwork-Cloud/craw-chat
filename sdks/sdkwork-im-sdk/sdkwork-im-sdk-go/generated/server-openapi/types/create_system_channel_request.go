package types


type CreateSystemChannelRequest struct {
	ConversationId string `json:"conversationId"`
	SubscriberId string `json:"subscriberId"`
}

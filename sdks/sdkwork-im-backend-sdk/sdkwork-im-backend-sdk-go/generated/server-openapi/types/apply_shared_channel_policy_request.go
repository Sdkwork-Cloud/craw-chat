package types


type ApplySharedChannelPolicyRequest struct {
	AppliedAt string `json:"appliedAt"`
	ChannelId string `json:"channelId"`
	ConnectionId string `json:"connectionId"`
	ConversationId string `json:"conversationId"`
	EventId string `json:"eventId"`
	HistoryVisibility string `json:"historyVisibility"`
	PolicyId string `json:"policyId"`
	PolicyVersion int `json:"policyVersion"`
}

package types


type RtcSessionMutationResponse struct {
	TenantId string `json:"tenantId"`
	RtcSessionId string `json:"rtcSessionId"`
	ConversationId string `json:"conversationId"`
	InitiatorId string `json:"initiatorId"`
	InitiatorKind string `json:"initiatorKind"`
	ProviderPluginId string `json:"providerPluginId"`
	ProviderSessionId string `json:"providerSessionId"`
	AccessEndpoint string `json:"accessEndpoint"`
	ProviderRegion string `json:"providerRegion"`
	RtcMode string `json:"rtcMode"`
	State string `json:"state"`
	SignalingStreamId string `json:"signalingStreamId"`
	ArtifactMessageId string `json:"artifactMessageId"`
	StartedAt string `json:"startedAt"`
	EndedAt string `json:"endedAt"`
	RequestKey string `json:"requestKey"`
	DeliveryStatus string `json:"deliveryStatus"`
	ProofVersion string `json:"proofVersion"`
}

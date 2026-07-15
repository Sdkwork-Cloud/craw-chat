package types


type GroupKnowledgebaseLaunchResponse struct {
	ConversationId string `json:"conversationId"`
	LifecycleState GroupKnowledgebaseLifecycleState `json:"lifecycleState"`
	SpaceId string `json:"spaceId"`
	SpaceUuid string `json:"spaceUuid"`
	LaunchTicket string `json:"launchTicket"`
	ExpiresAt string `json:"expiresAt"`
	MembershipEpoch string `json:"membershipEpoch"`
	UpstreamLinkGeneration string `json:"upstreamLinkGeneration"`
	ProvisioningOperationId string `json:"provisioningOperationId"`
}

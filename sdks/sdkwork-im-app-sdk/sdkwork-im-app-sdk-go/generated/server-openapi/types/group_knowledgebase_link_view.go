package types


type GroupKnowledgebaseLinkView struct {
	ConversationId string `json:"conversationId"`
	SpaceId string `json:"spaceId"`
	SpaceUuid string `json:"spaceUuid"`
	LifecycleState GroupKnowledgebaseLifecycleState `json:"lifecycleState"`
	ProvisioningOperationId string `json:"provisioningOperationId"`
	MembershipEpoch string `json:"membershipEpoch"`
	UpstreamLinkGeneration string `json:"upstreamLinkGeneration"`
	LastErrorCode string `json:"lastErrorCode"`
}

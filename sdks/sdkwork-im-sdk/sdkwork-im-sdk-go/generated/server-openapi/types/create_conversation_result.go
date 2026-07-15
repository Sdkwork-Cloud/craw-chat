package types


type CreateConversationResult struct {
	ConversationId string `json:"conversationId"`
	EventId string `json:"eventId"`
	RequestKey string `json:"requestKey"`
	DeliveryStatus string `json:"deliveryStatus"`
	ProofVersion string `json:"proofVersion"`
	KnowledgebaseInitialization string `json:"knowledgebaseInitialization"`
}

package types


type ArchiveGroupConversationResponse struct {
	Accepted bool `json:"accepted"`
	ResourceId string `json:"resourceId"`
	Status string `json:"status"`
	ArchiveEventId string `json:"archiveEventId"`
	ArchivedAt string `json:"archivedAt"`
	KnowledgebaseArchiveScheduled bool `json:"knowledgebaseArchiveScheduled"`
}

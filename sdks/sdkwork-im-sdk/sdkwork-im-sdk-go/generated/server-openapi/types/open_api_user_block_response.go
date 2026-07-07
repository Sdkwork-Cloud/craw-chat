package types


type OpenApiUserBlockResponse struct {
	UserBlock UserBlock `json:"userBlock"`
	LatestCommit CommitEnvelopeResponse `json:"latestCommit"`
	Persistence SocialWritePersistence `json:"persistence"`
}

package types


type PortalAuditRecordView struct {
	RecordId string `json:"recordId"`
	Action string `json:"action"`
	ActorId string `json:"actorId"`
	RecordedAt string `json:"recordedAt"`
	Severity string `json:"severity"`
}

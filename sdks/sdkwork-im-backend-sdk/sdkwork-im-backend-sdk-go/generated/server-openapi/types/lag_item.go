package types


type LagItem struct {
	Component string `json:"component"`
	ScopeId string `json:"scopeId"`
	CurrentOffset string `json:"currentOffset"`
	CommittedOffset string `json:"committedOffset"`
	Lag string `json:"lag"`
}

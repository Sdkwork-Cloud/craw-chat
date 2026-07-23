package types


type PortalGovernanceRiskSample struct {
	CriticalCount PortalInt64Count `json:"criticalCount"`
	HighCount PortalInt64Count `json:"highCount"`
	WarningCount PortalInt64Count `json:"warningCount"`
	InformationalCount PortalInt64Count `json:"informationalCount"`
}

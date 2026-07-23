package types


type PortalGovernanceSnapshot struct {
	Meta PortalSnapshotMeta `json:"meta"`
	Availability PortalDataAvailability `json:"availability"`
	SampledEventCount PortalInt64Count `json:"sampledEventCount"`
	RiskSample PortalGovernanceRiskSample `json:"riskSample"`
}

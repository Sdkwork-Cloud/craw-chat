package types


type RolloutPolicyResponse struct {
	CellSelector string `json:"cellSelector"`
	OperatorOverride bool `json:"operatorOverride"`
	PolicyId string `json:"policyId"`
	RegionSelector string `json:"regionSelector"`
	ReleaseChannel string `json:"releaseChannel"`
	TenantAllowlist []string `json:"tenantAllowlist"`
	TrafficPercent int `json:"trafficPercent"`
}

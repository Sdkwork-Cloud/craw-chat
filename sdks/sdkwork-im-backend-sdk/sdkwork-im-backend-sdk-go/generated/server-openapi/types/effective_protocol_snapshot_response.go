package types


type EffectiveProtocolSnapshotResponse struct {
	AllowedBindings []string `json:"allowedBindings"`
	AllowedCodecs []string `json:"allowedCodecs"`
	EnabledCapabilities []string `json:"enabledCapabilities"`
	KillSwitchActive bool `json:"killSwitchActive"`
	Precedence []string `json:"precedence"`
	ProtocolVersion string `json:"protocolVersion"`
	QuotaProfileId string `json:"quotaProfileId"`
	ReleaseChannel string `json:"releaseChannel"`
}

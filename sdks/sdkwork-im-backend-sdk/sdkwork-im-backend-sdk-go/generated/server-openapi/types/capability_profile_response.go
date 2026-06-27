package types


type CapabilityProfileResponse struct {
	EnabledCapabilities []string `json:"enabledCapabilities"`
	ExperimentalCapabilities []string `json:"experimentalCapabilities"`
	ProfileId string `json:"profileId"`
	ReleaseChannel string `json:"releaseChannel"`
}
